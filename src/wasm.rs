//! The WASM module.
//! 
//! This module contains the WASM wrappers / bindings for the rest of the library.

use anyhow::Context;

use wasm_bindgen::prelude::*;

use crate::core::{note::Note, base::{Parsable, Res, HasName, HasDescription, HasStaticName, HasPreciseName}, chord::{Chord, HasChord, HasRoot, HasScale, HasSlash, HasInversion, HasIsCrunchy}, pitch::{HasFrequency}, named_pitch::HasNamedPitch, octave::HasOctave};

// [`Note`] ABI.

/// The [`Note`] wrapper.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KordNote {
    note: Note,
}

/// The [`Note`] impl.
#[wasm_bindgen]
impl KordNote {
    /// Creates a new [`Note`] from a frequency.
    #[wasm_bindgen]
    pub fn parse(name: String) -> Result<KordNote, JsValue> {
        Ok(Self {
            note: Note::parse(&name).to_js_error()?
        })
    }

    /// Returns [`Note`]s from audio data.
    #[cfg(feature = "analyze_base")]
    #[wasm_bindgen]
    pub fn from_audio(data: &[f32], length_in_seconds: u8) -> Result<KordNotes, JsValue> {
        let notes = Note::try_from_audio(data, length_in_seconds).to_js_error()?;

        Ok(notes.into())
    }

    /// Returns [`Note`]s from audio data using the ML inference algorithm.
    #[cfg(all(feature = "ml_infer", feature = "analyze_base"))]
    #[wasm_bindgen]
    pub fn from_audio_ml(data: &[f32], length_in_seconds: u8) -> Result<KordNotes, JsValue> {
        let notes = Note::try_from_audio_ml(data, length_in_seconds).to_js_error()?;

        Ok(notes.into())
    }

    /// Returns the [`Note`]'s friendly name.
    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.note.name()
    }

    /// Returns the [`Note`]'s [`NamedPitch`].
    #[wasm_bindgen]
    pub fn pitch(&self) -> String {
        self.note.named_pitch().static_name().to_string()
    }

    /// Returns the [`Note`]'s [`Octave`].
    #[wasm_bindgen]
    pub fn octave(&self) -> u8 {
        self.note.octave() as u8
    }

    /// Returns the [`Note`]'s frequency.
    #[wasm_bindgen]
    pub fn frequency(&self) -> f32 {
        self.note.frequency()
    }
}

// [`Note`]s ABI.

/// The [`Note`]s wrapper.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KordNotes {
    notes: Vec<KordNote>,
}

/// The [`Note`]s impl.
#[wasm_bindgen]
impl KordNotes {
    /// Gets an item from the collection.
    #[wasm_bindgen(js_name = get)]
    pub fn get(&self, index: usize) -> Option<KordNote> {
        self.notes.get(index).cloned()
    }

    /// Returns the number of items in the collection.
    #[wasm_bindgen(getter, js_name = length)]
    pub fn length(&self) -> usize {
        self.notes.len()
    }

    /// Returns the [`Note`]s as a space-separated string.
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        self.notes.iter().map(|note| note.note.name()).collect::<Vec<_>>().join(" ")
    }
}

impl From<Vec<Note>> for KordNotes {
    fn from(notes: Vec<Note>) -> Self {
        Self {
            notes: notes.into_iter().map(|note| KordNote { note }).collect()
        }
    }
}

// [`Chord`] ABI.

/// The [`Chord`] wrapper.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KordChord {
    chord: Chord,
}

/// The [`Chord`] impl.
#[wasm_bindgen]
impl KordChord {
    /// Creates a new [`Chord`] from a frequency.
    #[wasm_bindgen]
    pub fn parse(name: String) -> Result<KordChord, JsValue> {
        Ok(Self {
            chord: Chord::parse(&name).to_js_error()?
        })
    }

    /// Creates a new [`Chord`] from a set of [`Note`]s.
    /// 
    /// The [`Note`]s should be represented as a space-separated string.
    /// E.g., `C E G`.
    #[wasm_bindgen]
    pub fn from_notes_string(notes: String) -> Result<KordChords, JsValue> {
        let notes = notes.split_whitespace()
            .map(|note| Note::parse(note).to_js_error())
            .collect::<Result<Vec<Note>, JsValue>>()?;

        let candidates = Chord::try_from_notes(&notes).to_js_error()?;

        Ok(candidates.into())
    }

