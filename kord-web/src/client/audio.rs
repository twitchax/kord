use std::time::Duration;

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
