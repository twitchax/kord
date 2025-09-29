//! Module that defines how data is batched and loaded for training.

use std::path::{Path, PathBuf};

use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    tensor::{backend::Backend, Int, Tensor},
};
use rand::seq::SliceRandom;
use rayon::prelude::*;

use crate::{
    core::base::Res,
    ml::base::{
        data::{kord_item_to_sample_tensor, kord_item_to_target_tensor},
        helpers::load_kord_item,
        KordItem,
    },
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
    ///
    /// This will load a simulated training set and a folder-based validation set.
    pub fn from_simulated_training_and_folder_validation<R>(
        noise_asset_root: R,
        validation_source: impl AsRef<Path>,
        count: usize,
        peak_radius: f32,
        harmonic_decay: f32,
        frequency_wobble: f32,
    ) -> Res<(Self, Self)>
    where
        R: AsRef<Path> + Clone + Send + Sync,
    {
        let test_files = get_bin_files_in_directory(validation_source);

        let test_items: Vec<_> = test_files.par_iter().map(load_kord_item).collect::<Res<Vec<_>>>()?;
        let train_items = get_simulated_kord_items(noise_asset_root, count, peak_radius, harmonic_decay, frequency_wobble)?;

        // Return the train and test datasets.
        let train = Self { items: train_items };
        let test = Self { items: test_items };

        Ok((train, test))
    }

    /// Load the training set and testing set from the given folder (no simulation).
    ///
    /// The items are shuffled before splitting, and the dataset is split 80 / 20.
    pub fn from_one_folder(training_source: impl AsRef<Path>) -> Res<(Self, Self)> {
        let files = get_bin_files_in_directory(training_source);

        let mut items: Vec<_> = files.par_iter().map(load_kord_item).collect::<Res<Vec<_>>>()?;
        items.shuffle(&mut rand::rng());

        // Split the items into train and test sets (80/20 split).
        let split = (items.len() as f32 * 0.8) as usize;
        let (train_items, test_items) = items.split_at(split);

        let train = Self { items: train_items.to_vec() };
        let test = Self { items: test_items.to_vec() };

        Ok((train, test))
    }

    /// Load the training set and validation set from the given folders.
    ///
    /// If the validation source is `None`, the training set will be split 80/20.
    pub fn from_two_folders(training_source: impl AsRef<Path>, validation_source: &Option<impl AsRef<Path>>) -> Res<(Self, Self)> {
        match validation_source {
            Some(validation_source) => {
                let train = Self::from_folder(training_source)?;
                let validation = Self::from_folder(validation_source)?;
                Ok((train, validation))
            }
            None => Self::from_one_folder(training_source),
        }
    }

    /// Load the dataset from the given folder.
    pub fn from_folder(training_source: impl AsRef<Path>) -> Res<Self> {
        let files = get_bin_files_in_directory(training_source);
        let items: Vec<_> = files.par_iter().map(load_kord_item).collect::<Res<Vec<_>>>()?;
        Ok(Self { items })
    }
}

// Get all bin files in a directory.
fn get_bin_files_in_directory(name: impl AsRef<Path>) -> Vec<PathBuf> {
    std::fs::read_dir(name)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .filter(|path| path.extension().unwrap() == "bin")
        .collect()
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
