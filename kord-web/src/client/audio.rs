use std::time::Duration;

use klib::analyze::base::{get_frequency_space, get_smoothed_frequency_space};
use klib::core::{base::Playable, chord::Chord, note::Note};
use leptos::prelude::set_timeout;

/// Errors that can occur during audio processing and inference.
#[derive(Debug, Clone)]
pub enum AudioError {
    /// The audio buffer length is not a valid multiple of 4 bytes for f32 samples.
    InvalidBufferLength,
    /// An error occurred during ML inference.
    InferenceError(String),
    /// An error occurred while generating chords from pitches.
    ChordGenerationError(String),
}

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioError::InvalidBufferLength => write!(f, "invalid audio buffer length (must be multiple of 4)"),
            AudioError::InferenceError(msg) => write!(f, "inference failed: {}", msg),
            AudioError::ChordGenerationError(msg) => write!(f, "chord generation failed: {}", msg),
        }
    }
}

impl std::error::Error for AudioError {}

/// Convert little-endian f32 PCM bytes to samples.
pub fn le_bytes_to_f32_samples(bytes: &[u8]) -> Result<Vec<f32>, AudioError> {
    if !bytes.len().is_multiple_of(4) {
        return Err(AudioError::InvalidBufferLength);
    }

    let mut samples = Vec::with_capacity(bytes.len() / 4);
    for chunk in bytes.chunks_exact(4) {
        samples.push(f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
    }

    Ok(samples)
}

/// Run ML inference and return the full inference result.
pub fn infer_from_samples(samples: &[f32], secs: u8) -> Result<klib::ml::infer::InferenceResult, AudioError> {
    klib::ml::infer::infer(samples, secs).map_err(|e| AudioError::InferenceError(e.to_string()))
}

/// Get notes from pitches for display.
pub fn pitches_to_notes(pitches: &[klib::core::pitch::Pitch]) -> Vec<Note> {
    pitches
        .iter()
        .map(|&pitch| Note::new(klib::core::named_pitch::NamedPitch::from(pitch), klib::core::octave::Octave::Four))
        .collect()
}

/// Generate chord candidates from pitch classes.
pub fn chords_from_pitches(pitches: &[klib::core::pitch::Pitch]) -> Result<Vec<Chord>, AudioError> {
    if pitches.is_empty() {
        return Ok(vec![]);
    }

    let mut chords = Chord::try_from_pitches(pitches).map_err(|e| AudioError::ChordGenerationError(e.to_string()))?;
    chords.truncate(8);
    Ok(chords)
}

/// Run ML inference and derive up to 8 chord candidates from samples.
pub fn infer_chords_from_samples(samples: &[f32], secs: u8) -> Result<Vec<Chord>, AudioError> {
    let result = infer_from_samples(samples, secs)?;
    Ok(result.chords)
}

/// Compute the smoothed frequency space for visualization.
///
/// Returns a vector of (frequency, magnitude) pairs representing the FFT result
/// normalized to 1 second of playback. The result is limited to the first `max_bins`
/// entries for efficient rendering.
pub fn compute_frequency_space(samples: &[f32], length_in_seconds: u8, max_bins: usize) -> Vec<(f32, f32)> {
    let frequency_space = get_frequency_space(samples, length_in_seconds);
    let smoothed = get_smoothed_frequency_space(&frequency_space, length_in_seconds);

    // Take only up to max_bins for rendering efficiency
    smoothed.into_iter().take(max_bins).collect()
}

/// Play a chord for the specified duration in seconds.
pub fn play_chord(chord: &Chord, duration_secs: f64) {
    let delay = Duration::from_secs_f64(0.2);
    let length = Duration::from_secs_f64(duration_secs);
    let fade_in = Duration::from_secs_f64(0.2);

    let handle = chord.play(delay, length, fade_in).unwrap();

    set_timeout(
        move || {
            drop(handle);
        },
        length,
    );
}

/// Play a single note for the specified duration in seconds.
pub fn play_note(note: &Note, duration_secs: f64) {
    let delay = Duration::from_secs_f64(0.0);
    let length = Duration::from_secs_f64(duration_secs);
    let fade_in = Duration::from_secs_f64(0.2);

    let handle = note.play(delay, length, fade_in).unwrap();

    set_timeout(
        move || {
            drop(handle);
        },
        length,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_le_bytes_to_f32_samples_valid() {
        let bytes = vec![0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x40];
        let result = le_bytes_to_f32_samples(&bytes);
        assert!(result.is_ok());
        let samples = result.unwrap();
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0], 1.0);
        assert_eq!(samples[1], 2.0);
    }

    #[test]
    fn test_le_bytes_to_f32_samples_invalid_length() {
        let bytes = vec![0x00, 0x00, 0x80];
        let result = le_bytes_to_f32_samples(&bytes);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AudioError::InvalidBufferLength));
    }

    #[test]
    fn test_le_bytes_to_f32_samples_empty() {
        let bytes = vec![];
        let result = le_bytes_to_f32_samples(&bytes);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_pitches_to_notes() {
        use klib::core::pitch::Pitch;
        let pitches = vec![Pitch::C, Pitch::E, Pitch::G];
        let notes = pitches_to_notes(&pitches);
        assert_eq!(notes.len(), 3);
    }

    #[test]
    fn test_pitches_to_notes_empty() {
        let pitches = vec![];
        let notes = pitches_to_notes(&pitches);
        assert_eq!(notes.len(), 0);
    }

    #[test]
    fn test_chords_from_pitches_empty() {
        let pitches = vec![];
        let result = chords_from_pitches(&pitches);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_chords_from_pitches_valid() {
        use klib::core::pitch::Pitch;
        let pitches = vec![Pitch::C, Pitch::E, Pitch::G];
        let result = chords_from_pitches(&pitches);
        assert!(result.is_ok());
        let chords = result.unwrap();
        assert!(!chords.is_empty());
        assert!(chords.len() <= 8);
    }

    #[test]
    fn test_audio_error_display() {
        let err = AudioError::InvalidBufferLength;
        assert!(err.to_string().contains("multiple of 4"));

        let err = AudioError::InferenceError("test error".to_string());
        assert!(err.to_string().contains("inference failed"));
        assert!(err.to_string().contains("test error"));

        let err = AudioError::ChordGenerationError("test error".to_string());
        assert!(err.to_string().contains("chord generation failed"));
        assert!(err.to_string().contains("test error"));
    }
}
