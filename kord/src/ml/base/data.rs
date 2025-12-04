//! Generic data structures and functions for training or inference.

use burn::tensor::{backend::Backend, Tensor, TensorData};

use super::{helpers::u128_to_binary, KordItem, INPUT_SPACE_SIZE, TARGET_SPACE_SIZE};

#[cfg(feature = "ml_target_folded_bass")]
use super::PITCH_CLASS_COUNT;

#[cfg(any(feature = "ml_target_folded", feature = "ml_target_folded_bass"))]
use super::helpers::fold_binary;

#[cfg(feature = "ml_loader_note_binned_convolution")]
use super::helpers::note_binned_convolution;

#[cfg(feature = "ml_loader_mel")]
use super::helpers::mel_filter_banks_from;

#[cfg(feature = "ml_loader_frequency_pooled")]
use super::helpers::average_pool_frequency_space;

#[cfg(feature = "ml_loader_include_deterministic_guess")]
use super::{helpers::get_deterministic_guess, DETERMINISTIC_GUESS_SIZE};

/// Takes a loaded kord item and converts it to a sample tensor that is ready for classification.
pub fn kord_item_to_sample_tensor<B: Backend>(device: &B::Device, item: &KordItem) -> Tensor<B, 2> {
    #[cfg(feature = "ml_loader_note_binned_convolution")]
    {
        sample_from_note_binned_convolution(device, item)
    }

    #[cfg(feature = "ml_loader_mel")]
    {
        sample_from_mel(device, item)
    }

    #[cfg(feature = "ml_loader_frequency")]
    {
        sample_from_frequency(device, item)
    }

    #[cfg(feature = "ml_loader_frequency_pooled")]
    {
        sample_from_frequency_pooled(device, item)
    }
}

#[cfg(feature = "ml_loader_note_binned_convolution")]
fn sample_from_note_binned_convolution<B: Backend>(device: &B::Device, item: &KordItem) -> Tensor<B, 2> {
    let mut convolution = note_binned_convolution(&item.frequency_space);
    normalize(&mut convolution);
    sample_tensor_from_parts(device, item, convolution.to_vec())
}

#[cfg(feature = "ml_loader_mel")]
fn sample_from_mel<B: Backend>(device: &B::Device, item: &KordItem) -> Tensor<B, 2> {
    let mut mel_space = mel_filter_banks_from(&item.frequency_space);
    normalize(&mut mel_space);
    sample_tensor_from_parts(device, item, mel_space.to_vec())
}

#[cfg(feature = "ml_loader_frequency")]
fn sample_from_frequency<B: Backend>(device: &B::Device, item: &KordItem) -> Tensor<B, 2> {
    let mut frequency_space = item.frequency_space;
    normalize(&mut frequency_space);
    sample_tensor_from_parts(device, item, frequency_space.to_vec())
}

#[cfg(feature = "ml_loader_frequency_pooled")]
fn sample_from_frequency_pooled<B: Backend>(device: &B::Device, item: &KordItem) -> Tensor<B, 2> {
    let mut frequency_space = average_pool_frequency_space(&item.frequency_space);
    normalize(&mut frequency_space);
    sample_tensor_from_parts(device, item, frequency_space.to_vec())
}

#[allow(unused_variables)]
fn sample_tensor_from_parts<B: Backend>(device: &B::Device, item: &KordItem, mut features: Vec<f32>) -> Tensor<B, 2> {
    #[cfg(feature = "ml_loader_include_deterministic_guess")]
    {
        let mut combined = Vec::with_capacity(DETERMINISTIC_GUESS_SIZE + features.len());
        combined.extend_from_slice(&deterministic_guess_array(item));
        combined.extend_from_slice(&features);
        features = combined;
    }

    to_zero_mean_unit_variance(features.as_mut_slice());

    tensor_from_vec_with_expected_size(device, features, INPUT_SPACE_SIZE)
}

/// Takes a loaded kord item and converts it to a target tensor that is ready for classification.
#[cfg(feature = "ml_target_full")]
pub fn kord_item_to_target_tensor<B: Backend>(device: &B::Device, item: &KordItem) -> Tensor<B, 2> {
    let binary_full = u128_to_binary(item.label);
    tensor_from_vec_with_expected_size(device, binary_full.to_vec(), TARGET_SPACE_SIZE)
}

