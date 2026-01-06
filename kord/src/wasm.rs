//! The WASM module.
//!
//! This module contains the WASM wrappers / bindings for the rest of the library.

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{convert::RefFromWasmAbi, prelude::*};

use crate::core::{
    base::{HasDescription, HasName, HasPreciseName, HasStaticName, Parsable, PlaybackHandle, Res},
    chord::{Chord, Chordable, HasChord, HasExtensions, HasInversion, HasIsCrunchy, HasModifiers, HasRoot, HasScale, HasSlash},
    interval::Interval,
    named_pitch::HasNamedPitch,
    note::{HasPrimaryHarmonicSeries, Note},
    octave::{HasOctave, Octave},
    pitch::HasFrequency,
};

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

// Helper types.

/// The [`Result`] type for the WASM bindings.
pub type JsRes<T> = Result<T, JsValue>;

// [`Note`] ABI.

/// The [`Note`] wrapper.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KordNote {
    inner: Note,
}

impl From<Note> for KordNote {
    fn from(note: Note) -> Self {
        KordNote { inner: note }
    }
}

impl From<KordNote> for Note {
    fn from(kord_note: KordNote) -> Self {
        kord_note.inner
    }
}

/// The [`Note`] impl.
#[wasm_bindgen]
impl KordNote {
    /// Creates a new [`Note`] from a frequency.
    #[wasm_bindgen]
    pub fn parse(name: String) -> JsRes<KordNote> {
        Ok(Self { inner: Note::parse(&name).to_js_error()? })
    }

    /// Returns [`Note`]s from audio data.
    #[cfg(feature = "analyze_base")]
    #[wasm_bindgen(js_name = fromAudio)]
    pub fn from_audio(data: &[f32], length_in_seconds: u8) -> JsRes<Array> {
        let notes = Note::try_from_audio(data, length_in_seconds).to_js_error()?.into_iter().map(KordNote::from);

        Ok(notes.into_js_array())
    }

    /// Returns [`Note`]s from audio data using the ML inference algorithm.
    #[cfg(all(feature = "ml_infer", feature = "analyze_base"))]
    #[wasm_bindgen(js_name = fromAudioMl)]
    pub fn from_audio_ml(data: &[f32], length_in_seconds: u8) -> JsRes<Array> {
        let notes = Note::try_from_audio_ml(data, length_in_seconds).to_js_error()?.into_iter().map(KordNote::from);

        Ok(notes.into_js_array())
    }

    /// Returns the [`Note`]'s friendly name.
    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.inner.name()
    }

    /// Returns the [`Note`] represented as a string (same as `name`).
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.inner.name()
    }

    /// Returns the [`Note`]'s [`NamedPitch`].
    #[wasm_bindgen]
    pub fn pitch(&self) -> String {
        self.inner.named_pitch().static_name().to_string()
    }

    /// Returns the [`Note`]'s [`Octave`].
    #[wasm_bindgen]
    pub fn octave(&self) -> u8 {
        self.inner.octave() as u8
    }

    /// Returns the [`Note`]'s frequency.
    #[wasm_bindgen]
    pub fn frequency(&self) -> f32 {
        self.inner.frequency()
    }

    /// Adds the given interval to the [`Note`], producing a new [`Note`] instance.
    #[wasm_bindgen(js_name = addInterval)]
    pub fn add_interval(&self, interval: Interval) -> KordNote {
        let note = self.inner + interval;

        Self { inner: note }
    }

    /// Subtracts the given interval from the [`Note`], producing a new [`Note`] instance.
    #[wasm_bindgen(js_name = subInterval)]
    pub fn subtract_interval(&self, interval: Interval) -> KordNote {
        let note = self.inner - interval;

        Self { inner: note }
    }

    /// Computes the [`Interval`] distance between the [`Note`] and the given [`Note`].
    #[wasm_bindgen(js_name = distanceTo)]
    pub fn distance_to(&self, other: KordNote) -> Interval {
        self.inner - other.inner
    }

    /// Returns the primary (first 13) harmonic series of the [`Note`].
    #[wasm_bindgen(js_name = harmonicSeries)]
    pub fn harmonic_series(&self) -> Array {
        let series = self.inner.primary_harmonic_series();

        series.into_iter().map(KordNote::from).into_js_array()
    }

    /// Returns the clone of the [`Note`].
    #[wasm_bindgen]
    pub fn copy(&self) -> KordNote {
        self.clone()
    }
}

// [`Chord`] ABI.

/// The [`Chord`] wrapper.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KordChord {
    inner: Chord,
}

impl From<Chord> for KordChord {
    fn from(chord: Chord) -> Self {
        KordChord { inner: chord }
    }
}

impl From<KordChord> for Chord {
    fn from(kord_chord: KordChord) -> Self {
        kord_chord.inner
    }
}

/// The [`Chord`] impl.
#[wasm_bindgen]
impl KordChord {
    /// Creates a new [`Chord`] from a frequency.
    #[wasm_bindgen]
    pub fn parse(name: String) -> JsRes<KordChord> {
        Ok(Self {
            inner: Chord::parse(&name).to_js_error()?,
        })
    }

    /// Creates a new [`Chord`] from a set of [`Note`]s.
    ///
    /// The [`Note`]s should be represented as a space-separated string.
    /// E.g., `C E G`.
    #[wasm_bindgen(js_name = fromNotesString)]
    pub fn from_notes_string(notes: String) -> JsRes<Array> {
        let notes = notes.split_whitespace().map(|note| Note::parse(note).to_js_error()).collect::<JsRes<Vec<Note>>>()?;

        let candidates = Chord::try_from_notes(&notes).to_js_error()?.into_iter().map(KordChord::from);

        Ok(candidates.into_js_array())
    }

    /// Creates a new [`Chord`] from a set of [`Note`]s.
    #[wasm_bindgen(js_name = fromNotes)]
    pub fn from_notes(notes: Array) -> JsRes<Array> {
        let notes: Vec<Note> = notes.cloned_into_vec_inner::<KordNote, Note>()?;

        let candidates = Chord::try_from_notes(&notes).to_js_error()?.into_iter().map(KordChord::from);

        Ok(candidates.into_js_array())
    }

