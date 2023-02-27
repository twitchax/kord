//! The `ml` module contains the core ml logic for gathering, training, and inference.

#[cfg(all(feature = "ml_base", feature = "analyze_base"))]
pub mod base;

#[cfg(all(feature = "ml_train", feature = "analyze_base"))]
pub mod train;

#[cfg(all(feature = "ml_infer", feature = "analyze_base"))]
pub mod infer;
