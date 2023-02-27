//! Generic data structures and functions for training or inference.

use burn::tensor::{backend::Backend, Data, Tensor};

use crate::analyze::base::translate_frequency_space_to_peak_space;

use super::{
    helpers::{get_deterministic_guess, mel_filter_banks_from, u128_to_binary},
    KordItem, INPUT_SPACE_SIZE, MEL_SPACE_SIZE, NUM_CLASSES,
};

/// Takes a loaded kord item and converts it to a sample tensor that is ready for classification.
pub fn kord_item_to_sample_tensor<B: Backend>(item: &KordItem) -> Tensor<B, 2> {
    kord_item_to_mel_sample_tensor(item)
    //kord_item_to_frequency_sample_tensor(item)
    //kord_item_to_peak_sample_tensor(item)
}

fn _kord_item_to_frequency_sample_tensor<B: Backend>(item: &KordItem) -> Tensor<B, 2> {
    let frequency_space = item.frequency_space;

    // Get the max value.
    let max = frequency_space.iter().fold(0f32, |acc, &x| acc.max(x));

    // Normalize the mel space peaks.
    let mut normalized_frequency_space = [0f32; MEL_SPACE_SIZE];
    (0..MEL_SPACE_SIZE).for_each(|k| {
        normalized_frequency_space[k] = frequency_space[k] / max;
    });

    // Get the "deterministic guess".
    let deterministic_guess = u128_to_binary(get_deterministic_guess(item));

    let result: [f32; INPUT_SPACE_SIZE] = [&deterministic_guess[..], &normalized_frequency_space[..]].concat().try_into().unwrap();
    //let result = normalized_frequency_space;

    let data = Data::<f32, 1>::from(result);
    let tensor = Tensor::<B, 1>::from_data(data.convert());

    tensor.reshape([1, INPUT_SPACE_SIZE])
}

/// Takes a loaded kord item and converts it to a sample tensor that is ready for classification.
fn kord_item_to_mel_sample_tensor<B: Backend>(item: &KordItem) -> Tensor<B, 2> {
    let frequency_space = item.frequency_space;
    let mel_space = mel_filter_banks_from(&frequency_space);

    // Get the max value.
    let max = mel_space.iter().fold(0f32, |acc, &x| acc.max(x));

    // Normalize the mel space peaks.
    let mut normalized_mel_space = [0f32; MEL_SPACE_SIZE];
    (0..MEL_SPACE_SIZE).for_each(|k| {
        normalized_mel_space[k] = mel_space[k] / max;
    });

    // Get the "deterministic guess".
    let deterministic_guess = u128_to_binary(get_deterministic_guess(item));
    //let deterministic_guess = fold_binary(&deterministic_guess);

    let result: [f32; INPUT_SPACE_SIZE] = [&deterministic_guess[..], &normalized_mel_space[..]].concat().try_into().unwrap();
    //let result = normalized_mel_space;

    // Convert the result values to zero-mean and unit-variance.
    let mean = result.iter().sum::<f32>() / result.len() as f32;
    let variance = result.iter().map(|x| (x - mean).powf(2.0)).sum::<f32>() / result.len() as f32;
    let std = variance.sqrt();

    let result: [f32; INPUT_SPACE_SIZE] = result.iter().map(|x| (x - mean) / std).collect::<Vec<_>>().try_into().unwrap();

    let data = Data::<f32, 1>::from(result);
    let tensor = Tensor::<B, 1>::from_data(data.convert());

    tensor.reshape([1, INPUT_SPACE_SIZE])
}

/// Takes a loaded kord item and converts it to a sample tensor that is ready for classification.
fn _kord_item_to_peak_sample_tensor<B: Backend>(item: &KordItem) -> Tensor<B, 2> {
    let frequency_space = item.frequency_space;
    let frequency_space = frequency_space.into_iter().enumerate().map(|(k, v)| (k as f32, v)).collect::<Vec<_>>();

    let mut peak_space = translate_frequency_space_to_peak_space(&frequency_space);
    peak_space.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    peak_space.iter_mut().skip(12).for_each(|(_, v)| *v = 0.0);
    peak_space.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let peak_space = peak_space.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    let mel_space = mel_filter_banks_from(&peak_space);

    // Get the max value.
    let max = mel_space.iter().fold(0f32, |acc, &x| acc.max(x));

    // Normalize the mel space peaks.
    let mut normalized_mel_space = [0f32; MEL_SPACE_SIZE];
    (0..MEL_SPACE_SIZE).for_each(|k| {
        normalized_mel_space[k] = mel_space[k] / max;
    });

    // Get the "deterministic guess".
    let deterministic_guess = u128_to_binary(get_deterministic_guess(item));
    //let deterministic_guess = fold_binary(&deterministic_guess);

    let result: [f32; INPUT_SPACE_SIZE] = [&deterministic_guess[..], &normalized_mel_space[..]].concat().try_into().unwrap();

    let data = Data::<f32, 1>::from(result);
    let tensor = Tensor::<B, 1>::from_data(data.convert());

    tensor.reshape([1, INPUT_SPACE_SIZE])
}

/// Takes a loaded kord item and converts it to a target tensor that is ready for classification.
pub fn kord_item_to_target_tensor<B: Backend>(item: &KordItem) -> Tensor<B, 2> {
    let binary = u128_to_binary(item.label);

    //let folded = fold_binary(&binary);

    let data = Data::<f32, 1>::from(binary);
    let tensor = Tensor::<B, 1>::from_data(data.convert());

    tensor.reshape([1, NUM_CLASSES])
}