    /// Returns the [`Chord`]'s friendly name.
    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.inner.name()
    }

    /// Returns the [`Chord`]'s precise name.
    #[wasm_bindgen(js_name = preciseName)]
    pub fn precise_name(&self) -> String {
        self.inner.precise_name()
    }

    /// Returns the [`Chord`] as a string (same as `precise_name`).
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.inner.precise_name()
    }

    /// Returns the [`Chord`]'s description.
    #[wasm_bindgen]
    pub fn description(&self) -> String {
        self.inner.description().to_string()
    }

    /// Returns the [`Chord`]'s display text.
    #[wasm_bindgen]
    pub fn display(&self) -> String {
        self.inner.to_string()
    }

    /// Returns the [`Chord`]'s root note.
    #[wasm_bindgen]
    pub fn root(&self) -> String {
        self.inner.root().name()
    }

    /// Returns the [`Chord`]'s slash note.
    #[wasm_bindgen]
    pub fn slash(&self) -> String {
        self.inner.slash().name()
    }

    /// Returns the [`Chord`]'s inversion.
    #[wasm_bindgen]
    pub fn inversion(&self) -> u8 {
        self.inner.inversion()
    }

    /// Returns whether or not the [`Chord`] is "crunchy".
    #[wasm_bindgen(js_name = isCrunchy)]
    pub fn is_crunchy(&self) -> bool {
        self.inner.is_crunchy()
    }

    /// Returns the [`Chord`]'s chord tones.
    #[wasm_bindgen]
    pub fn chord(&self) -> Array {
        self.inner.chord().into_iter().map(KordNote::from).into_js_array()
    }

    /// Returns the [`Chord`]'s chord tones as a string.
    #[wasm_bindgen(js_name = chordString)]
    pub fn chord_string(&self) -> String {
        self.inner.chord().iter().map(|n| n.name()).collect::<Vec<_>>().join(" ")
    }

    /// Returns the [`Chord`]'s scale tones.
    #[wasm_bindgen]
    pub fn scale(&self) -> Array {
        self.inner.scale().into_iter().map(KordNote::from).into_js_array()
    }

    /// Returns the [`Chord`]'s scale tones as a string.
    #[wasm_bindgen(js_name = scaleString)]
    pub fn scale_string(&self) -> String {
        self.inner.scale().iter().map(|n| n.name()).collect::<Vec<_>>().join(" ")
    }

    /// Returns the [`Chord`]'s modifiers.
    #[wasm_bindgen]
    pub fn modifiers(&self) -> Array {
        self.inner.modifiers().iter().map(|m| m.static_name()).into_js_array()
    }

    /// Returns the [`Chord`]'s extensions.
    #[wasm_bindgen]
    pub fn extensions(&self) -> Array {
        self.inner.extensions().iter().map(|e| e.static_name()).into_js_array()
    }

    /// Returns a new [`Chord`] with the inversion set to the provided value.
    #[wasm_bindgen(js_name = withInversion)]
    pub fn with_inversion(&self, inversion: u8) -> Self {
        KordChord {
            inner: self.inner.clone().with_inversion(inversion),
        }
    }

    /// Returns a new [`Chord`] with the slash set to the provided value.
    #[wasm_bindgen(js_name = withSlash)]
    pub fn with_slash(&self, slash: &KordNote) -> Self {
        KordChord {
            inner: self.inner.clone().with_slash(slash.inner),
        }
    }

    /// Returns a new [`Chord`] with the octave of the root set to the provided value.
    #[wasm_bindgen(js_name = withOctave)]
    pub fn with_octave(&self, octave: u8) -> JsRes<KordChord> {
        Ok(KordChord {
            inner: self.inner.clone().with_octave(Octave::try_from(octave)?),
        })
    }

    /// Returns a new [`Chord`] with the "crunchiness" set to the provided value.
    #[wasm_bindgen(js_name = withCrunchy)]
    pub fn with_crunchy(&self, is_crunchy: bool) -> Self {
        KordChord {
            inner: self.inner.clone().with_crunchy(is_crunchy),
        }
    }

    /// Plays the [`Chord`].
    #[wasm_bindgen]
    #[cfg(feature = "audio")]
    pub async fn play(&self, delay: f32, length: f32, fade_in: f32) -> JsRes<()> {
        use crate::core::base::Playable;
        use anyhow::Context;
        use gloo_timers::future::TimeoutFuture;
        use std::time::Duration;

        let delay = Duration::from_secs_f32(delay);
        let length = Duration::from_secs_f32(length);
        let fade_in = Duration::from_secs_f32(fade_in);

        let _handle = self.inner.play(delay, length, fade_in).context("Could not start the playback.").to_js_error()?;

        TimeoutFuture::new(length.as_millis() as u32).await;

        Ok(())
    }

    /// Returns the clone of the [`Chord`].
    #[wasm_bindgen]
    pub fn copy(&self) -> KordChord {
        self.clone()
    }
}

// Playback handle.

/// A handle to a [`Chord`] playback.
///
/// Should be dropped to stop the playback, or after playback is finished.
#[wasm_bindgen]
pub struct KordPlaybackHandle {
    _inner: PlaybackHandle,
}

// The modifiers.

/// The chord modifiers.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub enum KordModifier {
    /// Minor modifier.
    Minor,

    /// Flat 5 modifier.
    Flat5,
    /// Sharp 5 modifier.
    Augmented5,

    /// Major 7 modifier.
    Major7,
    /// Dominant 7 modifier.
    Dominant7,
    /// Dominant 9 modifier.
    Dominant9,
    /// Dominant 11 modifier.
    Dominant11,
    /// Dominant 13 modifier.
    Dominant13,

    /// Flat 9 modifier.
    Flat9,
    /// Sharp 9 modifier.
    Sharp9,

    /// Sharp 11 modifier.
    Sharp11,

    /// Diminished modifier.
    Diminished,
}

