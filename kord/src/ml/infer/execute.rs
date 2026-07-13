//! Module for executing inference.

use std::sync::{LazyLock, Mutex};

use burn::{
    backend::{ndarray::NdArrayDevice, NdArray},
    config::Config,
    module::Module,
    record::{BinBytesRecorder, Recorder},
};

use crate::{
    analyze::base::{get_frequency_space, get_smoothed_frequency_space},
    core::{base::Res, chord::Chord, pitch::Pitch},
    ml::base::{
        data::kord_item_to_sample_tensor,
        helpers::{logits_to_predictions, logits_to_probabilities},
        model::KordModel,
        KordItem, StorePrecisionSettings, TrainConfig, FREQUENCY_SPACE_SIZE, PITCH_CLASS_COUNT,
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

/// Cached inference state for the `NdArray<f32>` backend.
///
/// Deserializing config, loading model weights, and parsing thresholds is expensive.
/// This struct is initialized once via [`INFERENCE_STATE`] and reused across calls.
struct InferenceState {
    model: KordModel<NdArray<f32>>,
    thresholds: Vec<f32>,
}

static INFERENCE_STATE: LazyLock<Mutex<InferenceState>> = LazyLock::new(|| Mutex::new({
    let device = NdArrayDevice::Cpu;

    let config = TrainConfig::load_binary(CONFIG).expect("Could not load the config from within the binary");

    let recorder = BinBytesRecorder::<StorePrecisionSettings>::new()
        .load(Vec::from_iter(STATE_BINCODE.iter().cloned()), &device)
        .expect("Could not load the state from within the binary");

    let model = KordModel::<NdArray<f32>>::new(&device, config.mha_heads, config.dropout, config.trunk_hidden_size).load_record(recorder);

    let thresholds: Vec<f32> = serde_json::from_slice(THRESHOLDS_JSON).expect("failed to deserialize thresholds");

    InferenceState { model, thresholds }
}));

/// Run ML inference on audio data and return detected pitches and chord candidates.
///
/// This is the main entry point for inference. It processes audio data through the ML model
/// and returns a structured result containing all detected pitch classes and ranked chord
/// candidates.
pub fn infer(audio_data: &[f32], length_in_seconds: u8) -> Res<InferenceResult> {
    let frequency_space = get_frequency_space(audio_data, length_in_seconds);
    let smoothed_frequency_space: [_; FREQUENCY_SPACE_SIZE] = get_smoothed_frequency_space(&frequency_space, length_in_seconds)
        .into_iter()
        .take(FREQUENCY_SPACE_SIZE)
        .map(|(_, v)| v)
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| anyhow::Error::msg("Failed to convert smoothed frequency space into array"))?;

    let kord_item = KordItem {
        frequency_space: smoothed_frequency_space,
        ..Default::default()
    };

    let state = INFERENCE_STATE.lock().map_err(|e| anyhow::anyhow!("inference state lock poisoned: {e}"))?;
    let device = NdArrayDevice::Cpu;

    // Prepare the sample.
    let sample = kord_item_to_sample_tensor(&device, &kord_item).detach();

    // Run the inference.
    let logits = state.model.forward(sample).detach();
    let logits_vec: Vec<f32> = logits
        .into_data()
        .convert::<f32>()
        .to_vec()
        .map_err(|e| anyhow::anyhow!("failed to convert logits tensor to vec: {e:?}"))?;
    let probabilities = logits_to_probabilities(&logits_vec);
    let inferred = logits_to_predictions(&probabilities, &state.thresholds);

    // Decode pitch classes from the prediction vector.
    //
    // For folded_bass the first 12 elements are the bass one-hot; the pitch-class
    // mask lives at indices 12..24. For plain folded, the mask starts at 0.
    // Both are indexed by true pitch class (C=0, Db=1, ..., B=11).
    #[cfg(feature = "ml_target_full")]
    compile_error!("Inference with ml_target_full is not supported; use ml_target_folded or ml_target_folded_bass.");
    #[cfg(feature = "ml_target_folded_bass")]
    let note_offset = PITCH_CLASS_COUNT;
    #[cfg(feature = "ml_target_folded")]
    let note_offset = 0;

    let mut pitches = Vec::new();
    let mut pitch_deltas = [0.0f32; 12];

    for pitch_class_index in 0..PITCH_CLASS_COUNT {
        let idx = note_offset + pitch_class_index;
        let is_present = inferred.get(idx).copied().unwrap_or(0.0);

        // Calculate delta (probability - threshold) for debugging.
        let probability = probabilities.get(idx).copied().unwrap_or(0.0);
        let threshold = state.thresholds.get(idx).copied().unwrap_or(0.5);
        pitch_deltas[pitch_class_index] = probability - threshold;

        if is_present == 1.0 {
            let pitch = Pitch::try_from(pitch_class_index as u8).map_err(|e| anyhow::Error::msg(format!("Invalid pitch class {}: {}", pitch_class_index, e)))?;
            pitches.push(pitch);
        }
    }

    // Generate chord candidates using smart octave permutations.
    // If there are no pitches detected, return empty chord list.
    // If chord detection fails, log it but still return the pitches.
    let chords = if pitches.is_empty() {
        vec![]
    } else {
        #[allow(unused_variables)]
        Chord::try_from_pitches(&pitches).unwrap_or_else(|e| {
            #[cfg(feature = "cli")]
            eprintln!("Could not determine chords from pitches: {}", e);
            vec![]
        })
    };

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

        // The folded model predicts pitch classes directly (no octave information).
        // We expect a C7-family chord from the test audio.
        assert!(!inference_result.pitches.is_empty(), "expected at least one pitch class");
        assert!(!inference_result.chords.is_empty(), "expected at least one chord candidate");

        let name = inference_result.chords[0].name_ascii();
        assert!(name.starts_with("C7") || name.starts_with("C/C 7"), "expected a C7 chord variant, got: {name}");
    }
}