    /// Creates a new [`Chord`] from a set of [`Note`]s.
    #[wasm_bindgen]
    pub fn from_notes(notes: KordNotes) -> Result<KordChords, JsValue> {
        let notes = notes.notes.into_iter()
            .map(|note| note.note)
            .collect::<Vec<Note>>();

        let candidates = Chord::try_from_notes(&notes).to_js_error()?;

        Ok(candidates.into())
    }

    /// Returns the [`Chord`]'s friendly name.
    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.chord.name()
    }

    /// Returns the [`Chord`]'s precise name.
    #[wasm_bindgen]
    pub fn precise_name(&self) -> String {
        self.chord.precise_name()
    }

    /// Returns the [`Chord`]'s description.
    #[wasm_bindgen]
    pub fn description(&self) -> String {
        self.chord.description().to_string()
    }

    /// Returns the [`Chord`]'s display text.
    #[wasm_bindgen]
    pub fn display(&self) -> String {
        self.chord.to_string()
    }

    /// Returns the [`Chord`]'s root note.
    #[wasm_bindgen]
    pub fn root(&self) -> String {
        self.chord.root().name()
    }

    /// Returns the [`Chord`]'s slash note.
    #[wasm_bindgen]
    pub fn slash(&self) -> String {
        self.chord.slash().name()
    }

    /// Returns the [`Chord`]'s inversion.
    #[wasm_bindgen]
    pub fn inversion(&self) -> u8 {
        self.chord.inversion()
    }

    /// Returns whether or not the [`Chord`] is "crunchy".
    #[wasm_bindgen]
    pub fn is_crunchy(&self) -> bool {
        self.chord.is_crunchy()
    }

    /// Returns the [`Chord`]'s chord tones.
    #[wasm_bindgen]
    pub fn chord(&self) -> KordNotes {
        KordNotes::from(self.chord.chord())
    }

    /// Returns the [`Chord`]'s chord tones as a string.
    #[wasm_bindgen]
    pub fn chord_string(&self) -> String {
        self.chord.chord().iter().map(|n| n.name()).collect::<Vec<_>>().join(" ")
    }

    /// Returns the [`Chord`]'s scale tones.
    #[wasm_bindgen]
    pub fn scale(&self) -> KordNotes {
        KordNotes::from(self.chord.scale())
    }

    /// Returns the [`Chord`]'s scale tones as a string.
    #[wasm_bindgen]
    pub fn scale_string(&self) -> String {
        self.chord.scale().iter().map(|n| n.name()).collect::<Vec<_>>().join(" ")
    }

    /// Plays the [`Chord`].
    #[wasm_bindgen]
    #[cfg(feature = "audio")]
    pub async fn play(&self, delay: f32, length: f32, fade_in: f32) -> Result<(), JsValue> {
        use futures_timer::Delay;
        use std::time::Duration;
        use crate::core::base::Playable;

        let _handle = self.chord.play(delay, length, fade_in).context("Could not start the playback.").to_js_error()?;

        Delay::new(Duration::from_secs_f32(length)).await;

        Ok(())
    }
}

// [`Chord`]s ABI.

/// The [`Chord`]s wrapper.
#[wasm_bindgen]
pub struct KordChords {
    chords: Vec<KordChord>,
}

/// The [`Chord`]s impl.
#[wasm_bindgen]
impl KordChords {
    /// Gets an item from the collection.
    #[wasm_bindgen(js_name = get)]
    pub fn get(&self, index: usize) -> Option<KordChord> {
        self.chords.get(index).cloned()
    }

    /// Returns the number of items in the collection.
    #[wasm_bindgen(getter, js_name = length)]
    pub fn length(&self) -> usize {
        self.chords.len()
    }

    /// Returns the [`Chord`]s as a space-separated string.
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        self.chords.iter().map(|chord| chord.chord.to_string()).collect::<Vec<_>>().join(" ")
    }
}

impl From<Vec<Chord>> for KordChords {
    fn from(chords: Vec<Chord>) -> Self {
        Self {
            chords: chords.into_iter().map(|chord| KordChord { chord }).collect()
        }
    }
}

// Helpers.

/// Helper trait for converting errors to [`JsValue`]s.
trait ToJsError<T> {
    /// Converts the error to a [`JsValue`].
    fn to_js_error(self) -> Result<T, JsValue>;
}

impl<T> ToJsError<T> for Res<T> {
    fn to_js_error(self) -> Result<T, JsValue> {
        self.map_err(|e| JsValue::from_str(&e.to_string()))
    }
}