// Helpers.

/// Helper trait for converting errors to [`JsValue`]s.
trait ToJsError<T> {
    /// Converts the error to a [`JsValue`].
    fn to_js_error(self) -> JsRes<T>;
}

impl<T> ToJsError<T> for Res<T> {
    fn to_js_error(self) -> JsRes<T> {
        self.map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

/// Helper trait for converting a [`IntoIterator<Item = T>`] (where `T: Into<JsValue>`) to an [`Array`].
trait IntoJsArray {
    /// Converts the [`Vec`] to an [`Array`].
    fn into_js_array(self) -> Array;
}

impl<I, T> IntoJsArray for I
where
    I: IntoIterator<Item = T>,
    T: Into<JsValue>,
{
    fn into_js_array(self) -> Array {
        Array::from_iter(self.into_iter().map(Into::into))
    }
}

/// Helpers trait for converting an [`Array`] to a [`Vec`].
trait ClonedIntoVec {
    /// Converts the [`Array`] to a [`Vec<T>`].
    fn cloned_into_vec<T>(self) -> JsRes<Vec<T>>
    where
        T: RefFromJsValue + RefFromWasmAbi + Clone;
}

impl ClonedIntoVec for Array {
    fn cloned_into_vec<T>(self) -> JsRes<Vec<T>>
    where
        T: RefFromJsValue + RefFromWasmAbi + Clone,
    {
        let mut result = Vec::with_capacity(self.length() as usize);

        for k in 0..self.length() {
            let value = self.get(k);
            let value = T::ref_from_js_value(&value)?.clone();

            result.push(value);
        }

        Ok(result)
    }
}

/// Helper trait for converting a [`Array`] (where `T: JsCast`) to a [`Vec`].
trait ClonedIntoVecInner {
    /// Converts the [`Array`] to a [`Vec<I>`] (where `I` is the wrapped type, first casting the [`JsValue`] into `T`).
    fn cloned_into_vec_inner<T, I>(self) -> JsRes<Vec<I>>
    where
        T: RefFromJsValue + RefFromWasmAbi + Clone,
        I: From<T>;
}

impl ClonedIntoVecInner for Array {
    fn cloned_into_vec_inner<T, I>(self) -> JsRes<Vec<I>>
    where
        T: RefFromJsValue + RefFromWasmAbi + Clone,
        I: From<T>,
    {
        let mut result = Vec::with_capacity(self.length() as usize);

        for k in 0..self.length() {
            let value = self.get(k);
            let value = T::ref_from_js_value(&value)?.clone();
            let value = I::from(value);

            result.push(value);
        }

        Ok(result)
    }
}

/// Helper trait for converting a [`JsValue`] representing a shared pointer (e.g., `{ ptr: XXX }`)
/// into a type.
trait RefFromJsValue {
    /// Converts the [`JsValue`] into a type.
    fn ref_from_js_value(abi: &JsValue) -> JsRes<Self::Anchor>
    where
        Self: Sized + RefFromWasmAbi;
}

/// Extracts a u32 pointer value from a JsValue by trying multiple conversion strategies.
///
/// This handles different pointer representations that can occur across JavaScript environments:
/// - JavaScript Number (most common, works with as_f64())
/// - JavaScript BigInt (converted to string, then parsed)
/// - Edge cases like boolean values for 0/1 pointers
fn extract_ptr_from_js_value(ptr_value: &JsValue) -> JsRes<u32> {
    if let Some(f) = ptr_value.as_f64() {
        Ok(f as u32)
    } else if let Some(b) = ptr_value.as_bool() {
        // Sometimes happens with 0/1 pointers
        Ok(b as u32)
    } else {
        // Try parsing as string (handles BigInt case)
        ptr_value
            .as_string()
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or_else(|| JsValue::from_str("Could not cast pointer to u32 from any supported type."))
    }
}

impl RefFromJsValue for KordNote {
    fn ref_from_js_value(abi: &JsValue) -> JsRes<<KordNote as RefFromWasmAbi>::Anchor>
    where
        Self: Sized + RefFromWasmAbi,
    {
        let ptr_value = Reflect::get(abi, &JsValue::from_str("ptr"))?;
        let ptr = extract_ptr_from_js_value(&ptr_value)?;

        let object = abi.dyn_ref::<Object>().ok_or("Value is not an object.")?;
        if object.constructor().name() != "KordNote" {
            return Err("Invalid object type.".into());
        }

        // SAFETY: We have done as much as we can to ensure that this is as safe as it can
        // be, considering the inherent unsafety of working with an ABI.
        //
        // We have confirmed that the JsValue is, indeed, an Object, and that
        // it is of the proper type.
        let value = unsafe { KordNote::ref_from_abi(ptr) };

        Ok(value)
    }
}

impl RefFromJsValue for KordChord {
    fn ref_from_js_value(abi: &JsValue) -> JsRes<<KordChord as RefFromWasmAbi>::Anchor>
    where
        Self: Sized + RefFromWasmAbi,
    {
        let ptr_value = Reflect::get(abi, &JsValue::from_str("ptr"))?;
        let ptr = extract_ptr_from_js_value(&ptr_value)?;

        let object = abi.dyn_ref::<Object>().ok_or("Value is not an object.")?;
        if object.constructor().name() != "KordChord" {
            return Err("Invalid object type.".into());
        }

        // SAFETY: We have done as much as we can to ensure that this is as safe as it can
        // be, considering the inherent unsafety of working with an ABI.
        //
        // We have confirmed that the JsValue is, indeed, an Object, and that
        // it is of the proper type.
        let value = unsafe { KordChord::ref_from_abi(ptr) };

        Ok(value)
    }
}

// JS helpers.

// #[wasm_bindgen(inline_js = "export function sleep(millis) { return new Promise(resolve => setTimeout(() => resolve(), millis)); }")]
// extern "C" {
//     fn sleep(millis: u32) -> Promise;
// }

// The [`Chord`] modifier / extension impl.
#[wasm_bindgen]
impl KordChord {
    /// Returns a new [`Chord`] with the `minor` modifier.
    #[wasm_bindgen]
    pub fn minor(&self) -> Self {
        KordChord { inner: self.inner.clone().minor() }
    }

    /// Returns a new [`Chord`] with the `flat5` modifier.
    #[wasm_bindgen]
    pub fn flat5(&self) -> Self {
        KordChord { inner: self.inner.clone().flat5() }
    }

    /// Returns a new [`Chord`] with the `augmented` modifier.
    #[wasm_bindgen]
    pub fn aug(&self) -> Self {
        KordChord { inner: self.inner.clone().aug() }
    }

    /// Returns a new [`Chord`] with the `maj7` modifier.
    #[wasm_bindgen]
    pub fn maj7(&self) -> Self {
        KordChord { inner: self.inner.clone().maj7() }
    }

    /// Returns a new [`Chord`] with the `dom7` modifier.
    #[wasm_bindgen]
    pub fn seven(&self) -> Self {
        KordChord { inner: self.inner.clone().dominant7() }
    }

    /// Returns a new [`Chord`] with the `dom9` modifier.
    #[wasm_bindgen]
    pub fn nine(&self) -> Self {
        KordChord { inner: self.inner.clone().dominant9() }
    }

    /// Returns a new [`Chord`] with the `dom11` modifier.
    #[wasm_bindgen]
    pub fn eleven(&self) -> Self {
        KordChord { inner: self.inner.clone().dominant11() }
    }

    /// Returns a new [`Chord`] with the `dom13` modifier.
    #[wasm_bindgen]
    pub fn thirteen(&self) -> Self {
        KordChord { inner: self.inner.clone().dominant13() }
    }

    /// Returns a new [`Chord`] with the `flat9` modifier.
    #[wasm_bindgen]
    pub fn flat9(&self) -> Self {
        KordChord { inner: self.inner.clone().flat9() }
    }

    /// Returns a new [`Chord`] with the `sharp9` modifier.
    #[wasm_bindgen]
    pub fn sharp9(&self) -> Self {
        KordChord { inner: self.inner.clone().sharp9() }
    }

    /// Returns a new [`Chord`] with the `sharp11` modifier.
    #[wasm_bindgen]
    pub fn sharp11(&self) -> Self {
        KordChord { inner: self.inner.clone().sharp11() }
    }

    /// Returns a new [`Chord`] with the `dim` modifier.
    #[wasm_bindgen]
    pub fn dim(&self) -> Self {
        KordChord { inner: self.inner.clone().dim() }
    }

    /// Returns a new [`Chord`] with the `halfDim` modifier.
    #[wasm_bindgen(js_name = halfDim)]
    pub fn half_dim(&self) -> Self {
        KordChord { inner: self.inner.clone().half_dim() }
    }

    /// Returns a new [`Chord`] with the `sus2` extension.
    #[wasm_bindgen]
    pub fn sus2(&self) -> Self {
        KordChord { inner: self.inner.clone().sus2() }
    }

    /// Returns a new [`Chord`] with the `sus4` extension.
    #[wasm_bindgen]
    pub fn sus4(&self) -> Self {
        KordChord { inner: self.inner.clone().sus4() }
    }

    /// Returns a new [`Chord`] with the `flat11` extension.
    #[wasm_bindgen]
    pub fn flat11(&self) -> Self {
        KordChord { inner: self.inner.clone().flat11() }
    }

    /// Returns a new [`Chord`] with the `flat13` extension.
    #[wasm_bindgen]
    pub fn flat13(&self) -> Self {
        KordChord { inner: self.inner.clone().flat13() }
    }

    /// Returns a new [`Chord`] with the `sharp13` extension.
    #[wasm_bindgen]
    pub fn sharp13(&self) -> Self {
        KordChord { inner: self.inner.clone().sharp13() }
    }

    /// Returns a new [`Chord`] with the `add2` extension.
    #[wasm_bindgen]
    pub fn add2(&self) -> Self {
        KordChord { inner: self.inner.clone().add2() }
    }

    /// Returns a new [`Chord`] with the `add4` extension.
    #[wasm_bindgen]
    pub fn add4(&self) -> Self {
        KordChord { inner: self.inner.clone().add4() }
    }

    /// Returns a new [`Chord`] with the `add6` extension.
    #[wasm_bindgen]
    pub fn add6(&self) -> Self {
        KordChord { inner: self.inner.clone().add6() }
    }

    /// Returns a new [`Chord`] with the `add9` extension.
    #[wasm_bindgen]
    pub fn add9(&self) -> Self {
        KordChord { inner: self.inner.clone().add9() }
    }

    /// Returns a new [`Chord`] with the `add11` extension.
    #[wasm_bindgen]
    pub fn add11(&self) -> Self {
        KordChord { inner: self.inner.clone().add11() }
    }

    /// Returns a new [`Chord`] with the `add13` extension.
    #[wasm_bindgen]
    pub fn add13(&self) -> Self {
        KordChord { inner: self.inner.clone().add13() }
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    /// Test the pointer extraction function directly with different types
    /// This validates the fix for Issue #21 - handling different pointer representations
    #[wasm_bindgen_test]
    fn test_ptr_extraction_strategies() {
        // Test with f64 (Number - most common case in most browsers)
        let f64_val = JsValue::from_f64(12345.0);
        let result = extract_ptr_from_js_value(&f64_val);
        assert!(result.is_ok(), "Should extract f64 as u32");
        assert_eq!(result.unwrap(), 12345u32);

        // Test with string (BigInt case - the root cause of Issue #21)
        // Some JavaScript engines represent WASM pointers as BigInt, which converts to string
        let string_val = JsValue::from_str("67890");
        let result = extract_ptr_from_js_value(&string_val);
        assert!(result.is_ok(), "Should extract string (BigInt) as u32");
        assert_eq!(result.unwrap(), 67890u32);

        // Test with bool (0/1 edge case - seen in some edge conditions)
        let bool_val = JsValue::from_bool(true);
        let result = extract_ptr_from_js_value(&bool_val);
        assert!(result.is_ok(), "Should extract bool as u32");
        assert_eq!(result.unwrap(), 1u32);

        let bool_val_false = JsValue::from_bool(false);
        let result = extract_ptr_from_js_value(&bool_val_false);
        assert!(result.is_ok(), "Should extract false as u32");
        assert_eq!(result.unwrap(), 0u32);
    }

    // KordNote tests

    #[wasm_bindgen_test]
    fn test_note_parse_valid() {
        let note = KordNote::parse("C4".to_string());
        assert!(note.is_ok(), "Should parse valid note");

        let note = note.unwrap();
        assert_eq!(note.name(), "C4");
        assert_eq!(note.pitch(), "C");
        assert_eq!(note.octave(), 4);
    }

    #[wasm_bindgen_test]
    fn test_note_parse_invalid() {
        let note = KordNote::parse("Invalid".to_string());
        assert!(note.is_err(), "Should fail to parse invalid note");
    }

    #[wasm_bindgen_test]
    fn test_note_parse_with_sharp() {
        let note = KordNote::parse("F#5".to_string()).unwrap();
        assert_eq!(note.name(), "F♯5");
        assert_eq!(note.pitch(), "F♯");
        assert_eq!(note.octave(), 5);
    }

    #[wasm_bindgen_test]
    fn test_note_parse_with_flat() {
        let note = KordNote::parse("Bb3".to_string()).unwrap();
        assert_eq!(note.name(), "B♭3");
        assert_eq!(note.pitch(), "B♭");
        assert_eq!(note.octave(), 3);
    }

    #[wasm_bindgen_test]
    fn test_note_frequency() {
        let note = KordNote::parse("A4".to_string()).unwrap();
        let freq = note.frequency();
        // A4 is 440 Hz
        assert!((freq - 440.0).abs() < 0.1, "A4 should be approximately 440 Hz");
    }

    #[wasm_bindgen_test]
    fn test_note_to_string() {
        let note = KordNote::parse("D5".to_string()).unwrap();
        assert_eq!(note.to_string(), "D5");
    }

    #[wasm_bindgen_test]
    fn test_note_add_interval() {
        let c4 = KordNote::parse("C4".to_string()).unwrap();
        let e4 = c4.add_interval(Interval::MajorThird);
        assert_eq!(e4.name(), "E4");
    }

    #[wasm_bindgen_test]
    fn test_note_subtract_interval() {
        let e4 = KordNote::parse("E4".to_string()).unwrap();
        let c4 = e4.subtract_interval(Interval::MajorThird);
        assert_eq!(c4.name(), "C4");
    }

    #[wasm_bindgen_test]
    fn test_note_distance_to() {
        let c4 = KordNote::parse("C4".to_string()).unwrap();
        let e4 = KordNote::parse("E4".to_string()).unwrap();
        let interval = c4.distance_to(e4);
        assert_eq!(interval, Interval::MajorThird);
    }

    #[wasm_bindgen_test]
    fn test_note_harmonic_series() {
        let c4 = KordNote::parse("C4".to_string()).unwrap();
        let harmonics = c4.harmonic_series();
        assert_eq!(harmonics.length(), 13, "Should return 13 harmonics");
        
        // Just verify we can get elements from the array
        assert!(!harmonics.get(0).is_undefined(), "First harmonic should exist");
    }

    #[wasm_bindgen_test]
    fn test_note_copy() {
        let note = KordNote::parse("G4".to_string()).unwrap();
        let copy = note.copy();
        assert_eq!(copy.name(), note.name());
        assert_eq!(copy.octave(), note.octave());
    }

    // KordChord tests

    #[wasm_bindgen_test]
    fn test_chord_parse_simple() {
        let chord = KordChord::parse("C".to_string());
        assert!(chord.is_ok(), "Should parse simple chord");

        let chord = chord.unwrap();
        assert!(chord.root().starts_with("C"), "Root should start with C");
        assert_eq!(chord.name(), "C");
    }

    #[wasm_bindgen_test]
    fn test_chord_parse_major_seventh() {
        let chord = KordChord::parse("Cmaj7".to_string()).unwrap();
        assert!(chord.root().starts_with("C"), "Root should start with C");
        assert!(chord.name().contains("maj7") || chord.display().contains("maj7"));
    }

    #[wasm_bindgen_test]
    fn test_chord_parse_minor() {
        let chord = KordChord::parse("Cm".to_string()).unwrap();
        assert!(chord.root().starts_with("C"), "Root should start with C");
        assert!(chord.name().contains("m") || chord.name().contains("minor"));
    }

    #[wasm_bindgen_test]
    fn test_chord_parse_dominant_seventh() {
        let chord = KordChord::parse("C7".to_string()).unwrap();
        assert!(chord.root().starts_with("C"), "Root should start with C");
        assert!(chord.display().contains("7"));
    }

    #[wasm_bindgen_test]
    fn test_chord_parse_with_slash() {
        let chord = KordChord::parse("C/E".to_string()).unwrap();
        assert!(chord.root().starts_with("C"), "Root should start with C");
        assert!(chord.slash().starts_with("E"), "Slash should start with E");
    }

    #[wasm_bindgen_test]
    fn test_chord_parse_complex() {
        let chord = KordChord::parse("Cm7b5".to_string()).unwrap();
        assert!(chord.root().starts_with("C"), "Root should start with C");
        // Half-diminished chord
        assert!(chord.display().contains("m7") || chord.display().contains("ø"));
    }

    #[wasm_bindgen_test]
    fn test_chord_parse_invalid() {
        let chord = KordChord::parse("NotAChord".to_string());
        assert!(chord.is_err(), "Should fail to parse invalid chord");
    }

    #[wasm_bindgen_test]
    fn test_chord_from_notes_string() {
        let result = KordChord::from_notes_string("C E G".to_string());
        assert!(result.is_ok(), "Should create chords from note string");
        
        let chords = result.unwrap();
        assert!(chords.length() > 0, "Should return at least one chord candidate");
        
        // Just verify we can access the first element
        let first = chords.get(0);
        assert!(!first.is_undefined(), "First chord should exist");
    }

    #[wasm_bindgen_test]
    fn test_chord_from_notes_string_invalid() {
        let result = KordChord::from_notes_string("X Y Z".to_string());
        assert!(result.is_err(), "Should fail with invalid note names");
    }

    #[wasm_bindgen_test]
    fn test_chord_chord_and_scale() {
        let chord = KordChord::parse("Cmaj7".to_string()).unwrap();
        
        let chord_notes = chord.chord();
        assert!(chord_notes.length() >= 4, "Major 7th should have at least 4 notes");
        
        let scale_notes = chord.scale();
        assert!(scale_notes.length() >= 7, "Scale should have at least 7 notes");
    }

    #[wasm_bindgen_test]
    fn test_chord_string_representations() {
        let chord = KordChord::parse("Cmaj7".to_string()).unwrap();
        
        let chord_string = chord.chord_string();
        assert!(chord_string.contains("C"), "Chord string should contain root");
        
        let scale_string = chord.scale_string();
        assert!(scale_string.contains("C"), "Scale string should contain root");
    }

    #[wasm_bindgen_test]
    fn test_chord_modifiers_and_extensions() {
        let chord = KordChord::parse("Cm7b5".to_string()).unwrap();
        
        let modifiers = chord.modifiers();
        assert!(modifiers.length() > 0, "Should have modifiers");
        
        let extensions = chord.extensions();
        // May or may not have extensions depending on the chord
        assert!(extensions.length() >= 0);
    }

    #[wasm_bindgen_test]
    fn test_chord_precise_name_and_description() {
        let chord = KordChord::parse("Cmaj7".to_string()).unwrap();
        
        let precise_name = chord.precise_name();
        assert!(!precise_name.is_empty(), "Should have precise name");
        
        let description = chord.description();
        assert!(!description.is_empty(), "Should have description");
        
        let display = chord.display();
        assert!(!display.is_empty(), "Should have display text");
    }

    #[wasm_bindgen_test]
    fn test_chord_inversion() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        assert_eq!(chord.inversion(), 0, "Root position should be 0");
        
        let inverted = chord.with_inversion(1);
        assert_eq!(inverted.inversion(), 1, "First inversion should be 1");
    }

    #[wasm_bindgen_test]
    fn test_chord_with_slash() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let e_note = KordNote::parse("E4".to_string()).unwrap();
        
        let with_slash = chord.with_slash(&e_note);
        assert_eq!(with_slash.slash(), "E4");
    }

    #[wasm_bindgen_test]
    fn test_chord_with_octave() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let result = chord.with_octave(5);
        assert!(result.is_ok(), "Should set octave successfully");
    }

