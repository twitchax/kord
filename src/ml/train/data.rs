//! Module that defines how data is batched and loaded for training.

use std::path::Path;

use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    tensor::{backend::Backend, Data, Tensor},
};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use crate::{ml::base::{KordItem, INPUT_SPACE_SIZE, NUM_CLASSES, MEL_SPACE_SIZE}, analyze::base::get_notes_from_smoothed_frequency_space, core::note::{Note, HasNoteId}};

use super::base::{load_kord_item, mel_filter_banks_from};

// Dataset.

/// A dataset of kord samples.
pub(crate) struct KordDataset {
    pub items: Vec<KordItem>,
}

impl KordDataset {
    /// Load the kord dataset from the given folder.
    pub fn from_folder(name: impl AsRef<Path>, seed: u64) -> (Self, Self) {
        // First, get all of the *.bin files in the folder.
        let mut files = std::fs::read_dir(name)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().unwrap() == "bin")
            .collect::<Vec<_>>();

        //Convert the seed.
        let mut seed_bytes = [0u8; 32];
        for (i, byte) in seed.to_be_bytes().iter().enumerate() {
            seed_bytes[i] = *byte;
        }

        // Shuffle the files.
        let mut rng = StdRng::from_seed(seed_bytes);
        files.shuffle(&mut rng);

        // Split the files into train and test.
        let (test_files, train_files) = files.split_at(files.len() / 10);

        // Load the files into memory.
        let test_items = test_files.iter().map(load_kord_item).collect();
        let train_items = train_files.iter().map(load_kord_item).collect();

        // Return the train and test datasets.
        let train = Self { items: train_items };
        let test = Self { items: test_items };

        (train, test)
    }
}

impl Dataset<KordItem> for KordDataset {
    fn get(&self, index: usize) -> Option<KordItem> {
        self.items.get(index).cloned()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

// Batcher.

pub(crate) struct KordBatcher<B: Backend> {
    device: B::Device,
}

impl<B: Backend> KordBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct KordBatch<B: Backend> {
    pub(crate) samples: Tensor<B, 2>,
    pub(crate) targets: Tensor<B, 2>,
}

impl<B: Backend> Batcher<KordItem, KordBatch<B>> for KordBatcher<B> {
    fn batch(&self, items: Vec<KordItem>) -> KordBatch<B> {
        let frequency_spaces = items
            .iter()
            .map(kord_item_to_sample_tensor)
            .collect();

        let targets = items
            .iter()
            .map(kord_item_to_target_tensor)
            .collect();

        let frequency_spaces = Tensor::cat(frequency_spaces, 0).to_device(&self.device).detach();
        let targets = Tensor::cat(targets, 0).to_device(&self.device).detach();

        KordBatch { samples: frequency_spaces, targets }
    }
}

/// Takes a loaded kord item and converts it to a sample tensor that is ready for classification.
pub(crate) fn kord_item_to_sample_tensor<B: Backend>(item: &KordItem) -> Tensor<B, 2> {
    let frequency_space = item.frequency_space;
    let mel_space = mel_filter_banks_from(&frequency_space);

    // Get the max value.
    let max = mel_space.iter().fold(0f32, |acc, &x| acc.max(x));

    // Normalize the mel space peaks.
    let mut normalized_mel_space = [0f32; MEL_SPACE_SIZE];
    (0..MEL_SPACE_SIZE).for_each(|k| {
        normalized_mel_space[k] = mel_space[k] / max;
    });

    // Get the "deterministic guess".
    let deterministic_guess = u128_to_binary(get_deterministic_guess(item));

    let result: [f32; INPUT_SPACE_SIZE] = [
        &deterministic_guess[..],
        &normalized_mel_space[..],
    ].concat().try_into().unwrap();

    let data = Data::<f32, 1>::from(result);
    let tensor = Tensor::<B, 1>::from_data(data.convert());

    tensor.reshape([1, INPUT_SPACE_SIZE])
}

/// Takes a loaded kord item and converts it to a target tensor that is ready for classification.
pub(crate) fn kord_item_to_target_tensor<B: Backend>(item: &KordItem) -> Tensor<B, 2> {
    let binary = u128_to_binary(item.label);

    // let mut folded = [0f32; 12];

    // for k in 0..10 {
    //     let slice = &binary[k * 12..(k + 1) * 12];

    //     for i in 0..12 {
    //         folded[i] = (slice[i] as f32).max(folded[i]) as f32;
    //     }
    // }

    let data = Data::<f32, 1>::from(binary);
    let tensor = Tensor::<B, 1>::from_data(data.convert());

    tensor.reshape([1, NUM_CLASSES])
}

/// Gets the "deterministic guess" for a given kord item.
pub(crate) fn get_deterministic_guess(kord_item: &KordItem) -> u128 {
    let smoothed_frequency_space = kord_item.frequency_space.into_iter().enumerate().map(|(k, v)| (k as f32, v)).collect::<Vec<_>>();

    let notes = get_notes_from_smoothed_frequency_space(&smoothed_frequency_space);

    Note::id_mask(&notes)
}

/// Produces a 128 element array of 0s and 1s from a u128.
pub(crate) fn u128_to_binary(num: u128) -> [f32; 128] {
    let mut binary = [0f32; 128];
    for i in 0..128 {
        binary[127 - i] = (num >> i & 1) as f32;
    }

    binary
}

/// Produces a u128 from a 128 element array of 0s and 1s.
pub(crate) fn binary_to_u128(binary: &[f32]) -> u128 {
    let mut num = 0u128;
    for i in 0..128 {
        num += (binary[i] as u128) << (127 - i);
    }

    num
}