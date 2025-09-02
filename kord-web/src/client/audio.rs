use std::time::Duration;

use klib::core::{base::Playable, chord::Chord, note::Note};
use leptos::prelude::set_timeout;

/// Convert little-endian f32 PCM bytes to samples.
pub fn le_bytes_to_f32_samples(bytes: &[u8]) -> Result<Vec<f32>, &'static str> {
    if !bytes.len().is_multiple_of(4) {
        return Err("invalid audio buffer length");
    }

    let mut samples = Vec::with_capacity(bytes.len() / 4);
    for chunk in bytes.chunks_exact(4) {
        samples.push(f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
    }

    Ok(samples)
}

/// Run ML inference and derive up to 8 chord candidates from samples.
pub fn infer_chords_from_samples(samples: &[f32], secs: u8) -> Result<Vec<Chord>, String> {
    let notes: Vec<Note> = klib::ml::infer::infer(samples, secs).map_err(|e| e.to_string())?;
    let mut candidates = Chord::try_from_notes(&notes).map_err(|e| e.to_string())?;

    candidates.truncate(8);

    Ok(candidates)
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
