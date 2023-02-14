//! Base types for machine learning.

/// The standard frequency space size to use across all ML operations.
/// 
/// This covers up to C9, which is beyond the range of a standard 88-key piano (C8).
pub(crate) const FREQUENCY_SPACE_SIZE: usize = 8192;

/// A single kord sample.
/// 
/// This is a single sample of a kord, which is a set of notes played together.
#[derive(Clone, Debug)]
pub(crate) struct KordItem {
    pub frequency_space: [f32; FREQUENCY_SPACE_SIZE],
    pub label: u128,
}