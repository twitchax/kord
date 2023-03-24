//! Module that defines how data is batched and loaded for training.

use std::path::Path;

use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    tensor::{backend::Backend, Tensor},
};
use rayon::prelude::*;

use crate::ml::base::{
    data::{kord_item_to_sample_tensor, kord_item_to_target_tensor},
    helpers::load_kord_item,
    KordItem,
};

use super::helpers::get_simulated_kord_items;

// Dataset.

/// A dataset of kord samples.
pub struct KordDataset {
    pub items: Vec<KordItem>,
}

impl KordDataset {
    /// Load the kord dataset from the given folder.
    pub fn from_folder_and_simulation(name: impl AsRef<Path>, count: usize, peak_radius: f32, harmonic_decay: f32, frequency_wobble: f32) -> (Self, Self) {
        // First, get all of the *.bin files in the folder.
        let test_files = std::fs::read_dir(name)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file())
            .filter(|path| path.extension().unwrap() == "bin")
            .collect::<Vec<_>>();

        let test_items: Vec<_> = test_files.par_iter().map(load_kord_item).collect();
        let train_items = get_simulated_kord_items(count, peak_radius, harmonic_decay, frequency_wobble);

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

pub struct KordBatcher<B: Backend> {
    device: B::Device,
}

impl<B: Backend> KordBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

#[derive(Clone, Debug)]
pub struct KordBatch<B: Backend> {
    pub samples: Tensor<B, 2>,
    pub targets: Tensor<B, 2>,
}

impl<B: Backend> Batcher<KordItem, KordBatch<B>> for KordBatcher<B> {
    fn batch(&self, items: Vec<KordItem>) -> KordBatch<B> {
        let samples = items.iter().map(kord_item_to_sample_tensor).collect();

        let targets = items.iter().map(kord_item_to_target_tensor).collect();

        let frequency_spaces = Tensor::cat(samples, 0).to_device(&self.device).detach();
        let targets = Tensor::cat(targets, 0).to_device(&self.device).detach();

        KordBatch { samples: frequency_spaces, targets }
    }
}
