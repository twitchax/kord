//! Module that defines how data is batched and loaded for training.

use std::path::Path;

use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    tensor::{backend::Backend, Int, Tensor},
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
    /// The items in the dataset.
    pub items: Vec<KordItem>,
}

impl KordDataset {
    /// Load the kord dataset from the given folder.
    pub fn from_folder_and_simulation<R>(noise_asset_root: R, name: impl AsRef<Path>, count: usize, peak_radius: f32, harmonic_decay: f32, frequency_wobble: f32) -> (Self, Self)
    where
        R: AsRef<Path> + Clone + Send + Sync,
    {
        // First, get all of the *.bin files in the folder.
        let test_files = std::fs::read_dir(name)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file())
            .filter(|path| path.extension().unwrap() == "bin")
            .collect::<Vec<_>>();

        let test_items: Vec<_> = test_files.par_iter().map(load_kord_item).collect();
        let train_items = get_simulated_kord_items(noise_asset_root, count, peak_radius, harmonic_decay, frequency_wobble);

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

/// A batcher for kord samples.
#[derive(Clone, Debug)]
pub struct KordBatcher<B: Backend> {
    device: B::Device,
}

impl<B: Backend> KordBatcher<B> {
    /// Create a new kord batcher.
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

/// A batch of kord samples (samples and targets).
#[derive(Clone, Debug)]
pub struct KordBatch<B: Backend> {
    /// The samples in the batch.
    pub samples: Tensor<B, 2>,
    /// The targets in the batch.
    pub targets: Tensor<B, 2, Int>,
}

impl<B: Backend> Batcher<B, KordItem, KordBatch<B>> for KordBatcher<B> {
    fn batch(&self, items: Vec<KordItem>, _device: &B::Device) -> KordBatch<B> {
        let samples = items.iter().map(|i| kord_item_to_sample_tensor(&self.device, i)).collect();

        let targets = items.iter().map(|i| kord_item_to_target_tensor(&self.device, i)).collect();

        let frequency_spaces = Tensor::cat(samples, 0).to_device(&self.device).detach();
        let targets = Tensor::cat(targets, 0).int().to_device(&self.device); // No need to detach targets because `int()` essentially creates a new tensor.

        KordBatch { samples: frequency_spaces, targets }
    }
}
