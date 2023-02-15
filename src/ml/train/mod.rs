//! Module for all sampling and training code.

pub mod base;
pub mod data;
pub mod execute;
pub mod gather;
pub mod helpers;
pub mod mlp;
pub mod model;

pub use execute::run;
