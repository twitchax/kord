//! Base types for machine learning.

// Add an allow for the [`Config`] derive on [`TrainConfig`].
#![allow(clippy::too_many_arguments)]

pub mod data;
#[cfg(all(feature = "ml_sample_gather", feature = "analyze_mic"))]
pub mod gather;
pub mod helpers;
pub mod model;
#[cfg(feature = "ml_sample_process")]
pub mod process;

use burn::config::Config;
use std::path::PathBuf;

/// The standard frequency space size to use across all ML operations.
///
/// This covers up to C9, which is beyond the range of a standard 88-key piano (C8).
pub const FREQUENCY_SPACE_SIZE: usize = 8192;

/// The number of MIDI-indexed note bins used throughout the pipeline.
pub const NOTE_SIGNATURE_SIZE: usize = 128;

/// The number of pitch classes in a single octave (C through B).
pub const PITCH_CLASS_COUNT: usize = 12;

/// The deterministic guess vector mirrors the MIDI note signature.
pub const DETERMINISTIC_GUESS_SIZE: usize = NOTE_SIGNATURE_SIZE;

/// The standard mel space size to use across all ML operations.
pub const MEL_SPACE_SIZE: usize = 512;

/// Ensure exactly one primary sample loader feature is enabled when ML base is compiled.
#[cfg(any(
    all(
        feature = "ml_loader_note_binned_convolution",
        any(feature = "ml_loader_mel", feature = "ml_loader_frequency", feature = "ml_loader_frequency_pooled")
    ),
    all(feature = "ml_loader_mel", any(feature = "ml_loader_frequency", feature = "ml_loader_frequency_pooled")),
    all(feature = "ml_loader_frequency", feature = "ml_loader_frequency_pooled"),
))]
compile_error!(
    "Multiple ml_loader_* features enabled; enable exactly one of: \
     ml_loader_note_binned_convolution, ml_loader_mel, ml_loader_frequency, ml_loader_frequency_pooled."
);

#[cfg(not(any(
    feature = "ml_loader_note_binned_convolution",
    feature = "ml_loader_mel",
    feature = "ml_loader_frequency",
    feature = "ml_loader_frequency_pooled",
)))]
compile_error!(
    "No ml_loader_* feature enabled; enable exactly one of: \
     ml_loader_note_binned_convolution, ml_loader_mel, ml_loader_frequency, ml_loader_frequency_pooled."
);

/// The base dimensionality of the sample tensor produced by `kord_item_to_sample_tensor`.
#[cfg(feature = "ml_loader_note_binned_convolution")]
const INPUT_BASE_SIZE: usize = NOTE_SIGNATURE_SIZE;

/// The base dimensionality of the sample tensor produced by `kord_item_to_sample_tensor`.
#[cfg(feature = "ml_loader_mel")]
const INPUT_BASE_SIZE: usize = MEL_SPACE_SIZE;

/// The base dimensionality of the sample tensor produced by `kord_item_to_sample_tensor`.
#[cfg(feature = "ml_loader_frequency")]
const INPUT_BASE_SIZE: usize = FREQUENCY_SPACE_SIZE;

/// The frequency pooling factor applied when using the pooled loader variant.
#[cfg(feature = "ml_loader_frequency_pooled")]
pub const FREQUENCY_POOL_FACTOR: usize = 32;

/// The dimensionality of the pooled frequency space representation.
#[cfg(feature = "ml_loader_frequency_pooled")]
pub const FREQUENCY_SPACE_POOLED_SIZE: usize = FREQUENCY_SPACE_SIZE / FREQUENCY_POOL_FACTOR;

/// The base dimensionality of the sample tensor produced by `kord_item_to_sample_tensor`.
#[cfg(feature = "ml_loader_frequency_pooled")]
const INPUT_BASE_SIZE: usize = FREQUENCY_SPACE_POOLED_SIZE;

/// The dimensionality of the sample tensor produced by `kord_item_to_sample_tensor`.
#[cfg(feature = "ml_loader_include_deterministic_guess")]
pub const INPUT_SPACE_SIZE: usize = INPUT_BASE_SIZE + DETERMINISTIC_GUESS_SIZE;

/// The dimensionality of the sample tensor produced by `kord_item_to_sample_tensor`.
#[cfg(not(feature = "ml_loader_include_deterministic_guess"))]
pub const INPUT_SPACE_SIZE: usize = INPUT_BASE_SIZE;

/// Ensure at least one target encoding feature is enabled when ML base is compiled.
#[cfg(not(any(feature = "ml_target_full", feature = "ml_target_folded")))]
compile_error!("No ml_target_* feature enabled; enable at least one of: ml_target_full, ml_target_folded.");

/// Whether the full 128-note target encoding is enabled.
#[cfg(feature = "ml_target_full")]
const TARGET_FULL_SIZE: usize = NOTE_SIGNATURE_SIZE;

#[cfg(not(feature = "ml_target_full"))]
const TARGET_FULL_SIZE: usize = 0;

/// Whether the folded 12-class target encoding is enabled.
#[cfg(feature = "ml_target_folded")]
const TARGET_FOLDED_SIZE: usize = PITCH_CLASS_COUNT;

#[cfg(not(feature = "ml_target_folded"))]
const TARGET_FOLDED_SIZE: usize = 0;

/// The dimensionality of the target tensor produced by `kord_item_to_target_tensor`.
pub const TARGET_SPACE_SIZE: usize = TARGET_FULL_SIZE + TARGET_FOLDED_SIZE;

/// Backward-compatible alias for target dimensionality.
pub const NUM_CLASSES: usize = TARGET_SPACE_SIZE;

// Training configuration.

/// The training configuration used for all training, inference, and hyper parameter tuning.
#[derive(Debug, Config)]
pub struct TrainConfig {
    /// The source directory for the noise assets used to generate simulated items.
    pub noise_asset_root: String,
    /// The source directory for the training samples.
    pub training_source: String,
    /// The source directory for the validation samples.
    pub validation_source: Option<String>,
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
    pub adam_weight_decay: f32,
    /// The Adam optimizer beta1.
    pub adam_beta1: f32,
    /// The Adam optimizer beta2.
    pub adam_beta2: f32,
    /// The Adam optimizer epsilon.
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
