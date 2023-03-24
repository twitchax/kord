//! Generic data structures and functions for training or inference.

use burn::tensor::{backend::Backend, Data, Tensor};

use super::{
    helpers::{get_deterministic_guess, mel_filter_banks_from, u128_to_binary},
    KordItem, INPUT_SPACE_SIZE, NUM_CLASSES,
};

/// Takes a loaded kord item and converts it to a sample tensor that is ready for classification.
pub fn kord_item_to_sample_tensor<B: Backend>(item: &KordItem) -> Tensor<B, 2> {
    kord_item_to_mel_sample_tensor(item)
    //kord_item_to_bins_sample_tensor(item)
}

/// Takes a loaded kord item and converts it to a sample tensor that is ready for classification.
fn kord_item_to_mel_sample_tensor<B: Backend>(item: &KordItem) -> Tensor<B, 2> {
    let frequency_space = item.frequency_space;
    let mut mel_space = mel_filter_banks_from(&frequency_space);

    // Normalize the mel space peaks.
    normalize(&mut mel_space);

    // Get the "deterministic guess".
    let deterministic_guess: [f32; 128] = u128_to_binary(get_deterministic_guess(item)).iter().map(|v| v * 1.0).collect::<Vec<_>>().try_into().unwrap();
    //let deterministic_guess = fold_binary(&deterministic_guess);

    let mut result: [f32; INPUT_SPACE_SIZE] = [&deterministic_guess[..], &mel_space[..]].concat().try_into().unwrap();
    //let mut result = mel_space;

    // Convert the result values to zero-mean and unit-variance.
    to_zero_mean_unit_variance(&mut result);

    let data = Data::<f32, 1>::from(result);
    let tensor = Tensor::<B, 1>::from_data(data.convert());

    tensor.reshape([1, INPUT_SPACE_SIZE])
}

/// Takes a loaded kord item and converts it to a target tensor that is ready for classification.
pub fn kord_item_to_target_tensor<B: Backend>(item: &KordItem) -> Tensor<B, 2> {
    let binary = u128_to_binary(item.label);

    //let binary = fold_binary(&binary);

    let data = Data::<f32, 1>::from(binary);
    let tensor = Tensor::<B, 1>::from_data(data.convert());

    tensor.reshape([1, NUM_CLASSES])
}

/// Modifies a slice in place to convert values to zero mean and unit variance.
pub fn to_zero_mean_unit_variance(slice: &mut [f32]) {
    let mean = slice.iter().sum::<f32>() / slice.len() as f32;
    let variance = slice.iter().map(|x| (x - mean).powf(2.0)).sum::<f32>() / slice.len() as f32;
    let std = variance.sqrt();

    slice.iter_mut().for_each(|x| *x = (*x - mean) / std);
}

/// Normalizes a slice in place.
pub fn normalize(slice: &mut [f32]) {
    let max = slice.iter().fold(0f32, |acc, &x| acc.max(x));

    slice.iter_mut().for_each(|x| *x /= max);
}