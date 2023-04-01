//! Base audio analysis functions.
//!
//! Performs ffts, frequency space smoothing, peak detection, harmonic collapsing, and note detection.

use std::{collections::HashMap, ops::Deref};

use rustfft::{
    num_complex::{Complex, ComplexFloat},
    FftPlanner,
};

use crate::core::note::{HasPrimaryHarmonicSeries, ALL_PITCH_NOTES_WITH_FREQUENCY};

use crate::core::{base::Res, note::Note, pitch::HasFrequency};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

//#[cfg_attr(wasm, wasm_bindgen)]
pub fn get_notes_from_audio_data(data: &[f32], length_in_seconds: u8) -> Res<Vec<Note>> {
    if length_in_seconds < 1 {
        return Err(anyhow::Error::msg("Listening length in seconds must be greater than 1."));
    }

    let num_nan = data.iter().filter(|n| n.is_nan()).count();
    if num_nan > 0 {
        return Err(anyhow::Error::msg(format!("{} NaNs in audio data.", num_nan)));
    }

    let frequency_space = get_frequency_space(data, length_in_seconds);

    // Smooth the frequency space.

    let smoothed_frequency_space = get_smoothed_frequency_space(&frequency_space, length_in_seconds);
    //plot_frequency_space(&smoothed_frequency_space, "frequency_space", 100f32, 1000f32);

    Ok(get_notes_from_smoothed_frequency_space(&smoothed_frequency_space))
}

pub fn get_notes_from_smoothed_frequency_space(smoothed_frequency_space: &[(f32, f32)]) -> Vec<Note> {
    // Translate the frequency space into a "peak space" (dampen values that are not the "peak" of a specified window).

    let peak_space = translate_frequency_space_to_peak_space(smoothed_frequency_space);
    //plot_frequency_space(&peak_space, "peak_space", 100f32, 1000f32);

    // Bucket top N bins into their proper notes, and keep "magnitude".

    let peak_best_notes = get_likely_notes_from_peak_space(&peak_space, 0.1);
    //.into_iter().map(|(n, _)| n).collect::<Vec<_>>();
    let best_notes = peak_best_notes;
    //let binned_best_notes = get_likely_notes_using_bins(smoothed_frequency_space, 0.5, 0.1);

    //let best_notes = binned_best_notes.into_iter().filter(|(n, _)| peak_best_notes.contains(n)).collect::<Vec<_>>();

    // Fold the harmonic series into the core notes.

    reduce_notes_by_harmonic_series(&best_notes, 0.1)
}

/// Gets the frequency space from the audio data.
pub fn get_frequency_space(data: &[f32], length_in_seconds: u8) -> Vec<(f32, f32)> {
    let num_samples = data.len();

    // Perform the FFT.

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(num_samples);

    let mut buffer = data.iter().map(|n| Complex::new(*n, 0.0)).collect::<Vec<_>>();
    fft.process(&mut buffer);

    buffer.into_iter().enumerate().map(|(k, d)| (k as f32 / length_in_seconds as f32, d.abs())).collect::<Vec<_>>()
}

