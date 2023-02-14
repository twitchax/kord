//! Module for all sampling and training code.

pub mod base;
pub mod data;
pub mod gather;
pub mod helpers;
pub mod mlp;
pub mod model;
pub mod execute;

pub use execute::run;