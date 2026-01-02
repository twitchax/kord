//! Module for executing inference.

use burn::{
    backend::{ndarray::NdArrayDevice, NdArray},
    config::Config,
    module::Module,
    record::{BinBytesRecorder, Recorder},
    tensor::backend::Backend,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

use crate::{
    analyze::base::{get_frequency_space, get_smoothed_frequency_space},
    core::{base::Res, chord::Chord, pitch::Pitch},
    ml::base::{
        data::kord_item_to_sample_tensor,
        helpers::{logits_to_predictions, logits_to_probabilities},
        model::KordModel,
        KordItem, StorePrecisionSettings, TrainConfig, FREQUENCY_SPACE_SIZE, NUM_CLASSES,
    },
};

/// Result of ML inference containing all pitch classes and chord candidates.
#[derive(Debug, Clone)]
pub struct InferenceResult {
    /// All detected pitch classes.
    pub pitches: Vec<Pitch>,
    /// Chord candidates ranked by likelihood.
    pub chords: Vec<Chord>,
    /// Delta (probability - threshold) for each of the 12 pitch classes (C through B).
    /// Positive values indicate detected pitches, negative values indicate below threshold.
    pub pitch_deltas: [f32; 12],
}

/// Run ML inference on audio data and return bass, pitches, and chord candidates.
///
/// This is the main entry point for inference. It processes audio data through the ML model
/// and returns a structured result containing the detected bass pitch, all pitch classes,
/// and ranked chord candidates.
pub fn infer(audio_data: &[f32], length_in_seconds: u8) -> Res<InferenceResult> {
    let frequency_space = get_frequency_space(audio_data, length_in_seconds);
    let smoothed_frequency_space: [_; FREQUENCY_SPACE_SIZE] = get_smoothed_frequency_space(&frequency_space, length_in_seconds)
        .into_iter()
        .take(FREQUENCY_SPACE_SIZE)
        .map(|(_, v)| v)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let kord_item = KordItem {
        frequency_space: smoothed_frequency_space,
        ..Default::default()
    };

    let device = NdArrayDevice::Cpu;
    run_inference::<NdArray<f32>>(&device, &kord_item)
}

/// Core inference engine that runs the ML model on prepared input.
fn run_inference<B: Backend>(device: &B::Device, kord_item: &KordItem) -> Res<InferenceResult>
where
    B::FloatElem: Serialize + DeserializeOwned,
{
    // Load the config and state.
    let config = match TrainConfig::load_binary(CONFIG) {
        Ok(config) => config,
        Err(e) => {
            return Err(anyhow::Error::msg(format!("Could not load the config from within the binary: {e}.")));
        }
    };

    let recorder = match BinBytesRecorder::<StorePrecisionSettings>::new().load(Vec::from_iter(STATE_BINCODE.iter().cloned()), device) {
        Ok(recorder) => recorder,
        Err(_) => {
            return Err(anyhow::Error::msg("Could not load the state from within the binary."));
        }
    };

    // Verify we have the expected 12 classes for folded target.
    if NUM_CLASSES != 12 {
        return Err(anyhow::Error::msg(
            "Inference requires folded target with 12 classes; enable `ml_target_folded` when training / building the inference binary.",
        ));
    }

    // Define the model.
    let model = KordModel::<B>::new(device, config.mha_heads, config.dropout, config.trunk_hidden_size).load_record(recorder);

    // Prepare the sample.
    let sample = kord_item_to_sample_tensor(device, kord_item).detach();

    // Run the inference.
    let logits = model.forward(sample).detach();
    let logits_vec: Vec<f32> = logits.into_data().convert::<f32>().to_vec().unwrap_or_default();
    let probabilities = logits_to_probabilities(&logits_vec);
    let thresholds: Vec<f32> = serde_json::from_slice(THRESHOLDS_JSON).unwrap_or_default();
    let inferred = logits_to_predictions(&probabilities, thresholds.as_slice());

    // Decode folded format: 12 pitch classes.
    let mut pitches = Vec::new();
    let mut pitch_deltas = [0.0f32; 12];

    for (pitch_class_index, &is_present) in inferred.iter().take(12).enumerate() {
        // Calculate delta (probability - threshold) for debugging
        let probability = probabilities[pitch_class_index];
        let threshold = thresholds.get(pitch_class_index).copied().unwrap_or(0.5);
        pitch_deltas[pitch_class_index] = probability - threshold;

        if is_present == 1.0 {
            let pitch = Pitch::try_from(pitch_class_index as u8).map_err(|e| anyhow::Error::msg(format!("Invalid pitch class {}: {}", pitch_class_index, e)))?;
            pitches.push(pitch);
        }
    }

    // Generate chord candidates using smart octave permutations.
    // If there are no pitches detected, return empty chord list.
    let chords = if pitches.is_empty() { vec![] } else { Chord::try_from_pitches(&pitches)? };

    Ok(InferenceResult { pitches, chords, pitch_deltas })
}

// Statics - forward slashes work on both Unix and Windows in include_bytes!
static CONFIG: &[u8] = include_bytes!("../../../model/model_config.json");
static STATE_BINCODE: &[u8] = include_bytes!("../../../model/state.json.bin");
static THRESHOLDS_JSON: &[u8] = include_bytes!("../../../model/thresholds.json");

// Tests.

#[cfg(test)]
#[cfg(feature = "ml_infer")]
mod tests {
    use std::{fs::File, io::Read};

    use crate::core::base::HasName;

    use super::*;

    #[test]
    fn test_inference() {
        let mut file = File::open("tests/vec.bin").unwrap();
        let file_size = file.metadata().unwrap().len() as usize;
        let float_size = std::mem::size_of::<f32>();
        let element_count = file_size / float_size;
        let mut buffer = vec![0u8; file_size];

        // Read the contents of the file into the buffer
        file.read_exact(&mut buffer).unwrap();

        // Convert the buffer to a vector of f32
        let audio_data: Vec<f32> = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const f32, element_count).to_vec() };

        // The model always predicts a bass pitch. Pitch classes and chords may be empty for simple audio.
        let inference_result = infer(&audio_data, 5).unwrap();

        assert_eq!(inference_result.pitches.len(), 5);
        assert_eq!(inference_result.chords[0].name_ascii(), "C7(b9)");
    }
}
