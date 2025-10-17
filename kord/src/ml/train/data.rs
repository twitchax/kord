//! Module that defines how data is batched and loaded for training.

use std::path::{Path, PathBuf};

use anyhow::{bail, Context};

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
    /// Load training and validation datasets from the provided sources.
    ///
    /// When `validation_sources` is empty the merged training pool is shuffled and
    /// split 80/20 to provide both datasets. Otherwise, each side is built from the
    /// corresponding list of sources. The special source name `"sim"` injects the
    /// configured simulated samples into the merged pool.
    #[allow(clippy::too_many_arguments)]
    pub fn from_sources<R>(
        noise_asset_root: R,
        training_sources: &[String],
        validation_sources: &[String],
        simulation_size: usize,
        peak_radius: f32,
        harmonic_decay: f32,
        frequency_wobble: f32,
        captured_oversample_factor: usize,
    ) -> Res<(Self, Self)>
    where
        R: AsRef<Path> + Clone + Send + Sync,
    {
        if training_sources.is_empty() {
            bail!("training_sources must contain at least one entry");
        }

        let mut train_items = collect_items_from_sources(
            noise_asset_root.clone(),
            training_sources,
            simulation_size,
            peak_radius,
            harmonic_decay,
            frequency_wobble,
            captured_oversample_factor,
        )?;

        if train_items.is_empty() {
            bail!("no training samples found for the requested sources");
        }

        train_items.shuffle(&mut rand::rng());

        let validation_items = if validation_sources.is_empty() {
            let len = train_items.len();
            if len <= 1 {
                Vec::new()
            } else {
                let mut split_index = (len as f32 * 0.8).floor() as usize;
                if split_index == 0 {
                    split_index = 1;
                }
                if split_index >= len {
                    split_index = len - 1;
                }
                train_items.split_off(split_index)
            }
        } else {
            let mut items = collect_items_from_sources(
                noise_asset_root,
                validation_sources,
                simulation_size,
                peak_radius,
                harmonic_decay,
                frequency_wobble,
                captured_oversample_factor,
            )?;
            items.shuffle(&mut rand::rng());
            items
        };

        Ok((Self { items: train_items }, Self { items: validation_items }))
    }

    /// Load the dataset from the given folder.
    pub fn from_folder(training_source: impl AsRef<Path>) -> Res<Self> {
        let files = get_bin_files_in_directory(training_source)?;
        let items: Vec<_> = files.par_iter().map(load_kord_item).collect::<Res<Vec<_>>>()?;
        Ok(Self { items })
    }
}

// Get all bin files in a directory.
fn get_bin_files_in_directory(name: impl AsRef<Path>) -> Res<Vec<PathBuf>> {
    let path = name.as_ref();
    let entries = std::fs::read_dir(path).with_context(|| format!("failed to read directory {}", path.display()))?;

    let mut bin_files = Vec::new();
    for entry in entries {
        let entry = entry.with_context(|| format!("failed to read directory entry in {}", path.display()))?;
        let path = entry.path();
        if path.is_file() && matches!(path.extension().and_then(|ext| ext.to_str()), Some("bin")) {
            bin_files.push(path);
        }
    }

    Ok(bin_files)
}

fn collect_items_from_sources<R>(
    noise_asset_root: R,
    sources: &[String],
    simulation_size: usize,
    peak_radius: f32,
    harmonic_decay: f32,
    frequency_wobble: f32,
    captured_oversample_factor: usize,
) -> Res<Vec<KordItem>>
where
    R: AsRef<Path> + Clone + Send + Sync,
{
    let mut items = Vec::new();

    for source in sources {
        if source.eq_ignore_ascii_case("sim") {
            if simulation_size == 0 {
                continue;
            }

            let mut simulated = get_simulated_kord_items(noise_asset_root.clone(), simulation_size, peak_radius, harmonic_decay, frequency_wobble)?;
            items.append(&mut simulated);
            continue;
        }

        let files = get_bin_files_in_directory(source)?;
        let mut folder_items: Vec<_> = files.par_iter().map(load_kord_item).collect::<Res<Vec<_>>>()?;

        if should_oversample_captured(source, captured_oversample_factor) {
            folder_items = oversample_items(&folder_items, captured_oversample_factor);
        }

        items.append(&mut folder_items);
    }

    Ok(items)
}

fn should_oversample_captured(source: &str, factor: usize) -> bool {
    factor > 1 && source.to_ascii_lowercase().contains("captured")
}

fn oversample_items(items: &[KordItem], factor: usize) -> Vec<KordItem> {
    if factor <= 1 || items.is_empty() {
        return items.to_vec();
    }

    let mut oversampled = Vec::with_capacity(items.len() * factor);
    for _ in 0..factor {
        oversampled.extend(items.iter().cloned());
    }

    oversampled
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
