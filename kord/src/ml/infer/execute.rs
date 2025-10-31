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
    core::{
        base::Res,
        note::{HasNoteId, Note},
    },
    ml::base::{
        data::kord_item_to_sample_tensor,
        helpers::{binary_to_u128, logits_to_predictions, logits_to_probabilities},
        model::KordModel,
        KordItem, StorePrecisionSettings, TrainConfig, FREQUENCY_SPACE_SIZE, NOTE_SIGNATURE_SIZE, NUM_CLASSES,
    },
};

/// Run the inference on a sample to produce a [`Vec`] of [`Note`]s.
pub fn run_inference<B: Backend>(device: &B::Device, kord_item: &KordItem) -> Res<Vec<Note>>
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

    // TODO: remove this when inference uses just folded bass and folded targets (which is the direction we seem to be going).
    if NUM_CLASSES < NOTE_SIGNATURE_SIZE {
        return Err(anyhow::Error::msg(
            "Inference requires a target space with at least 128 classes; enable `ml_target_full` when building the inference binary.",
        ));
    }

    // Define the model.
    let model = KordModel::<B>::new(device, config.mha_heads, config.mha_dropout, config.sigmoid_strength).load_record(recorder);

    // Prepare the sample.
    let sample = kord_item_to_sample_tensor(device, kord_item).detach();

    // Run the inference.
    let logits = model.forward(sample).detach();
    let logits_vec: Vec<f32> = logits.into_data().convert::<f32>().to_vec().unwrap_or_default();
    let probabilities = logits_to_probabilities(&logits_vec);
    let thresholds: Vec<f32> = serde_json::from_slice(THRESHOLDS_JSON).unwrap_or_default();
    let inferred = logits_to_predictions(&probabilities, thresholds.as_slice());

    let inferred_array: [_; 128] = inferred.iter().cloned().take(128).collect::<Vec<_>>().try_into().unwrap();
    let mut inferred_notes = Note::from_id_mask(binary_to_u128(&inferred_array)).unwrap();
    inferred_notes.sort();

    Ok(inferred_notes)
}

/// Infer notes from the audio data.
pub fn infer(audio_data: &[f32], length_in_seconds: u8) -> Res<Vec<Note>> {
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

    // Run the inference.
    let notes = run_inference::<NdArray<f32>>(&device, &kord_item)?;

    Ok(notes)
}

// Statics.
#[cfg(host_family_unix)]
static CONFIG: &[u8] = include_bytes!("../../../model/model_config.json");
#[cfg(host_family_unix)]
static STATE_BINCODE: &[u8] = include_bytes!("../../../model/state.json.bin");
#[cfg(host_family_unix)]
static THRESHOLDS_JSON: &[u8] = include_bytes!("../../../model/thresholds.json");

#[cfg(host_family_windows)]
static CONFIG: &[u8] = include_bytes!("..\\..\\..\\model\\model_config.json");
#[cfg(host_family_windows)]
static STATE_BINCODE: &[u8] = include_bytes!("..\\..\\..\\model\\state.json.bin");
#[cfg(host_family_windows)]
static THRESHOLDS_JSON: &[u8] = include_bytes!("..\\..\\..\\model\\thresholds.json");

// Tests.

#[cfg(test)]
#[cfg(feature = "ml_infer")]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;
    use crate::core::{base::Parsable, chord::Chord};

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

        let notes = infer(&audio_data, 5).unwrap();

        let chord = Chord::try_from_notes(&notes).unwrap();

        assert_eq!(chord[0], Chord::parse("C7b9").unwrap());
    }
}
