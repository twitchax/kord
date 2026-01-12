//! The `ml` module contains the core ml logic for gathering, training, and inference.

#[cfg(feature = "ml_base")]
pub mod base;

#[cfg(feature = "ml_train")]
pub mod train;

#[cfg(feature = "ml_infer")]
pub mod infer;
