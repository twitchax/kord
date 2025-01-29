//! Base types for machine learning.

// Add an allow for the [`Config`] derive on [`TrainConfig`].
#![allow(clippy::too_many_arguments)]

pub mod data;
#[cfg(feature = "analyze_mic")]
pub mod gather;
pub mod helpers;
pub mod mlp;
pub mod model;

use burn::config::Config;
use std::path::PathBuf;

/// The standard frequency space size to use across all ML operations.
///
/// This covers up to C9, which is beyond the range of a standard 88-key piano (C8).
pub const FREQUENCY_SPACE_SIZE: usize = 8192;

/// The standard mel space size to use across all ML operations.
pub const INPUT_SPACE_SIZE: usize = NUM_CLASSES + 128;

/// The standard mel space size to use across all ML operations.
pub const MEL_SPACE_SIZE: usize = 512;

/// The standard number of classes to use across all ML operations.
pub const NUM_CLASSES: usize = 128;

// Training configuration.

/// The training configuration used for all training, inference, and hyper parameter tuning.
#[derive(Debug, Config)]
pub struct TrainConfig {
    /// The source directory for the noise assets used to generate simulated items.
    pub noise_asset_root: String,
    /// The source directory for the gathered samples.
    pub source: String,
    /// The destination directory for the trained model.
    pub destination: String,
    /// The log directory for training.
    pub log: String,

    /// Simulation data set size.
    pub simulation_size: usize,
    /// Simulation peak radius.
    pub simulation_peak_radius: f32,
    /// Simulation harmonic decay.
    pub simulation_harmonic_decay: f32,
    /// Simulation frequency wobble.
    pub simulation_frequency_wobble: f32,

    /// The number of Multi Head Attention (MHA) heads.
    pub mha_heads: usize,
    /// The Multi Head Attention (MHA) dropout rate.
    pub mha_dropout: f64,

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

    /// Suppresses the training plots.
    pub no_plots: bool,
}

/// A single kord sample.
///
/// This is a single sample of a kord, which is a set of notes played together.
#[derive(Clone, Debug)]
pub struct KordItem {
    /// The path to the sample.
    pub path: PathBuf,
    /// The frequency space of the sample.
    pub frequency_space: [f32; FREQUENCY_SPACE_SIZE],
    /// The label of the sample.
    pub label: u128,
}

impl Default for KordItem {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            frequency_space: [0.0; FREQUENCY_SPACE_SIZE],
            label: 0,
        }
    }
}
