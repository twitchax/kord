use std::{sync::{Arc, Mutex}, time::Duration};

use anyhow::Context;
use cpal::{traits::{HostTrait, StreamTrait}, InputCallbackInfo};
use rodio::DeviceTrait;

use std::{collections::HashMap, ops::Deref};

use rustfft::{FftPlanner, num_complex::{Complex, ComplexFloat}};

use crate::{note::{ALL_PITCH_NOTES_WITH_FREQUENCY}};

use crate::{base::Res, note::Note, interval::PRIMARY_HARMONIC_SERIES, pitch::HasFrequency};

pub async fn get_notes_from_microphone(length_in_seconds: u8) -> Res<Vec<Note>> {
    if length_in_seconds < 1 {
        return Err(anyhow::Error::msg("Listening length in seconds must be greater than 1."));
    }

    // Set up devices and systems.
    
    let (device, config) = get_device_and_config()?;

    // Record audio from the microphone.

    let data_from_microphone = record_from_device(device, config, length_in_seconds).await?;
    
    // Get notes.

    let result = get_notes_from_audio_data(&data_from_microphone, length_in_seconds)?;

    Ok(result)
}


pub fn get_notes_from_audio_data(data: &[f32], length_in_seconds: u8) -> Res<Vec<Note>> {
    if length_in_seconds < 1 {
        return Err(anyhow::Error::msg("Listening length in seconds must be greater than 1."));
    }

    let num_samples = data.len();
    
    // Perform the FFT.

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(num_samples);

    let mut buffer = data.into_iter().map(|n| Complex::new(*n, 0.0)).collect::<Vec<_>>();
    fft.process(&mut buffer);

    let frequency_space = buffer.into_iter().enumerate()
        .map(|(k, d)| (k as f32 / length_in_seconds as f32, d.abs()))
        .collect::<Vec<_>>();

    // Smooth the frequency space.

    let smoothed_frequency_space = get_smoothed_frequency_space(&frequency_space, length_in_seconds);
    //plot_frequency_space(&smoothed_frequency_space, "frequency_space", 100f32, 1000f32);
    
    // Translate the frequency space into a "peak space" (dampen values that are not the "peak" of a specified window).

    let peak_space = translate_frequency_space_to_peak_space(&smoothed_frequency_space);
    //plot_frequency_space(&peak_space, "peak_space", 100f32, 1000f32);
    
    // Bucket top N bins into their proper notes, and keep "magnitude".

    let best_notes = get_likely_notes_from_peak_space(&peak_space, 12);

    // Fold the harmonic series into the core notes.
    
    let result = reduce_notes_by_harmonic_series(&best_notes);

    Ok(result)
}

/// Gets the system device, and config.
fn get_device_and_config() -> Res<(cpal::Device, cpal::SupportedStreamConfig)> {
    let host = cpal::default_host();

    let device = host
        .default_input_device().ok_or_else(|| anyhow::Error::msg("Failed to get default input device."))?;

    let config = device
        .default_input_config().context("Could not get default input config.")?;

    Ok((device, config))
}

async fn record_from_device(device: cpal::Device, config: cpal::SupportedStreamConfig, length_in_seconds: u8) -> Res<Vec<f32>> {
    // Set up recording.

    let likely_sample_count = config.sample_rate().0 as f32 * config.channels() as f32 * length_in_seconds as f32;

    let data_from_microphone = Arc::new(Mutex::new(Vec::with_capacity(likely_sample_count as usize)));
    let last_error = Arc::new(Mutex::new(None));

    let stream = {
        let result = data_from_microphone.clone();
        let last_error = last_error.clone();

        device.build_input_stream::<f32, _, _>(&config.into(), move |data: &[_], _: &InputCallbackInfo| {
            result.lock().unwrap().extend_from_slice(data);
        }, move |err| {
            last_error.lock().unwrap().replace(err);
        })?
    };

    // Begin recording.

    stream.play()?;
    futures_timer::Delay::new(Duration::from_secs_f32(length_in_seconds as f32)).await;
    drop(stream);

    // SAFETY: We are the only thread that can access the arc right now since the stream is dropped.
    if let Err(err) = Arc::try_unwrap(last_error).unwrap().into_inner() {
        return Err(err.into());
    }

    // SAFETY: We are the only thread that can access the arc right now since the stream is dropped.
    let data_from_microphone = Arc::try_unwrap(data_from_microphone).unwrap().into_inner()?;

    Ok(data_from_microphone)
}

