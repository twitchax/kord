//! Module for generic training and inference helpers.

use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::{BufReader, Cursor, Write},
    path::{Path, PathBuf},
};

use burn::{
    module::Module,
    tensor::{backend::Backend, Tensor},
};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    analyze::base::get_notes_from_smoothed_frequency_space,
    core::{
        base::Res,
        helpers::{inv_mel, mel},
        note::{HasNoteId, Note, ALL_PITCH_NOTES_WITH_FREQUENCY},
        pitch::HasFrequency,
    },
};

use super::{KordItem, FREQUENCY_SPACE_SIZE, MEL_SPACE_SIZE, NUM_CLASSES};

// Operations for working with kord samples.

/// Load the kord sample from the binary file into a new [`KordItem`].
pub fn load_kord_item(path: impl AsRef<Path>) -> KordItem {
    let file = std::fs::File::open(path.as_ref()).unwrap();
    let mut reader = BufReader::new(file);

    // Read 8192 f32s in big endian from the file.
    let mut frequency_space = [0f32; 8192];

    (0usize..FREQUENCY_SPACE_SIZE).for_each(|k| {
        frequency_space[k] = reader.read_f32::<BigEndian>().unwrap();
    });

    let label = reader.read_u128::<BigEndian>().unwrap();

    KordItem {
        path: path.as_ref().to_owned(),
        frequency_space,
        label,
    }
}

/// Save the kord sample into a binary file.
pub fn save_kord_item(destination: impl AsRef<Path>, prefix: &str, note_names: &str, item: &KordItem) -> Res<PathBuf> {
    let mut output_data: Vec<u8> = Vec::with_capacity(FREQUENCY_SPACE_SIZE);
    let mut cursor = Cursor::new(&mut output_data);

    // Write frequency space.
    for value in item.frequency_space {
        cursor.write_f32::<BigEndian>(value)?;
    }

    // Write result.
    cursor.write_u128::<BigEndian>(item.label)?;

    // Get the hash.
    let mut hasher = DefaultHasher::new();
    output_data.hash(&mut hasher);
    let hash = hasher.finish();

    // Write the file.
    let path = destination.as_ref().join(format!("{prefix}{note_names}_{hash}.bin"));
    let mut f = File::create(&path)?;
    f.write_all(&output_data)?;

    Ok(path)
}

// Operations for working with mels.

/// Convert the [`FREQUENCY_SPACE_SIZE`] f32s in frequency space into [`MEL_SPACE_SIZE`] mel filter bands.
pub fn mel_filter_banks_from(spectrum: &[f32]) -> [f32; MEL_SPACE_SIZE] {
    let num_frequencies = spectrum.len();
    let num_mels = MEL_SPACE_SIZE;

    let f_min = 0f32;
    let f_max = FREQUENCY_SPACE_SIZE as f32;

    let mel_points = linspace(mel(f_min), mel(f_max), num_mels + 2);
    let f_points = mel_points.iter().map(|m| inv_mel(*m)).collect::<Vec<_>>();

    let mut filter_banks = [0f32; MEL_SPACE_SIZE];

    for i in 0..num_mels {
        let f_m_minus = f_points[i];
        let f_m = f_points[i + 1];
        let f_m_plus = f_points[i + 2];

        let k_minus = (num_frequencies as f32 * f_m_minus / 8192f32).floor() as usize;
        let k = (num_frequencies as f32 * f_m / 8192f32).floor() as usize;
        let k_plus = (num_frequencies as f32 * f_m_plus / 8192f32).floor() as usize;

        for j in k_minus..k {
            filter_banks[i] += spectrum[j] * (j - k_minus) as f32 / (k - k_minus) as f32;
        }

        for j in k..k_plus {
            filter_banks[i] += spectrum[j] * (k_plus - j) as f32 / (k_plus - k) as f32;
        }
    }

    filter_banks
}

