//! Precision selection for ML pipelines.

use burn::{
    record::{FullPrecisionSettings, HalfPrecisionSettings},
    tensor::DType,
};

#[cfg(all(feature = "ml_train_precision_fp32", feature = "ml_train_precision_fp16"))]
compile_error!("`ml_train_precision_fp32` and `ml_train_precision_fp16` cannot be enabled together.");

#[cfg(all(feature = "ml_train_precision_fp32", feature = "ml_train_precision_bf16"))]
compile_error!("`ml_train_precision_fp32` and `ml_train_precision_bf16` cannot be enabled together.");

#[cfg(all(feature = "ml_train_precision_fp16", feature = "ml_train_precision_bf16"))]
compile_error!("`ml_train_precision_fp16` and `ml_train_precision_bf16` cannot be enabled together.");

#[cfg(not(any(feature = "ml_train_precision_fp32", feature = "ml_train_precision_fp16", feature = "ml_train_precision_bf16",)))]
compile_error!("No ML training precision feature enabled; enable exactly one of `ml_train_precision_fp32`, `ml_train_precision_fp16`, or `ml_train_precision_bf16`.");

#[cfg(all(feature = "ml_ndarray", any(feature = "ml_train_precision_fp16", feature = "ml_train_precision_bf16")))]
compile_error!("The NdArray backend only supports `ml_train_precision_fp32` for training. Choose a GPU backend for reduced precision training.");

#[cfg(all(feature = "ml_store_precision_full", feature = "ml_store_precision_half"))]
compile_error!("`ml_store_precision_full` and `ml_store_precision_half` cannot be enabled together.");

#[cfg(not(any(feature = "ml_store_precision_full", feature = "ml_store_precision_half")))]
compile_error!("No ML storage precision feature enabled; enable exactly one of `ml_store_precision_full` or `ml_store_precision_half`.");

/// The element type used throughout the ML training pipelines.
///
/// This type is selected via feature flags:
/// - `ml_train_precision_fp32` for `f32`
/// - `ml_train_precision_fp16` for `burn::tensor::f16`
/// - `ml_train_precision_bf16` for `burn::tensor::bf16`
#[cfg(feature = "ml_train_precision_fp32")]
pub type PrecisionElement = f32;

/// The element type used throughout the ML training pipelines.
///
/// This type is selected via feature flags:
/// - `ml_train_precision_fp32` for `f32`
/// - `ml_train_precision_fp16` for `burn::tensor::f16`
/// - `ml_train_precision_bf16` for `burn::tensor::bf16`
#[cfg(feature = "ml_train_precision_fp16")]
pub type PrecisionElement = burn::tensor::f16;

/// The element type used throughout the ML training pipelines.
///
/// This type is selected via feature flags:
/// - `ml_train_precision_fp32` for `f32`
/// - `ml_train_precision_fp16` for `burn::tensor::f16`
/// - `ml_train_precision_bf16` for `burn::tensor::bf16`
#[cfg(feature = "ml_train_precision_bf16")]
pub type PrecisionElement = burn::tensor::bf16;

/// The data type used throughout the ML pipelines.
///
/// This type is selected via feature flags:
/// - `ml_train_precision_fp32` for `DType::F32`
/// - `ml_train_precision_fp16` for `DType::F16`
/// - `ml_train_precision_bf16` for `DType::BF16`
#[cfg(feature = "ml_train_precision_fp32")]
pub const PRECISION_DTYPE: DType = DType::F32;

/// The data type used throughout the ML pipelines.
///
/// This type is selected via feature flags:
/// - `ml_train_precision_fp32` for `DType::F32`
/// - `ml_train_precision_fp16` for `DType::F16`
/// - `ml_train_precision_bf16` for `DType::BF16`
#[cfg(feature = "ml_train_precision_fp16")]
pub const PRECISION_DTYPE: DType = DType::F16;

/// The data type used throughout the ML pipelines.
///
/// This type is selected via feature flags:
/// - `ml_train_precision_fp32` for `DType::F32`
/// - `ml_train_precision_fp16` for `DType::F16`
/// - `ml_train_precision_bf16` for `DType::BF16`
#[cfg(feature = "ml_train_precision_bf16")]
pub const PRECISION_DTYPE: DType = DType::BF16;

/// The precision settings used when serializing model artifacts.
///
/// This type is selected via feature flags:
/// - `ml_store_precision_full` for `FullPrecisionSettings`
/// - `ml_store_precision_half` for `HalfPrecisionSettings`
#[cfg(feature = "ml_store_precision_full")]
pub type StorePrecisionSettings = FullPrecisionSettings;

/// The precision settings used when serializing model artifacts.
///
/// This type is selected via feature flags:
/// - `ml_store_precision_full` for `FullPrecisionSettings`
/// - `ml_store_precision_half` for `HalfPrecisionSettings`
#[cfg(feature = "ml_store_precision_half")]
pub type StorePrecisionSettings = HalfPrecisionSettings;