/// Get likely notes from the peak space.
fn get_likely_notes_from_peak_space(peak_space: &[(f32, f32)], count: usize) -> Vec<(Note, f32)> {
    let mut peak_space = peak_space.iter().filter(|(_,m)|*m > 0.1).copied().collect::<Vec<_>>();
    peak_space.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut candidates = HashMap::new();

    for (frequency, magnitude) in peak_space.iter().take(count) {
        if let Some(pair) = binary_search_closest(ALL_PITCH_NOTES_WITH_FREQUENCY.deref(),  *frequency, |t| t.1) {
            let note = pair.0;
            let entry = candidates.entry(note).or_insert(*magnitude);
            *entry += magnitude;
        }
    }

    candidates.into_iter().collect::<Vec<_>>()
}

/// Calculates the "smoothed" frequency space by normalizing to 1.0 seconds of playback.
fn get_smoothed_frequency_space(frequency_space: &[(f32, f32)], length_in_seconds: u8) -> Vec<(f32, f32)> {
    let mut smoothed_frequency_space = Vec::new();
    let size = length_in_seconds as usize;

    for k in (0..frequency_space.len()).step_by(size) {
        let average_frequency = frequency_space[k..k+size].iter().map(|(f, _)| f).sum::<f32>() / size as f32;
        let average_magnitude= frequency_space[k..k+size].iter().map(|(_, m)| m).sum::<f32>() / size as f32;
        
        smoothed_frequency_space.push((average_frequency, average_magnitude));
    }

    smoothed_frequency_space
}

/// Reduce a vector of notes by removing all notes that are part of the harmonic series of another note.
fn reduce_notes_by_harmonic_series(notes: &[(Note, f32)]) -> Vec<Note> {
    let mut working_set = notes.to_vec();
    working_set.sort_by(|a, b| a.0.frequency().partial_cmp(&b.0.frequency()).unwrap());

    // First, remove harmonic series notes.

    let mut k = 0;
    while k < working_set.len() {
        let note = working_set[k].0;

        let mut j = k + 1;
        while j < working_set.len() {
            let other_note = working_set[j].0;

            for interval in PRIMARY_HARMONIC_SERIES.iter() {
                if (note + *interval).frequency() == other_note.frequency() {
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

    let cutoff = working_set[0].1 / 10f32;
    working_set.retain(|(_, magnitude)| *magnitude > cutoff);

    working_set.into_iter().map(|(note, _)| note).collect()
}

/// Translate the frequency space into a "peak space".
/// 
/// Returns a vector of (frequency, magnitude) pair peaks sorted from largest magnitude to smallest.
fn translate_frequency_space_to_peak_space(frequency_space: &[(f32, f32)]) -> Vec<(f32, f32)> {
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

        let max_in_window = (k..k + window_size)
            .map(|i| frequency_space[i].1)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or_default();

        peak_space[k] = (peak_space[k].0, peak_space[k].1);

        let mut next =  0;
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

/// Perform a binary search of an array to find the the element that is closest to the target as defined by a closure.
/// 
/// The array must be sorted in ascending order.
fn binary_search_closest<T, F>(array: &[T], target: f32, mut get_value: F) -> Option<&T>
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

/// Plot the frequency space of the microphone input using plotters.
#[cfg(feature = "plot")]
fn plot_frequency_space(frequency_space: &[(f32, f32)], name: &'static str, x_min: f32, x_max: f32) {
    use plotters::prelude::*;

    let max = frequency_space.iter().map(|(_, d)| d).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let normalized_frequency_space = frequency_space.iter().map(|(f, m)| {
        (f, m / max)
    }).collect::<Vec<_>>();

    let file_name = format!("{}.png", name);
    let root = BitMapBackend::new(&file_name, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Frequency Space", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_min..x_max, 0f32..1f32).unwrap();

    chart.configure_mesh().draw().unwrap();

    chart.draw_series(LineSeries::new(normalized_frequency_space.iter().map(|(x, y)| (**x, *y)), RED)).unwrap();
}