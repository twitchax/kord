//! Module for executing inference.

use burn::{
    config::Config,
    module::{Module, State},
    tensor::backend::Backend,
};
use burn_ndarray::{NdArrayBackend, NdArrayDevice};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    analyze::base::{get_frequency_space, get_smoothed_frequency_space},
    core::{
        base::Res,
        note::{HasNoteId, Note},
    },
    ml::base::{data::kord_item_to_sample_tensor, helpers::binary_to_u128, model::KordModel, KordItem, TrainConfig, FREQUENCY_SPACE_SIZE},
};

pub fn run_inference<B: Backend>(device: &B::Device, kord_item: &KordItem) -> Res<Vec<Note>>
where
    B::Elem: Serialize + DeserializeOwned,
{
    // Load the config and state.

    // [TODO] Read this from within the binary itself.

    let config = TrainConfig::load_binary(CONFIG)?;
    let state = State::<B::Elem>::load_binary(STATE)?;

    // Define the model.
    let mut model = KordModel::<B>::new(config.mlp_layers, config.mlp_size, config.mlp_dropout, config.sigmoid_strength);
    model = model.load(&state)?;

    // Prepare the sample.
    let sample = kord_item_to_sample_tensor(kord_item).to_device(device).detach();

    // Run the inference.
    let inferred = model.forward(sample).to_data().convert().value.into_iter().map(f32::round).collect::<Vec<_>>();
    let inferred_array: [_; 128] = inferred.try_into().unwrap();
    let mut inferred_notes = Note::from_id_mask(binary_to_u128(&inferred_array)).unwrap();
    inferred_notes.sort();

    Ok(inferred_notes)
}

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
    let notes = run_inference::<NdArrayBackend<f32>>(&device, &kord_item)?;

    Ok(notes)
}

// Statics.
#[cfg(host_family_unix)]
static CONFIG: &[u8] = include_bytes!("../../../model/model_config.json");
#[cfg(host_family_unix)]
static STATE: &[u8] = include_bytes!("../../../model/state.json.gz");

#[cfg(host_family_windows)]
static CONFIG: &[u8] = include_bytes!("..\\..\\..\\model\\model_config.json");
#[cfg(host_family_windows)]
static STATE: &[u8] = include_bytes!("..\\..\\..\\model\\state.json.gz");


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

        let chord = Chord::from_notes(&notes).unwrap();

        assert_eq!(chord[0], Chord::parse("C7b9").unwrap());
    }
}