/// Takes a loaded kord item and converts it to a folded target tensor that is ready for classification.
#[cfg(feature = "ml_target_folded")]
pub fn kord_item_to_target_tensor<B: Backend>(device: &B::Device, item: &KordItem) -> Tensor<B, 2> {
    let binary_full = u128_to_binary(item.label);
    let folded = fold_binary(&binary_full);
    tensor_from_vec_with_expected_size(device, folded.to_vec(), TARGET_SPACE_SIZE)
}

/// Takes a loaded kord item and converts it to a folded+bass target tensor that is ready for classification.
#[cfg(feature = "ml_target_folded_bass")]
pub fn kord_item_to_target_tensor<B: Backend>(device: &B::Device, item: &KordItem) -> Tensor<B, 2> {
    let binary_full = u128_to_binary(item.label);
    let folded = fold_binary(&binary_full);
    let bass = lowest_pitch_class_mask(item.label);

    let mut components = Vec::with_capacity(TARGET_SPACE_SIZE);
    components.extend_from_slice(&bass);
    components.extend_from_slice(&folded);

    tensor_from_vec_with_expected_size(device, components, TARGET_SPACE_SIZE)
}

/// Modifies a slice in place to convert values to zero mean and unit variance.
pub fn to_zero_mean_unit_variance(slice: &mut [f32]) {
    let mean = slice.iter().sum::<f32>() / slice.len() as f32;
    let variance = slice.iter().map(|x| (x - mean).powf(2.0)).sum::<f32>() / slice.len() as f32;
    let std = variance.sqrt();

    if std == 0.0 {
        slice.fill(0.0);
    } else {
        slice.iter_mut().for_each(|x| *x = (*x - mean) / std);
    }
}

/// Normalizes a slice in place.
pub fn normalize(slice: &mut [f32]) {
    let max = slice.iter().fold(0f32, |acc, &x| acc.max(x));

    if max == 0.0 {
        return;
    }

    slice.iter_mut().for_each(|x| *x /= max);
}

fn tensor_from_vec_with_expected_size<B: Backend>(device: &B::Device, data: Vec<f32>, expected: usize) -> Tensor<B, 2> {
    debug_assert_eq!(data.len(), expected, "Tensor length mismatch: expected {expected}, received {}", data.len());

    let len = data.len();
    let tensor_data = TensorData::from(data.as_slice()).convert::<B::FloatElem>();
    let tensor = Tensor::<B, 1>::from_data(tensor_data, device);

    tensor.reshape([1, len])
}

#[cfg(feature = "ml_loader_include_deterministic_guess")]
fn deterministic_guess_array(item: &KordItem) -> [f32; DETERMINISTIC_GUESS_SIZE] {
    let guess_binary = get_deterministic_guess(item);
    u128_to_binary(guess_binary)
}

#[cfg(feature = "ml_target_folded_bass")]
fn lowest_pitch_class_mask(label: u128) -> [f32; PITCH_CLASS_COUNT] {
    let mut mask = [0.0; PITCH_CLASS_COUNT];

    if label != 0 {
        let lowest = label.trailing_zeros() as usize;
        let pitch_class = lowest % PITCH_CLASS_COUNT;
        mask[pitch_class] = 1.0;
    }

    mask
}

#[cfg(all(test, feature = "ml_target_folded_bass"))]
mod tests {
    use super::*;

    #[test]
    fn lowest_pitch_class_mask_marks_expected_bin() {
        let label = 1u128 << 17; // arbitrary note index 17 -> pitch class 5
        let mask = lowest_pitch_class_mask(label);

        assert_eq!(mask.iter().sum::<f32>(), 1.0);
        assert_eq!(mask[5], 1.0);
    }

    #[test]
    fn lowest_pitch_class_mask_handles_empty_label() {
        let mask = lowest_pitch_class_mask(0);
        assert!(mask.iter().all(|v| *v == 0.0));
    }
}
