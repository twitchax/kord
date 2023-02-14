//! The `ml` module contains the core ml logic for gathering, training, and inference.

#[cfg(feature = "ml_base")]
pub mod base;

#[cfg(all(feature = "ml_train", feature = "analyze_mic"))]
pub mod train;

// #[cfg(all(feature = "ml_infer",feature = "analyze_mic"))]
// pub mod infer;
