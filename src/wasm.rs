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

impl RefFromJsValue for KordNote {
    fn ref_from_js_value(abi: &JsValue) -> JsRes<<KordNote as RefFromWasmAbi>::Anchor>
    where
        Self: Sized + RefFromWasmAbi,
    {
        let ptr = Reflect::get(abi, &JsValue::from_str("ptr"))?.as_f64().ok_or("Could not cast pointer to f64.")? as u32;

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
        let ptr = Reflect::get(abi, &JsValue::from_str("ptr"))?.as_f64().ok_or("Could not cast pointer to f64.")? as u32;

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