/// Run a note-binned "harmonic convolution" over the frequency space data.
pub fn note_binned_convolution(spectrum: &[f32]) -> [f32; NUM_CLASSES] {
    let mut convolution = [0f32; NUM_CLASSES];

    for (note, _) in ALL_PITCH_NOTES_WITH_FREQUENCY.iter().skip(7).take(90) {
        let id_index = note.id_index();

        let (low, high) = note.tight_frequency_range();
        let low = low.round() as usize;
        let high = high.round() as usize;

        if high >= FREQUENCY_SPACE_SIZE {
            continue;
        }

        let mut sum = 0f32;
        for k in low..high {
            sum += spectrum[k];
        }

        convolution[id_index as usize] = sum;
    }

    convolution
}

/// Run a "harmonic convolution" over the frequency space data.
pub fn harmonic_convolution(spectrum: &[f32]) -> [f32; FREQUENCY_SPACE_SIZE] {
    let mut harmonic_convolution = [0f32; FREQUENCY_SPACE_SIZE];

    let (peak, _) = spectrum.iter().enumerate().fold((0usize, 0f32), |(k, max), (j, x)| if *x > max { (j, *x) } else { (k, max) });

    for center in (peak / 2)..4000 {
        let mut sum = spectrum[center];

        for k in 2..16 {
            let index = center * k;
            if index < FREQUENCY_SPACE_SIZE {
                sum += spectrum[index];
            }
        }

        for k in 2..16 {
            let index = center / k;
            if index < FREQUENCY_SPACE_SIZE {
                sum -= spectrum[index];
            }
        }

        harmonic_convolution[center] = sum.clamp(0.0, f32::MAX);
    }

    harmonic_convolution
}

/// Create a linearly spaced vector.
pub fn linspace(start: f32, end: f32, num_points: usize) -> Vec<f32> {
    let step = (end - start) / (num_points - 1) as f32;
    (0..num_points).map(|i| start + i as f32 * step).collect()
}

/// Gets the "deterministic guess" for a given kord item.
pub fn get_deterministic_guess(kord_item: &KordItem) -> u128 {
    let smoothed_frequency_space = kord_item.frequency_space.into_iter().enumerate().map(|(k, v)| (k as f32, v)).collect::<Vec<_>>();

    let notes = get_notes_from_smoothed_frequency_space(&smoothed_frequency_space);

    Note::id_mask(&notes)
}

/// Produces a 128 element array of 0s and 1s from a u128.
pub fn u128_to_binary(num: u128) -> [f32; 128] {
    let mut binary = [0f32; 128];
    for i in 0..128 {
        binary[127 - i] = (num >> i & 1) as f32;
    }

    binary
}

/// Produces a u128 from a 128 element array of 0s and 1s.
pub fn binary_to_u128(binary: &[f32]) -> u128 {
    let mut num = 0u128;
    for i in 0..128 {
        num += (binary[i] as u128) << (127 - i);
    }

    num
}

/// Folds the 128-bit binary signature of the the notes into a 12-bit signature (which represent one octave)
#[allow(dead_code)]
pub fn fold_binary(binary: &[f32; 128]) -> [f32; 12] {
    let mut folded = [0f32; 12];

    for k in 0..10 {
        let slice = &binary[k * 12..(k + 1) * 12];

        for i in 0..12 {
            folded[i] = slice[i].max(folded[i]);
        }
    }

    folded
}

/// Applies sigmoid activation and 0.5 threshold to convert logits to binary predictions.
pub fn logits_to_binary_predictions(logits: &[f32]) -> Vec<f32> {
    logits
        .iter()
        .map(|&logit| {
            let prob = 1.0 / (1.0 + (-logit).exp()); // sigmoid
            if prob > 0.5 { 1.0 } else { 0.0 }
        })
        .collect()
}

// Common tensor operations.

/// Module which represents a Sigmoid operation of variable strength.
#[derive(Module, Debug)]
pub struct Sigmoid<B: Backend> {
    scale: Tensor<B, 1>,
}

impl<B: Backend> Sigmoid<B> {
    /// Create a new Sigmoid module with the given scale.
    pub fn new(device: &B::Device, scale: f32) -> Self {
        Self { scale: Tensor::ones([1], device) * scale }
    }

    /// Forward pass of the Sigmoid module.
    pub fn forward<const D: usize>(&self, input: Tensor<B, D>) -> Tensor<B, D> {
        let scaled = input.mul_scalar(self.scale.clone().into_scalar());
        //let scaled = input;
        scaled.clone().exp().div(scaled.exp().add_scalar(1.0))
    }
}