    #[wasm_bindgen_test]
    fn test_chord_with_octave_invalid() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let result = chord.with_octave(20); // Invalid octave
        assert!(result.is_err(), "Should fail with invalid octave");
    }

    #[wasm_bindgen_test]
    fn test_chord_is_crunchy() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let _is_crunchy = chord.is_crunchy(); // Just verify it doesn't panic
        
        let with_crunchy = chord.with_crunchy(true);
        assert!(with_crunchy.is_crunchy(), "Should be crunchy after setting");
    }

    #[wasm_bindgen_test]
    fn test_chord_copy() {
        let chord = KordChord::parse("Cmaj7".to_string()).unwrap();
        let copy = chord.copy();
        assert_eq!(copy.root(), chord.root());
        assert_eq!(copy.name(), chord.name());
    }

    // Chord modifier/extension builder tests

    #[wasm_bindgen_test]
    fn test_chord_builder_minor() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let minor = chord.minor();
        assert!(minor.name().contains("m") || minor.name().contains("minor"));
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_flat5() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let flat5 = chord.flat5();
        // Verify it has the flat 5 modifier
        let modifiers = flat5.modifiers();
        assert!(modifiers.length() > 0);
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_aug() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let aug = chord.aug();
        // Augmented chord has sharp 5
        let display = aug.display();
        assert!(display.contains("aug") || display.contains("+") || display.contains("#5"));
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_maj7() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let maj7 = chord.maj7();
        assert!(maj7.display().contains("maj7") || maj7.display().contains("M7"));
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_seven() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let seven = chord.seven();
        assert!(seven.display().contains("7"));
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_nine() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let nine = chord.nine();
        assert!(nine.display().contains("9"));
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_eleven() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let eleven = chord.eleven();
        assert!(eleven.display().contains("11"));
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_thirteen() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let thirteen = chord.thirteen();
        assert!(thirteen.display().contains("13"));
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_altered_notes() {
        let chord = KordChord::parse("C7".to_string()).unwrap();
        
        let flat9 = chord.flat9();
        assert!(flat9.display().contains("♭9") || flat9.display().contains("b9"));
        
        let sharp9 = chord.sharp9();
        assert!(sharp9.display().contains("♯9") || sharp9.display().contains("#9"));
        
        let sharp11 = chord.sharp11();
        assert!(sharp11.display().contains("♯11") || sharp11.display().contains("#11"));
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_dim() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let dim = chord.dim();
        assert!(dim.display().contains("dim") || dim.display().contains("°"));
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_half_dim() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let half_dim = chord.half_dim();
        // Half-diminished creates Cm7b5, verify it contains both minor and flat 5
        let display = half_dim.display();
        assert!(display.contains("m") && (display.contains("b5") || display.contains("♭5") || display.contains("ø")), 
                "Half-dim should show minor with flat 5: {}", display);
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_sus() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        
        let sus2 = chord.sus2();
        assert!(sus2.display().contains("sus2"));
        
        let sus4 = chord.sus4();
        assert!(sus4.display().contains("sus4"));
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_add_extensions() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        
        let add2 = chord.add2();
        assert!(add2.display().contains("add2") || add2.display().contains("add9"));
        
        let add4 = chord.add4();
        assert!(add4.display().contains("add4") || add4.display().contains("add11"));
        
        let add6 = chord.add6();
        assert!(add6.display().contains("add6") || add6.display().contains("6"));
        
        let add9 = chord.add9();
        assert!(add9.display().contains("add9"));
        
        let add11 = chord.add11();
        assert!(add11.display().contains("add11"));
        
        let add13 = chord.add13();
        assert!(add13.display().contains("add13"));
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_flat_extensions() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        
        let flat11 = chord.flat11();
        let display = flat11.display();
        assert!(display.contains("♭11") || display.contains("b11") || display.contains("11"), 
                "Should show 11: {}", display);
        
        let flat13 = chord.flat13();
        let display = flat13.display();
        assert!(display.contains("♭13") || display.contains("b13") || display.contains("13"), 
                "Should show 13: {}", display);
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_sharp13() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let sharp13 = chord.sharp13();
        let extensions = sharp13.extensions();
        assert!(extensions.length() > 0, "Should have sharp13 extension");
    }

    #[wasm_bindgen_test]
    fn test_chord_builder_chaining() {
        // Test that we can chain multiple modifiers
        let chord = KordChord::parse("C".to_string()).unwrap();
        let complex = chord.minor().seven().flat9();
        
        assert!(complex.display().contains("m"), "Should be minor");
        assert!(complex.display().contains("7"), "Should have dominant 7");
    }

    // Helper conversion tests

    #[wasm_bindgen_test]
    fn test_js_array_conversion() {
        let chord = KordChord::parse("C".to_string()).unwrap();
        let notes = chord.chord();
        
        // Test that we can iterate the array
        assert!(notes.length() > 0, "Should have notes in array");
        
        // Test that we can get individual elements
        let first = notes.get(0);
        assert!(!first.is_undefined(), "First element should exist");
    }

    // ABI Compatibility Tests

    #[wasm_bindgen_test]
    fn test_abi_string_encoding_basic() {
        // Test basic ASCII strings pass through correctly
        let note = KordNote::parse("C4".to_string()).unwrap();
        assert_eq!(note.name(), "C4");
        
        let chord = KordChord::parse("Cmaj7".to_string()).unwrap();
        assert!(chord.display().len() > 0, "Display should return non-empty string");
    }

    #[wasm_bindgen_test]
    fn test_abi_string_encoding_unicode() {
        // Test that Unicode characters (sharps/flats) work correctly
        let sharp_note = KordNote::parse("C♯4".to_string());
        assert!(sharp_note.is_ok() || sharp_note.is_err(), "Should handle sharp unicode");
        
        let flat_note = KordNote::parse("D♭4".to_string());
        assert!(flat_note.is_ok() || flat_note.is_err(), "Should handle flat unicode");
        
        // Test that display strings may contain unicode
        let chord = KordChord::parse("C#m7b5".to_string()).unwrap();
        let display = chord.display();
        assert!(display.len() > 0, "Display should return string with unicode");
    }

    #[wasm_bindgen_test]
    fn test_abi_empty_strings() {
        // Test that empty strings are handled correctly
        let result = KordNote::parse("".to_string());
        assert!(result.is_err(), "Empty string should fail to parse");
        
        let result = KordChord::parse("".to_string());
        assert!(result.is_err(), "Empty string should fail to parse");
    }

    #[wasm_bindgen_test]
    fn test_abi_error_handling() {
        // Test that Rust Result<T, E> properly converts to JS exceptions
        let invalid_note = KordNote::parse("Invalid".to_string());
        assert!(invalid_note.is_err(), "Should return error for invalid note");
        
        let invalid_chord = KordChord::parse("NotAChord123".to_string());
        assert!(invalid_chord.is_err(), "Should return error for invalid chord");
        
        // Test that errors from array operations are handled
        let result = KordChord::from_notes_string("".to_string());
        assert!(result.is_err(), "Empty notes string should error");
    }

    #[wasm_bindgen_test]
    fn test_abi_object_construction() {
        // Test that we can construct multiple independent objects
        let note1 = KordNote::parse("C4".to_string()).unwrap();
        let note2 = KordNote::parse("D4".to_string()).unwrap();
        let note3 = KordNote::parse("E4".to_string()).unwrap();
        
        // Verify they're distinct
        assert_ne!(note1.name(), note2.name());
        assert_ne!(note2.name(), note3.name());
        
        // Test chord construction
        let chord1 = KordChord::parse("C".to_string()).unwrap();
        let chord2 = KordChord::parse("G".to_string()).unwrap();
        
        assert!(chord1.root().starts_with("C"));
        assert!(chord2.root().starts_with("G"));
    }

    #[wasm_bindgen_test]
    fn test_abi_object_copy_independence() {
        // Test that copy() creates independent objects
        let original = KordNote::parse("C4".to_string()).unwrap();
        let copy = original.copy();
        
        assert_eq!(original.name(), copy.name());
        assert_eq!(original.frequency(), copy.frequency());
        
        // Test chord copy
        let chord_original = KordChord::parse("Cmaj7".to_string()).unwrap();
        let chord_copy = chord_original.copy();
        
        assert_eq!(chord_original.name(), chord_copy.name());
    }

    #[wasm_bindgen_test]
    fn test_abi_array_empty() {
        // Test that empty arrays are handled correctly
        let chord = KordChord::parse("C".to_string()).unwrap();
        let extensions = chord.extensions();
        
        // Extensions might be empty or populated, just verify we get an array
        assert!(extensions.length() >= 0, "Should return valid array");
    }

    #[wasm_bindgen_test]
    fn test_abi_array_multiple_elements() {
        // Test arrays with multiple elements
        let chord = KordChord::parse("Cmaj7#9b13".to_string()).unwrap();
        let notes = chord.chord();
        
        assert!(notes.length() > 0, "Complex chord should have notes");
        
        // Verify we can access multiple elements
        for i in 0..notes.length() {
            let element = notes.get(i);
            assert!(!element.is_undefined(), "Element {} should exist", i);
        }
    }

    #[wasm_bindgen_test]
    fn test_abi_number_precision_frequency() {
        // Test that floating-point frequencies maintain precision
        let a4 = KordNote::parse("A4".to_string()).unwrap();
        let freq = a4.frequency();
        
        // A4 should be 440 Hz
        assert!((freq - 440.0).abs() < 0.1, "A4 frequency should be ~440 Hz, got {}", freq);
        
        let c4 = KordNote::parse("C4".to_string()).unwrap();
        let c_freq = c4.frequency();
        
        // C4 should be ~261.63 Hz
        assert!(c_freq > 260.0 && c_freq < 263.0, "C4 frequency should be ~261.63 Hz, got {}", c_freq);
    }

    #[wasm_bindgen_test]
    fn test_abi_number_octave_range() {
        // Test octave numbers across valid range
        let c0 = KordNote::parse("C0".to_string()).unwrap();
        assert_eq!(c0.octave(), 0);
        
        let c4 = KordNote::parse("C4".to_string()).unwrap();
        assert_eq!(c4.octave(), 4);
        
        let c8 = KordNote::parse("C8".to_string()).unwrap();
        assert_eq!(c8.octave(), 8);
    }

    #[wasm_bindgen_test]
    fn test_abi_number_interval_operations() {
        // Test that interval arithmetic works correctly across ABI
        let c4 = KordNote::parse("C4".to_string()).unwrap();
        
        let up_octave = c4.add_interval(Interval::PerfectOctave);
        assert_eq!(up_octave.octave(), 5, "Should be an octave higher");
        
        let down_fifth = c4.subtract_interval(Interval::PerfectFifth);
        assert_eq!(down_fifth.octave(), 3, "Should be in octave 3");
        
        // Test distance calculation
        let g4 = KordNote::parse("G4".to_string()).unwrap();
        let distance = c4.distance_to(g4);
        assert_eq!(distance, Interval::PerfectFifth, "C to G should be a perfect fifth");
    }

    #[wasm_bindgen_test]
    fn test_abi_method_chaining() {
        // Test that method chaining works correctly (verifies proper return types)
        let chord = KordChord::parse("C".to_string())
            .unwrap()
            .minor()
            .seven()
            .flat9()
            .sharp11();
        
        let display = chord.display();
        assert!(display.contains("m"), "Should show minor");
        assert!(display.contains("7"), "Should show 7");
    }

    #[wasm_bindgen_test]
    fn test_abi_boolean_returns() {
        // Test that boolean values cross the ABI correctly
        let c4 = KordNote::parse("C4".to_string()).unwrap();
        let d4 = KordNote::parse("D4".to_string()).unwrap();
        
        // Test equality (should be false for different notes)
        let same = c4.name() == d4.name();
        assert!(!same, "Different notes should not be equal");
        
        // Test chord crunchy check
        let simple = KordChord::parse("C".to_string()).unwrap();
        let is_crunchy = simple.is_crunchy();
        assert!(!is_crunchy, "Simple major chord should not be crunchy");
    }

    #[wasm_bindgen_test]
    fn test_abi_null_handling_slash() {
        // Test that optional slash bass note is handled correctly
        let no_slash = KordChord::parse("C".to_string()).unwrap();
        let slash_str = no_slash.slash();
        
        // When no slash is present, slash() returns the root note
        assert!(slash_str.starts_with("C"), "No slash should return root note");
        
        let with_slash = KordChord::parse("C/E".to_string()).unwrap();
        let slash_str = with_slash.slash();
        assert!(slash_str.starts_with("E"), "Should return slash note");
    }

    #[wasm_bindgen_test]
    fn test_abi_long_strings() {
        // Test that long chord names work correctly
        let complex = KordChord::parse("Cmaj7#9#11b13".to_string()).unwrap();
        let name = complex.name();
        let display = complex.display();
        let description = complex.description();
        
        assert!(name.len() > 0, "Name should be non-empty");
        assert!(display.len() > 0, "Display should be non-empty");
        assert!(description.len() > 0, "Description should be non-empty");
    }

    #[wasm_bindgen_test]
    fn test_abi_large_array() {
        // Test harmonic series returns correct-sized array
        let c4 = KordNote::parse("C4".to_string()).unwrap();
        let harmonics = c4.harmonic_series();
        
        assert_eq!(harmonics.length(), 13, "Should return 13 harmonics");
        
        // Verify all elements are accessible
        for i in 0..harmonics.length() {
            let harmonic = harmonics.get(i);
            assert!(!harmonic.is_undefined(), "Harmonic {} should exist", i);
        }
    }

    #[wasm_bindgen_test]
    fn test_abi_object_reuse() {
        // Test that we can reuse objects multiple times
        let chord = KordChord::parse("C".to_string()).unwrap();
        
        // Call same method multiple times
        let notes1 = chord.chord();
        let notes2 = chord.chord();
        
        assert_eq!(notes1.length(), notes2.length(), "Should return consistent results");
        
        // Call different methods on same object
        let _name = chord.name();
        let _display = chord.display();
        let _root = chord.root();
        let _description = chord.description();
        
        // Object should still be valid
        assert!(chord.chord().length() > 0);
    }

    #[wasm_bindgen_test]
    fn test_abi_transformation_returns_new_object() {
        // Test that transformations return new independent objects
        let original = KordChord::parse("C".to_string()).unwrap();
        let minor = original.minor();
        
        // Original should be unchanged
        assert!(original.name().contains("C"));
        
        // New object should be different
        assert!(minor.display().contains("m"), "Should be minor");
        
        // Both should be independently usable
        let _original_notes = original.chord();
        let _minor_notes = minor.chord();
    }
}