/// Gets the time space from the frequency space.
pub fn get_time_space(data: &[f32]) -> Vec<(f32, f32)> {
    let num_samples = data.len();

    // Perform the FFT.

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(num_samples);

    let mut buffer = data.iter().map(|n| Complex::new(*n, 0.0)).collect::<Vec<_>>();
    fft.process(&mut buffer);

    buffer.into_iter().enumerate().map(|(k, d)| (k as f32, d.abs())).collect::<Vec<_>>()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn compute_cqt(frequency_space: &[f32]) -> Vec<f32> {
    const Q_FACTOR: f32 = 24.7; // Q-factor for the CQT
    const MIN_FREQ: f32 = 65.41; // minimum frequency for the CQT
    const MAX_FREQ: f32 = 2093.0; // maximum frequency for the CQT
    const N_BINS: usize = 60; // number of frequency bins for the CQT

    let mut cqt_output = vec![vec![0.0; frequency_space.len()]; N_BINS];

    let log_min_freq = MIN_FREQ.log2();
    let log_max_freq = MAX_FREQ.log2();
    let log_freq_step = (log_max_freq - log_min_freq) / (N_BINS as f32 - 1.0);

    for i in 0..N_BINS {
        let log_freq_center = log_min_freq + i as f32 * log_freq_step;
        let freq_center = 2.0f32.powf(log_freq_center);
        let freq_bw = freq_center / Q_FACTOR;
        let fft_freq_step = 1.0;

        let start_bin = (freq_center - freq_bw / 2.0) / fft_freq_step;
        let end_bin = (freq_center + freq_bw / 2.0) / fft_freq_step;

        let mut cqt_bin = vec![rustfft::num_complex::Complex::new(0.0, 0.0); frequency_space.len()];

        for j in start_bin as usize..=end_bin as usize {
            let weight = (j as f32 - freq_center / fft_freq_step) / freq_bw;
            let weight = weight * std::f32::consts::PI * 2.0;
            let fft_bin = frequency_space[j];
            cqt_bin[j] = rustfft::num_complex::Complex::new(fft_bin * weight.sin(), 0.0);
        }

        let ifft = rustfft::FftPlanner::<f32>::new().plan_fft_inverse(cqt_bin.len());
        ifft.process(&mut cqt_bin);

        for j in 0..frequency_space.len() {
            cqt_output[i][j] = cqt_bin[j].abs();
        }
    }

    let mut result = vec![];
    for k in 0..N_BINS {
        let mut sum = 0.0;
        for j in 0..frequency_space.len() {
            sum += cqt_output[k][j];
        }
        result.push(sum);
    }

    result
}

/// Calculates the "smoothed" frequency space by normalizing to 1.0 seconds of playback.
pub fn get_smoothed_frequency_space(frequency_space: &[(f32, f32)], length_in_seconds: u8) -> Vec<(f32, f32)> {
    let mut smoothed_frequency_space = Vec::new();
    let size = length_in_seconds as usize;

    for k in (0..frequency_space.len()).step_by(size) {
        let average_frequency = frequency_space[k..k + size].iter().map(|(f, _)| f).sum::<f32>() / size as f32;
        let average_magnitude = frequency_space[k..k + size].iter().map(|(_, m)| m).sum::<f32>() / size as f32;

        smoothed_frequency_space.push((average_frequency, average_magnitude));
    }

    smoothed_frequency_space
}

/// Translate the frequency space into a "peak space".
///
/// Returns a vector of (frequency, magnitude) pair peaks sorted from largest magnitude to smallest.
pub fn translate_frequency_space_to_peak_space(frequency_space: &[(f32, f32)]) -> Vec<(f32, f32)> {
    // Dividing the frequency by 32.5 yields roughly 1/3 the distance between a note and the note one semitone away, which is the window size we want
    let magic_window_number = 50f32;

    // Compute proper start and end indexes.  // Only need to find peaks within the limits of a piano / singing.
    let min_index = 50;
    let max_index = 8_000;

    let mut peak_space = frequency_space.to_vec();

    // Find maximum peaks in the window.

    let mut last_k = min_index;
    let mut k = min_index;
    while k < max_index {
        let window_size = (frequency_space[k].0 / magic_window_number) as usize;

        let max_in_window = (k..k + window_size).map(|i| frequency_space[i].1).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();

        peak_space[k] = (peak_space[k].0, peak_space[k].1);

        let mut next = 0;
        for j in k..(k + window_size) {
            if frequency_space[j].1 == max_in_window {
                peak_space[j] = (peak_space[j].0, peak_space[j].1);
                next = j;
            } else {
                peak_space[j] = (peak_space[j].0, 0.0);
            }
        }

        k = next;

        if last_k == k {
            k += 1;
        }

        last_k = k;
    }

    // Zero out the peaks with a low relative derivative (they are "smooth", and therefore, more likely to be noise).

    let skip = min_index;
    let take = max_index - min_index;

    for (k, (_, magnitude)) in peak_space.iter_mut().enumerate().skip(skip).take(take) {
        let window_size = 3;

        // Compute the average derivative.
        let average_right_derivative = ((frequency_space[k + window_size].1 - frequency_space[k].1) / window_size as f32).abs();
        let average_left_derivative = ((frequency_space[k].1 - frequency_space[k - window_size].1) / window_size as f32).abs();
        let average_derivative = (average_right_derivative + average_left_derivative) / 2f32;

        // Zero out the peaks with a low relative derivative.

        if average_derivative / *magnitude < 0.1 {
            *magnitude = 0.0;
        }
    }

    peak_space.into_iter().skip(min_index).take(max_index - min_index).collect()
}

/// Get likely notes from the peak space.
fn get_likely_notes_from_peak_space(peak_space: &[(f32, f32)], cutoff: f32) -> Vec<(Note, f32)> {
    let mut peak_space = peak_space.iter().filter(|(_, m)| *m > 0.1).copied().collect::<Vec<_>>();
    peak_space.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let max_power = peak_space[0].1;

    // Take all peaks with 10% or more of the max power.
    let peak_space = peak_space.into_iter().filter(|(_, m)| *m > max_power * cutoff).collect::<Vec<_>>();

    let mut candidates = HashMap::new();

    for (frequency, magnitude) in peak_space.iter() {
        if let Some(pair) = binary_search_closest(ALL_PITCH_NOTES_WITH_FREQUENCY.deref(), *frequency, |t| t.1) {
            let note = pair.0;
            let entry = candidates.entry(note).or_insert(*magnitude);
            *entry += magnitude;
        }
    }

    candidates.into_iter().collect::<Vec<_>>()
}

/// Reduce a vector of notes by removing all notes that are part of the harmonic series of another note.
fn reduce_notes_by_harmonic_series(notes: &[(Note, f32)], cutoff: f32) -> Vec<Note> {
    let mut working_set = notes.to_vec();
    working_set.sort_unstable_by(|a, b| a.0.frequency().partial_cmp(&b.0.frequency()).unwrap());

    // First, remove harmonic series notes.

    let mut k = 0;
    while k < working_set.len() {
        let note = working_set[k].0;

        let mut j = k + 1;
        while j < working_set.len() {
            let other_note = working_set[j].0;

            for harmonic in note.primary_harmonic_series() {
                if harmonic.frequency() == other_note.frequency() {
                    working_set[k].1 += working_set[j].1;
                    working_set.remove(j);
                    j -= 1;
                }
            }

            j += 1;
        }

        k += 1;
    }

    // Reorder the rest by magnitude, and return the notes.

    working_set.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Remove notes that are below the threshold.

    let cutoff = working_set[0].1 * cutoff;
    working_set.retain(|(_, magnitude)| *magnitude > cutoff);

    working_set.into_iter().map(|(note, _)| note).collect()
}

/// For every note, get its "frequency window", which is halfway between the frequency of the note and the frequency of the
/// the one before, and the next one.
///
/// Returns a vector of tuples, where the first element is the note, and the second element is the frequency window as a (low, high) tuple.
/// The first and the last note supplied are ignored, so this method returns `notes.len() - 2` elements.
pub fn get_frequency_bins(notes: &[Note]) -> Vec<(Note, (f32, f32))> {
    let mut bins = Vec::new();

    for (i, note) in notes.iter().enumerate() {
        let low = if i == 0 {
            continue;
        } else {
            note.frequency() - 0.50 * (note.frequency() - notes[i - 1].frequency())
        };

        let high = if i == notes.len() - 1 {
            continue;
        } else {
            note.frequency() + 0.50 * (notes[i + 1].frequency() - note.frequency())
        };

        bins.push((*note, (low, high)));
    }

    bins
}

/// Perform a binary search of an array to find the the element that is closest to the target as defined by a closure.
///
/// The array must be sorted in ascending order.
pub fn binary_search_closest<T, F>(array: &[T], target: f32, mut get_value: F) -> Option<&T>
where
    F: FnMut(&T) -> f32,
{
    // Perform the binary search.

    let mut low = 0;
    let mut high = array.len();

    while low < high {
        let mid = (low + high) / 2;

        let value = get_value(&array[mid]);

        if value < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    if low == 0 || low == array.len() {
        return None;
    }

    // Find the closest element between the last two.

    let low_index = low - 1;
    let high_index = low;
    let low_value = get_value(&array[low_index]);
    let high_value = get_value(&array[high_index]);

    if (high_value - target).abs() < (target - low_value).abs() {
        Some(&array[high_index])
    } else {
        Some(&array[low_index])
    }
}

// Tests.

#[cfg(test)]
pub(crate) mod tests {
    use std::{fs::File, io::Read};

    use crate::core::note::ALL_PITCH_NOTES;

    use super::*;

    pub fn load_test_data() -> Vec<f32> {
        let mut file = File::open("tests/vec.bin").unwrap();
        let file_size = file.metadata().unwrap().len() as usize;
        let float_size = std::mem::size_of::<f32>();
        let element_count = file_size / float_size;
        let mut buffer = vec![0u8; file_size];

        // Read the contents of the file into the buffer
        file.read_exact(&mut buffer).unwrap();

        // Convert the buffer to a vector of f32
        let data: Vec<f32> = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const f32, element_count).to_vec() };

        data
    }

    #[test]
    #[should_panic]
    fn test_get_notes_from_audio_data_length() {
        get_notes_from_audio_data(&[0.0, 0.0, 0.0], 0).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_get_notes_from_audio_data_nan() {
        get_notes_from_audio_data(&[0.0, 0.0, f32::NAN], 10).unwrap();
    }

    #[test]
    fn test_get_time_space() {
        let data = load_test_data();

        let frequency_space = get_frequency_space(&data, 5).into_iter().map(|(_, v)| v).collect::<Vec<_>>();
        let _ = get_time_space(&frequency_space);
    }

    #[test]
    fn test_get_frequency_bins() {
        let bins = get_frequency_bins(&ALL_PITCH_NOTES.iter().skip(24).take(62).cloned().collect::<Vec<_>>());

        assert_eq!(bins.len(), 60);
    }

    #[test]
    #[should_panic]
    fn test_binary_search_closest_empty() {
        binary_search_closest(&[], 0.0, |x| *x).unwrap();
    }
}
