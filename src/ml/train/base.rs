//! Base types for the machine learning train module.

use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::{BufReader, Cursor, Write},
    path::Path,
};

use burn::config::Config;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    core::base::Void,
    ml::base::{KordItem, FREQUENCY_SPACE_SIZE, MEL_SPACE_SIZE},
};

// Training configuration.

#[derive(Debug, Config)]
pub struct TrainConfig {
    /// The source directory for the gathered samples.
    pub source: String,
    /// The destination directory for the trained model.
    pub destination: String,

    /// The number of Multi Layer Perceptron (MLP) layers.
    pub mlp_layers: usize,
    /// The number of neurons in each Multi Layer Perceptron (MLP) layer.
    pub mlp_size: usize,
    /// The Multi Layer Perceptron (MLP) dropout rate.
    pub mlp_dropout: f64,
    
    /// The number of epochs to train for.
    pub model_epochs: usize,
    /// The number of samples to use per epoch.
    pub model_batch_size: usize,
    /// The number of workers to use for training.
    pub model_workers: usize,
    /// The seed used for training.
    pub model_seed: u64,

    /// The Adam optimizer learning rate.
    pub adam_learning_rate: f64,
    /// The Adam optimizer weight decay.
    pub adam_weight_decay: f64,
    /// The Adam optimizer beta1.
    pub adam_beta1: f32,
    /// The Adam optimizer beta2.
    pub adam_beta2: f32,
    /// The Adam optimizer epsilon.`
    pub adam_epsilon: f32,

    /// The "sigmoid strength" of the final pass.
    pub sigmoid_strength: f32,
}

// Operations for working with kord samples.

/// Load the kord sample from the binary file into a new [`KordItem`].
pub(crate) fn load_kord_item(path: impl AsRef<Path>) -> KordItem {
    let file = std::fs::File::open(path.as_ref()).unwrap();
    let mut reader = BufReader::new(file);

    // Read 8192 f32s in big endian from the file.
    let mut frequency_space = [0f32; 8192];

    (0usize..FREQUENCY_SPACE_SIZE).for_each(|k| {
        frequency_space[k] = reader.read_f32::<BigEndian>().unwrap();
    });

    let label = reader.read_u128::<BigEndian>().unwrap();

    KordItem { path: path.as_ref().to_owned(), frequency_space, label }
}

/// Save the kord sample into a binary file.
pub(crate) fn save_kord_item(destination: impl AsRef<Path>, note_names: &str, item: &KordItem) -> Void {
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
    let mut f = File::create(destination.as_ref().join(format!("{}_{}.bin", note_names, hash)))?;
    f.write_all(&output_data)?;

    Ok(())
}

// Operations for working with mels.

/// Convert the [`FREQUENCY_SPACE_SIZE`] f32s in frequency space into [`MEL_SPACE_SIZE`] mel filter bands.
pub(crate) fn mel_filter_banks_from(spectrum: &[f32]) -> [f32; MEL_SPACE_SIZE] {
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

/// Create a linearly spaced vector.
pub(crate) fn linspace(start: f32, end: f32, num_points: usize) -> Vec<f32> {
    let step = (end - start) / (num_points - 1) as f32;
    (0..num_points).map(|i| start + i as f32 * step).collect()
}

/// Converts a frequency to a mel.
pub(crate) fn mel(f: f32) -> f32 {
    2595f32 * (1f32 + f / 700f32).log10()
}

/// Converts a mel to a frequency.
pub(crate) fn inv_mel(m: f32) -> f32 {
    700f32 * (10f32.powf(m / 2595f32) - 1f32)
}

// Tests.

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use pretty_assertions::assert_eq;

//     #[test]
//     fn test_kord_item() {
//         let destination = Path::new("test_data");
//         let length_in_seconds = 1;

//         gather_sample(destination, length_in_seconds).unwrap();
//     }
// }