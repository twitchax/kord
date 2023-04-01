//! Analyzes audio data from the microphone.

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Context;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo,
};

use crate::core::{base::Res, note::Note};

use super::base::get_notes_from_audio_data;

#[no_coverage]
pub async fn get_notes_from_microphone(length_in_seconds: u8) -> Res<Vec<Note>> {
    // Get data.

    let data_from_microphone = get_audio_data_from_microphone(length_in_seconds).await?;

    // Get notes.

    let result = get_notes_from_audio_data(&data_from_microphone, length_in_seconds)?;

    Ok(result)
}

/// Gets audio data from the microphone.
#[no_coverage]
pub async fn get_audio_data_from_microphone(length_in_seconds: u8) -> Res<Vec<f32>> {
    if length_in_seconds < 1 {
        return Err(anyhow::Error::msg("Listening length in seconds must be greater than 1."));
    }

    // Set up devices and systems.

    let (device, config) = get_device_and_config()?;

    // Record audio from the microphone.

    let data_from_microphone = record_from_device(device, config, length_in_seconds).await?;

    Ok(data_from_microphone)
}

/// Gets the system device, and config.
#[no_coverage]
fn get_device_and_config() -> Res<(cpal::Device, cpal::SupportedStreamConfig)> {
    let host = cpal::default_host();

    let device = host.default_input_device().ok_or_else(|| anyhow::Error::msg("Failed to get default input device."))?;

    let config = device.default_input_config().context("Could not get default input config.")?;

    Ok((device, config))
}

/// Records audio data from the device.
#[no_coverage]
async fn record_from_device(device: cpal::Device, config: cpal::SupportedStreamConfig, length_in_seconds: u8) -> Res<Vec<f32>> {
    // Set up recording.

    let likely_sample_count = config.sample_rate().0 as f32 * config.channels() as f32 * length_in_seconds as f32;

    let data_from_microphone = Arc::new(Mutex::new(Vec::with_capacity(likely_sample_count as usize)));
    let last_error = Arc::new(Mutex::new(None));

    let stream = {
        let result = data_from_microphone.clone();
        let last_error = last_error.clone();

        device.build_input_stream::<f32, _, _>(
            &config.into(),
            move |data: &[_], _: &InputCallbackInfo| {
                result.lock().unwrap().extend_from_slice(data);
            },
            move |err| {
                last_error.lock().unwrap().replace(err);
            },
            None,
        )?
    };

    // Begin recording.

    stream.play()?;
    futures_timer::Delay::new(Duration::from_secs_f32(length_in_seconds as f32)).await;
    drop(stream);

    // SAFETY: We are the only thread that can access the arc right now since the stream is dropped.
    if let Err(err) = Arc::try_unwrap(last_error).unwrap().into_inner() {
        return Err(err.into());
    }

    // SAFETY: We are the only thread that can access the arc right now since the stream is dropped.
    let data_from_microphone = Arc::try_unwrap(data_from_microphone).unwrap().into_inner()?;

    Ok(data_from_microphone)
}

// Tests.

#[cfg(test)]
mod tests {
    use crate::core::{base::Parsable, chord::Chord, note::Note};

    #[test]
    fn test_mic() {
        let data = crate::analyze::base::tests::load_test_data();

        let notes = Note::try_from_audio(&data, 5).unwrap();

        let chord = Chord::try_from_notes(&notes).unwrap();

        assert_eq!(chord[0], Chord::parse("C7b9").unwrap());
    }
}
