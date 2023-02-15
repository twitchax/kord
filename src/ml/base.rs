//! Base types for machine learning.

use std::path::PathBuf;

/// The standard frequency space size to use across all ML operations.
///
/// This covers up to C9, which is beyond the range of a standard 88-key piano (C8).
pub(crate) const FREQUENCY_SPACE_SIZE: usize = 8192;

/// The standard mel space size to use across all ML operations.
pub(crate) const INPUT_SPACE_SIZE: usize = MEL_SPACE_SIZE + 128;

/// The standard mel space size to use across all ML operations.
pub(crate) const MEL_SPACE_SIZE: usize = 512;

/// The standard number of classes to use across all ML operations.
pub(crate) const NUM_CLASSES: usize = 128;

/// A single kord sample.
///
/// This is a single sample of a kord, which is a set of notes played together.
#[derive(Clone, Debug)]
pub(crate) struct KordItem {
    pub path: PathBuf,
    pub frequency_space: [f32; FREQUENCY_SPACE_SIZE],
    pub label: u128,
}
