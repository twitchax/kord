//! The `ml` module contains the core ml logic for gathering, training, and inference.

#[cfg(feature = "ml_base")]
pub mod base;

#[cfg(all(feature = "ml_gather", feature = "analyze_mic"))]
pub mod gather;
