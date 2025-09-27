//! A module for working with notes.
//!
//! A note is a named pitch with an octave.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Sub},
};

use crate::core::{
    base::{HasName, HasStaticName, Parsable, Res},
    chord::Chord,
    interval::{HasEnharmonicDistance, Interval, PRIMARY_HARMONIC_SERIES},
    named_pitch::{HasNamedPitch, NamedPitch},
    octave::{HasOctave, Octave, ALL_OCTAVES},
    parser::{note_str_to_note, octave_str_to_octave, ChordParser, Rule},
    pitch::{HasBaseFrequency, HasFrequency, HasPitch, Pitch, ALL_PITCHES},
};
use paste::paste;
use pest::Parser;
use std::sync::LazyLock;

use super::interval::ALL_INTERVALS;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Macros.

/// Defines a note from a [`NamedPitch`].
macro_rules! define_note {
    ( $name:ident, $named_pitch:expr, $octave_num:ident, $octave:expr) => {
        paste! {
            /// The note [<$name$octave_num>].
            pub const [<$name$octave_num>]: Note = Note {
                named_pitch: $named_pitch,
                octave: $octave,
            };
        }
    };
}

/// Defines an octave of notes.
macro_rules! define_octave {
    ($octave_num:ident, $octave:expr) => {
        define_note!(FTripleFlat, NamedPitch::FTripleFlat, $octave_num, $octave);
        define_note!(CTripleFlat, NamedPitch::CTripleFlat, $octave_num, $octave);
        define_note!(GTripleFlat, NamedPitch::GTripleFlat, $octave_num, $octave);
        define_note!(DTripleFlat, NamedPitch::DTripleFlat, $octave_num, $octave);
        define_note!(ATripleFlat, NamedPitch::ATripleFlat, $octave_num, $octave);
        define_note!(ETripleFlat, NamedPitch::ETripleFlat, $octave_num, $octave);
        define_note!(BTripleFlat, NamedPitch::BTripleFlat, $octave_num, $octave);

        define_note!(FDoubleFlat, NamedPitch::FDoubleFlat, $octave_num, $octave);
        define_note!(CDoubleFlat, NamedPitch::CDoubleFlat, $octave_num, $octave);
        define_note!(GDoubleFlat, NamedPitch::GDoubleFlat, $octave_num, $octave);
        define_note!(DDoubleFlat, NamedPitch::DDoubleFlat, $octave_num, $octave);
        define_note!(ADoubleFlat, NamedPitch::ADoubleFlat, $octave_num, $octave);
        define_note!(EDoubleFlat, NamedPitch::EDoubleFlat, $octave_num, $octave);
        define_note!(BDoubleFlat, NamedPitch::BDoubleFlat, $octave_num, $octave);

        define_note!(FFlat, NamedPitch::FFlat, $octave_num, $octave);
        define_note!(CFlat, NamedPitch::CFlat, $octave_num, $octave);
        define_note!(GFlat, NamedPitch::GFlat, $octave_num, $octave);
        define_note!(DFlat, NamedPitch::DFlat, $octave_num, $octave);
        define_note!(AFlat, NamedPitch::AFlat, $octave_num, $octave);
        define_note!(EFlat, NamedPitch::EFlat, $octave_num, $octave);
        define_note!(BFlat, NamedPitch::BFlat, $octave_num, $octave);

        define_note!(F, NamedPitch::F, $octave_num, $octave);
        define_note!(C, NamedPitch::C, $octave_num, $octave);
        define_note!(G, NamedPitch::G, $octave_num, $octave);
        define_note!(D, NamedPitch::D, $octave_num, $octave);
        define_note!(A, NamedPitch::A, $octave_num, $octave);
        define_note!(E, NamedPitch::E, $octave_num, $octave);
        define_note!(B, NamedPitch::B, $octave_num, $octave);

        define_note!(FSharp, NamedPitch::FSharp, $octave_num, $octave);
        define_note!(CSharp, NamedPitch::CSharp, $octave_num, $octave);
        define_note!(GSharp, NamedPitch::GSharp, $octave_num, $octave);
        define_note!(DSharp, NamedPitch::DSharp, $octave_num, $octave);
        define_note!(ASharp, NamedPitch::ASharp, $octave_num, $octave);
        define_note!(ESharp, NamedPitch::ESharp, $octave_num, $octave);
        define_note!(BSharp, NamedPitch::BSharp, $octave_num, $octave);

        define_note!(FDoubleSharp, NamedPitch::FDoubleSharp, $octave_num, $octave);
        define_note!(CDoubleSharp, NamedPitch::CDoubleSharp, $octave_num, $octave);
        define_note!(GDoubleSharp, NamedPitch::GDoubleSharp, $octave_num, $octave);
        define_note!(DDoubleSharp, NamedPitch::DDoubleSharp, $octave_num, $octave);
        define_note!(ADoubleSharp, NamedPitch::ADoubleSharp, $octave_num, $octave);
        define_note!(EDoubleSharp, NamedPitch::EDoubleSharp, $octave_num, $octave);
        define_note!(BDoubleSharp, NamedPitch::BDoubleSharp, $octave_num, $octave);

        define_note!(FTripleSharp, NamedPitch::FTripleSharp, $octave_num, $octave);
        define_note!(CTripleSharp, NamedPitch::CTripleSharp, $octave_num, $octave);
        define_note!(GTripleSharp, NamedPitch::GTripleSharp, $octave_num, $octave);
        define_note!(DTripleSharp, NamedPitch::DTripleSharp, $octave_num, $octave);
        define_note!(ATripleSharp, NamedPitch::ATripleSharp, $octave_num, $octave);
        define_note!(ETripleSharp, NamedPitch::ETripleSharp, $octave_num, $octave);
        define_note!(BTripleSharp, NamedPitch::BTripleSharp, $octave_num, $octave);
    };
}

// Traits.

/// A trait for types that can be converted into a [`Chord`].
pub trait IntoChord {
    /// Converts this type into a [`Chord`] (usually a [`Note`]).
    fn into_chord(self) -> Chord;
}

/// A trait which allows for a [`Note`] to be recreated with different properties.
pub trait NoteRecreator {
    /// Recreates this [`Note`] with the given [`NamedPitch`].
    fn with_named_pitch(self, named_pitch: NamedPitch) -> Self;
    /// Recreates this [`Note`] with the given [`Octave`].
    fn with_octave(self, octave: Octave) -> Self;
}

/// A trait which allows for obtaining the primary harmonic series of the note.
pub trait HasPrimaryHarmonicSeries {
    /// Returns the primary harmonic series of the note.
    fn primary_harmonic_series(self) -> Vec<Note>;
}

/// A trait which allows for encoding the note as a [`u128`] ID.
pub trait HasNoteId {
    /// Returns the ID of the note.
    fn id(self) -> u128;

    /// Returns the position of the 1 for the ID of the note.
    fn id_index(self) -> u8;

    /// Returns the note from the given ID.
    fn from_id(id: u128) -> Res<Self>
    where
        Self: Sized;

    /// Returns the ID mask for the given notes.
    fn id_mask(notes: &[Self]) -> u128
    where
        Self: Sized;

    /// Returns the notes from the given ID mask.
    fn from_id_mask(id_mask: u128) -> Res<Vec<Self>>
    where
        Self: Sized;
}

/// A trait which allows for converting a note to the same octave, but using universal [`Pitch`]es.
///
/// Essentially, this would convert an F#4 into a Gb4, since [`Pitch`]es prefer the flats.
pub trait ToUniversal {
    /// Converts this note to a universal pitch.
    fn to_universal(self) -> Self;
}

// Struct.

/// A note type.
///
/// This is a named pitch with an octave.  This type allows for correctly attributing octave changes
/// across an interval from one [`Note`] to another.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Note {
    /// The octave of the note.
    octave: Octave,
    /// The named pitch of the note.
    named_pitch: NamedPitch,
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

// Impls.

impl Note {
    /// Creates a new [`Note`] from the given [`NamedPitch`] and [`Octave`].
    pub fn new(pitch: NamedPitch, octave: Octave) -> Self {
        Self { named_pitch: pitch, octave }
    }

    /// Attempts to create a [`Note`] from a MIDI note number.
    pub fn try_from_midi(midi: u8) -> Res<Self> {
        let pitch_index = midi % 12;
        let octave_component = midi / 12;

        // MIDI note numbers are defined such that note 60 is C4, which corresponds to octave index 4.
        // Therefore, subtract 1 to map to the [`Octave`] enum where 0 == C0.
        let octave_value = (octave_component as i16) - 1;

        if octave_value < 0 {
            return Err(anyhow::Error::msg(format!("MIDI note {midi} is below the supported octave range.")));
        }

        let octave = Octave::try_from(octave_value as u8).map_err(anyhow::Error::msg)?;
        let pitch = Pitch::try_from(pitch_index).map_err(anyhow::Error::msg)?;

        Ok(Self::new(NamedPitch::from(pitch), octave))
    }
}

impl Note {
    /// Attempts to use the default microphone to listen to audio for the specified time
    /// to identify the notes in the recorded audio.
    ///
    /// Currently, this does not work with WASM.
    #[coverage(off)]
    #[cfg(feature = "analyze_mic")]
    pub async fn try_from_mic(length_in_seconds: u8) -> Res<Vec<Note>> {
        use crate::analyze::mic::get_notes_from_microphone;

        get_notes_from_microphone(length_in_seconds).await
    }

    /// Attempts to use the provided to identify the notes in the audio data.
    #[cfg(feature = "analyze_base")]
    pub fn try_from_audio(data: &[f32], length_in_seconds: u8) -> Res<Vec<Note>> {
        use crate::analyze::base::get_notes_from_audio_data;

        get_notes_from_audio_data(data, length_in_seconds)
    }

    /// Attempts to use the default microphone to listen to audio for the specified time
    /// to identify the notes in the recorded audio using ML.
    ///
    /// Currently, this does not work with WASM.
    #[coverage(off)]
    #[cfg(all(feature = "ml_infer", feature = "analyze_mic"))]
    pub async fn try_from_mic_ml(length_in_seconds: u8) -> Res<Vec<Self>> {
        use crate::{analyze::mic::get_audio_data_from_microphone, ml::infer::infer};

        let audio_data = get_audio_data_from_microphone(length_in_seconds).await?;

        infer(&audio_data, length_in_seconds)
    }

    /// Attempts to use the provided to identify the notes in the audio data using ML.
    #[cfg(all(feature = "ml_infer", feature = "analyze_base"))]
    pub fn try_from_audio_ml(data: &[f32], length_in_seconds: u8) -> Res<Vec<Self>> {
        use crate::ml::infer::infer;

        infer(data, length_in_seconds)
    }
}

impl HasPitch for Note {
    fn pitch(&self) -> Pitch {
        self.named_pitch.pitch()
    }
}

impl HasNamedPitch for Note {
    fn named_pitch(&self) -> NamedPitch {
        self.named_pitch
    }
}

impl HasOctave for Note {
    fn octave(&self) -> Octave {
        self.octave
    }
}

impl HasStaticName for Note {
    fn static_name(&self) -> &'static str {
        self.named_pitch.static_name()
    }
}

impl HasName for Note {
    fn name(&self) -> String {
        format!("{}{}", self.named_pitch.static_name(), self.octave.static_name())
    }
}

impl HasFrequency for Note {
    fn frequency(&self) -> f32 {
        let mut octave = self.octave();
        let base_frequency = self.pitch().base_frequency();

        match self.named_pitch {
            NamedPitch::ATripleSharp | NamedPitch::BTripleSharp | NamedPitch::BDoubleSharp | NamedPitch::BSharp => {
                octave += 1;
            }
            NamedPitch::DTripleFlat | NamedPitch::CTripleFlat | NamedPitch::CDoubleFlat | NamedPitch::CFlat => {
                octave -= 1;
            }
            _ => {}
        }

        base_frequency * 2.0_f32.powf(octave as u8 as f32)
    }
}

impl IntoChord for Note {
    fn into_chord(self) -> Chord {
        Chord::new(self)
    }
}

impl Parsable for Note {
    fn parse(input: &str) -> Res<Self>
    where
        Self: Sized,
    {
        let root = ChordParser::parse(Rule::note_with_octave, input)?.next().unwrap();

        assert_eq!(Rule::note_with_octave, root.as_rule());

        let mut components = root.into_inner();

        let note = components.next().unwrap();

        assert_eq!(Rule::note, note.as_rule());

        let mut result = note_str_to_note(note.as_str())?;

        if let Some(octave) = components.next() {
            assert_eq!(Rule::digit, octave.as_rule());

            let octave = octave_str_to_octave(octave.as_str())?;

            result = result.with_octave(octave);
        }

        Ok(result)
    }
}

impl NoteRecreator for Note {
    fn with_named_pitch(self, named_pitch: NamedPitch) -> Self {
        Self::new(named_pitch, self.octave)
    }

    fn with_octave(self, octave: Octave) -> Self {
        Self::new(self.named_pitch, octave)
    }
}

impl HasPrimaryHarmonicSeries for Note {
    fn primary_harmonic_series(self) -> Vec<Self> {
        PRIMARY_HARMONIC_SERIES.iter().map(|interval| self + *interval).collect()
    }
}

impl HasNoteId for Note {
    fn id(self) -> u128 {
        1 << self.id_index()
    }

    fn id_index(self) -> u8 {
        let mut shift = 0u8;

        shift += 12 * self.octave as u8;
        shift += self.named_pitch.pitch() as u8;

        shift
    }

    fn from_id(id: u128) -> Res<Self> {
        let mut shift = 0u8;

        while id >> shift != 1 {
            shift += 1;
        }

        let octave_num = shift / 12;
        let pitch_num = shift % 12;

        let octave = Octave::try_from(octave_num).map_err(anyhow::Error::msg)?;
        let pitch = Pitch::try_from(pitch_num).map_err(anyhow::Error::msg)?;

        Ok(Self::new(NamedPitch::from(pitch), octave))
    }

    fn id_mask(notes: &[Self]) -> u128
    where
        Self: Sized,
    {
        notes.iter().fold(0, |acc, note| acc | note.id())
    }

    fn from_id_mask(id_mask: u128) -> Res<Vec<Self>>
    where
        Self: Sized,
    {
        let mut notes = Vec::new();
        let mut shift = 0u8;

        while id_mask >> shift != 0 {
            if id_mask & (1 << shift) != 0 {
                notes.push(Self::from_id(1 << shift)?);
            }

            shift += 1;
        }

        Ok(notes)
    }
}

impl ToUniversal for Note {
    fn to_universal(self) -> Note {
        self.with_named_pitch(NamedPitch::from(self.pitch()))
    }
}

impl Sub for Note {
    type Output = Interval;

    fn sub(self, rhs: Self) -> Self::Output {
        let (low, high) = if self < rhs { (self, rhs) } else { (rhs, self) };

        for interval in ALL_INTERVALS.iter() {
            if low + *interval == high {
                return *interval;
            }
        }

        panic!("{high} - {low} is not a valid interval");
    }
}

impl Add<Interval> for Note {
    type Output = Self;

    #[rustfmt::skip]
    fn add(self, rhs: Interval) -> Self::Output {
        let new_pitch = self.named_pitch() + rhs.enharmonic_distance();

        // Compute whether or not we "crossed" an octave.
        let wrapping_octave = if new_pitch.pitch() < self.pitch() { Octave::One } else { Octave::Zero };

        // There is a "special wrap" for `Cb`, and `Dbbb`, since they don't technically loop; and, for B#, etc., on the other side.
        // Basically, if we were already "on" the weird one (this is a perfect unision, or perfect octave, etc.), then we don't
        // do anything special.  Otherwise, if we landed on on of these edge cases, then we need to adjust the octave.
        let mut special_octave = 0;

        if self.named_pitch != new_pitch {
            if new_pitch == NamedPitch::CFlat
                || new_pitch == NamedPitch::CDoubleFlat
                || new_pitch == NamedPitch::CTripleFlat
                || new_pitch == NamedPitch::DTripleFlat
            {
                special_octave = 1;
            } else if new_pitch == NamedPitch::BSharp
                || new_pitch == NamedPitch::BDoubleSharp
                || new_pitch == NamedPitch::BTripleSharp
                || new_pitch == NamedPitch::ATripleSharp
            {
                special_octave = -1
            }
        }

        // Get whether or not the interval itself contains an octave.
        let interval_octave = rhs.octave();

        Note {
            octave: self.octave + wrapping_octave + special_octave + interval_octave,
            named_pitch: new_pitch,
        }
    }
}

impl Sub<Interval> for Note {
    type Output = Self;

    #[rustfmt::skip]
    fn sub(self, rhs: Interval) -> Self::Output {
        let new_pitch = self.named_pitch() - rhs.enharmonic_distance();

        // Compute whether or not we "crossed" an octave.
        let wrapping_octave = if new_pitch.pitch() > self.pitch() { Octave::One } else { Octave::Zero };

        // There is a "special wrap" for `Cb`, and `Dbbb`, since they don't technically loop; and, for B#, etc., on the other side.
        // Basically, if we were already "on" the weird one (this is a perfect unision, or perfect octave, etc.), then we don't
        // do anything special.  Otherwise, if we landed on on of these edge cases, then we need to adjust the octave.
        let mut special_octave = 0;

        if self.named_pitch != new_pitch {
            if new_pitch == NamedPitch::CFlat
                || new_pitch == NamedPitch::CDoubleFlat
                || new_pitch == NamedPitch::CTripleFlat
                || new_pitch == NamedPitch::DTripleFlat
            {
                special_octave = -1;
            } else if new_pitch == NamedPitch::BSharp
                || new_pitch == NamedPitch::BDoubleSharp
                || new_pitch == NamedPitch::BTripleSharp
                || new_pitch == NamedPitch::ATripleSharp
            {
                special_octave = 1
            }
        }

        // Get whether or not the interval itself contains an octave.
        let interval_octave = rhs.octave();

        Note {
            octave: self.octave - wrapping_octave - special_octave - interval_octave,
            named_pitch: new_pitch,
        }
    }
}

impl AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency().partial_cmp(&other.frequency()).unwrap_or(Ordering::Equal)
    }
}

// Define octaves.

define_octave!(Zero, Octave::Zero);
define_octave!(One, Octave::One);
define_octave!(Two, Octave::Two);
define_octave!(Three, Octave::Three);
define_octave!(Four, Octave::Four);
define_octave!(Five, Octave::Five);
define_octave!(Six, Octave::Six);
define_octave!(Seven, Octave::Seven);
define_octave!(Eight, Octave::Eight);
define_octave!(Nine, Octave::Nine);
define_octave!(Ten, Octave::Ten);

// Define notes.

/// The default F triple flat (in the fourth octave).
pub const FTripleFlat: Note = FTripleFlatFour;
/// The default C triple flat (in the fourth octave).
pub const CTripleFlat: Note = CTripleFlatFour;
/// The default G triple flat (in the fourth octave).
pub const GTripleFlat: Note = GTripleFlatFour;
/// The default D triple flat (in the fourth octave).
pub const DTripleFlat: Note = DTripleFlatFour;
/// The default A triple flat (in the fourth octave).
pub const ATripleFlat: Note = ATripleFlatFour;
/// The default E triple flat (in the fourth octave).
pub const ETripleFlat: Note = ETripleFlatFour;
/// The default B triple flat (in the fourth octave).
pub const BTripleFlat: Note = BTripleFlatFour;

/// The default F double flat (in the fourth octave).
pub const FDoubleFlat: Note = FDoubleFlatFour;
/// The default C double flat (in the fourth octave).
pub const CDoubleFlat: Note = CDoubleFlatFour;
/// The default G double flat (in the fourth octave).
pub const GDoubleFlat: Note = GDoubleFlatFour;
/// The default D double flat (in the fourth octave).
pub const DDoubleFlat: Note = DDoubleFlatFour;
/// The default A double flat (in the fourth octave).
pub const ADoubleFlat: Note = ADoubleFlatFour;
/// The default E double flat (in the fourth octave).
pub const EDoubleFlat: Note = EDoubleFlatFour;
/// The default B double flat (in the fourth octave).
pub const BDoubleFlat: Note = BDoubleFlatFour;

/// The default F flat (in the fourth octave).
pub const FFlat: Note = FFlatFour;
/// The default C flat (in the fourth octave).
pub const CFlat: Note = CFlatFour;
/// The default G flat (in the fourth octave).
pub const GFlat: Note = GFlatFour;
/// The default D flat (in the fourth octave).
pub const DFlat: Note = DFlatFour;
/// The default A flat (in the fourth octave).
pub const AFlat: Note = AFlatFour;
/// The default E flat (in the fourth octave).
pub const EFlat: Note = EFlatFour;
/// The default B flat (in the fourth octave).
pub const BFlat: Note = BFlatFour;

/// The default F (in the fourth octave).
pub const F: Note = FFour;
/// The default C (in the fourth octave).
pub const C: Note = CFour;
/// The default G (in the fourth octave).
pub const G: Note = GFour;
/// The default D (in the fourth octave).
pub const D: Note = DFour;
/// The default A (in the fourth octave).
pub const A: Note = AFour;
/// The default E (in the fourth octave).
pub const E: Note = EFour;
/// The default B (in the fourth octave).
pub const B: Note = BFour;

/// The default F sharp (in the fourth octave).
pub const FSharp: Note = FSharpFour;
/// The default C sharp (in the fourth octave).
pub const CSharp: Note = CSharpFour;
/// The default G sharp (in the fourth octave).
pub const GSharp: Note = GSharpFour;
/// The default D sharp (in the fourth octave).
pub const DSharp: Note = DSharpFour;
/// The default A sharp (in the fourth octave).
pub const ASharp: Note = ASharpFour;
/// The default E sharp (in the fourth octave).
pub const ESharp: Note = ESharpFour;
/// The default B sharp (in the fourth octave).
pub const BSharp: Note = BSharpFour;

/// The default F double sharp (in the fourth octave).
pub const FDoubleSharp: Note = FDoubleSharpFour;
/// The default C double sharp (in the fourth octave).
pub const CDoubleSharp: Note = CDoubleSharpFour;
/// The default G double sharp (in the fourth octave).
pub const GDoubleSharp: Note = GDoubleSharpFour;
/// The default D double sharp (in the fourth octave).
pub const DDoubleSharp: Note = DDoubleSharpFour;
/// The default A double sharp (in the fourth octave).
pub const ADoubleSharp: Note = ADoubleSharpFour;
/// The default E double sharp (in the fourth octave).
pub const EDoubleSharp: Note = EDoubleSharpFour;
/// The default B double sharp (in the fourth octave).
pub const BDoubleSharp: Note = BDoubleSharpFour;

/// The default F triple sharp (in the fourth octave).
pub const FTripleSharp: Note = FTripleSharpFour;
/// The default C triple sharp (in the fourth octave).
pub const CTripleSharp: Note = CTripleSharpFour;
/// The default G triple sharp (in the fourth octave).
pub const GTripleSharp: Note = GTripleSharpFour;
/// The default D triple sharp (in the fourth octave).
pub const DTripleSharp: Note = DTripleSharpFour;
/// The default A triple sharp (in the fourth octave).
pub const ATripleSharp: Note = ATripleSharpFour;
/// The default E triple sharp (in the fourth octave).
pub const ETripleSharp: Note = ETripleSharpFour;
/// The default B triple sharp (in the fourth octave).
pub const BTripleSharp: Note = BTripleSharpFour;

// Statics.

/// All the notes in all octaves.
pub static ALL_PITCH_NOTES: LazyLock<[Note; 192]> = LazyLock::new(|| {
    let mut all_notes = Vec::with_capacity(132);

    for octave in ALL_OCTAVES.iter() {
        for pitch in ALL_PITCHES.iter() {
            all_notes.push(Note {
                octave: *octave,
                named_pitch: pitch.into(),
            });
        }
    }

    all_notes.try_into().unwrap()
});

/// All the notes in all octaves with their frequency.
pub static ALL_PITCH_NOTES_WITH_FREQUENCY: LazyLock<[(Note, f32); 192]> = LazyLock::new(|| {
    let mut all_notes = Vec::with_capacity(132);

    for note in ALL_PITCH_NOTES.iter() {
        all_notes.push((*note, note.frequency()));
    }

    all_notes.try_into().unwrap()
});

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_text() {
        assert_eq!(CFlat.static_name(), "C♭");
        assert_eq!(C.to_string(), "C4");
    }

    #[test]
    fn test_intervals() {
        // Additions.

        assert_eq!(C + Interval::PerfectUnison, C);
        assert_eq!(C + Interval::DiminishedSecond, DDoubleFlat);

        assert_eq!(C + Interval::AugmentedUnison, CSharp);
        assert_eq!(C + Interval::MinorSecond, DFlat);

        assert_eq!(C + Interval::MajorSecond, D);
        assert_eq!(C + Interval::DiminishedThird, EDoubleFlat);

        assert_eq!(C + Interval::AugmentedSecond, DSharp);
        assert_eq!(C + Interval::MinorThird, EFlat);

        assert_eq!(C + Interval::MajorThird, E);
        assert_eq!(C + Interval::DiminishedFourth, FFlat);

        assert_eq!(C + Interval::AugmentedThird, ESharp);
        assert_eq!(C + Interval::PerfectFourth, F);

        assert_eq!(C + Interval::AugmentedFourth, FSharp);
        assert_eq!(C + Interval::DiminishedFifth, GFlat);

        assert_eq!(C + Interval::PerfectFifth, G);
        assert_eq!(C + Interval::DiminishedSixth, ADoubleFlat);

        assert_eq!(C + Interval::AugmentedFifth, GSharp);
        assert_eq!(C + Interval::MinorSixth, AFlat);

        assert_eq!(C + Interval::MajorSixth, A);
        assert_eq!(C + Interval::DiminishedSeventh, BDoubleFlat);

        assert_eq!(C + Interval::AugmentedSixth, ASharp);
        assert_eq!(C + Interval::MinorSeventh, BFlat);

        assert_eq!(C + Interval::MajorSeventh, B);
        assert_eq!(C + Interval::DiminishedOctave, CFlatFive);

        assert_eq!(C + Interval::AugmentedSeventh, BSharp);
        assert_eq!(C + Interval::PerfectOctave, CFive);

        assert_eq!(C + Interval::PerfectOctave + Interval::PerfectFifth, GFive);

        assert_eq!(C + Interval::MinorNinth, DFlatFive);
        assert_eq!(C + Interval::MajorNinth, DFive);
        assert_eq!(C + Interval::AugmentedNinth, DSharpFive);

        assert_eq!(C + Interval::DiminishedEleventh, FFlatFive);
        assert_eq!(C + Interval::PerfectEleventh, FFive);
        assert_eq!(C + Interval::AugmentedEleventh, FSharpFive);

        assert_eq!(C + Interval::MinorThirteenth, AFlatFive);
        assert_eq!(C + Interval::MajorThirteenth, AFive);
        assert_eq!(C + Interval::AugmentedThirteenth, ASharpFive);

        // Subtractions.

        assert_eq!(C - Interval::PerfectUnison, C);
        assert_eq!(DFlat - Interval::MinorSecond, C);
        assert_eq!(G - Interval::PerfectOctave, GThree);
        assert_eq!(G - Interval::PerfectFifth, C);
        assert_eq!(DFlat - Interval::DiminishedSecond, CSharp);
        assert_eq!(ATripleSharpSix - Interval::TwoPerfectOctaves, ATripleSharp);
        assert_eq!(GFlat - Interval::PerfectFifth, CFlat);
        assert_eq!(GDoubleFlat - Interval::PerfectFifth, CDoubleFlat);

        // Special cases to check.

        assert_eq!(C + Interval::DiminishedOctave, CFlatFive);
        assert_eq!(BFlat + Interval::MinorNinth, CFlatSix);
        assert_eq!(BFlatThree + Interval::MinorNinth, CFlatFive);
        assert_eq!(A + Interval::AugmentedNinth, BSharpFive);
        assert_eq!(CSharp + Interval::AugmentedSeventh, BDoubleSharp);

        assert_eq!(DTripleFlat + Interval::PerfectOctave, DTripleFlatFive);
        assert_eq!(DTripleFlat + Interval::PerfectUnison, DTripleFlat);

        assert_eq!(BSharp + Interval::PerfectOctave, BSharpFive);
        assert_eq!(BSharp + Interval::PerfectUnison, BSharp);

        assert_eq!(ATripleSharp + Interval::TwoPerfectOctaves, ATripleSharpSix);
    }

    #[test]
    fn test_distances() {
        assert_eq!(C - C, Interval::PerfectUnison);
        assert_eq!(C - D, Interval::MajorSecond);
        assert_eq!(D - C, Interval::MajorSecond);
        assert_eq!(C - E, Interval::MajorThird);
    }

    #[test]
    fn test_parse() {
        assert_eq!(Note::parse("C").unwrap(), C);
        assert_eq!(Note::parse("C#").unwrap(), CSharp);
        assert_eq!(Note::parse("Bb3").unwrap(), BFlatThree);
        assert_eq!(Note::parse("D#7").unwrap(), DSharpSeven);
    }

    #[test]
    #[should_panic]
    fn test_parse_panic() {
        assert_eq!(Note::parse("C11").unwrap(), C);
    }

    #[test]
    fn test_pitch() {
        assert_eq!(Note::new(NamedPitch::C, Octave::Four).frequency(), (CThree + Interval::PerfectOctave).frequency());
        assert_eq!(CFlatFour.frequency(), BThree.frequency());
        assert_eq!(BSharp.frequency(), CFive.frequency());
        assert_eq!(DTripleFlatFive.frequency(), B.frequency());
        assert_eq!(BDoubleSharpFive.with_named_pitch(NamedPitch::A).frequency(), AFive.frequency());
    }

    #[test]
    fn test_harmonics() {
        assert_eq!(
            C.primary_harmonic_series(),
            vec![CFive, GFive, CSix, ESix, GSix, BFlatSix, DSeven, ESeven, FSharpSeven, GSeven, AFlatSeven, BFlatSeven, BSeven]
        );
    }

    #[test]
    fn test_id() {
        // Individual notes.

        assert_eq!(CZero.id(), 1 << 0);
        assert_eq!(CSharpZero.id(), 1 << 1);
        assert_eq!(BZero.id(), 1 << 11);
        assert_eq!(Note::parse("C1").unwrap().id(), 1 << 12);
        assert_eq!(Note::parse("C#1").unwrap().id(), 1 << 13);
        assert_eq!(Note::parse("Db1").unwrap().id(), 1 << 13);
        assert_eq!(Note::parse("C4").unwrap().id(), 1 << 48);

        assert_eq!(Note::from_id(1 << 0).unwrap(), CZero);
        assert_eq!(Note::from_id(1 << 1).unwrap(), DFlatZero);
        assert_eq!(Note::from_id(1 << 11).unwrap(), BZero);
        assert_eq!(Note::from_id(1 << 12).unwrap(), Note::parse("C1").unwrap());
        assert_eq!(Note::from_id(1 << 13).unwrap(), Note::parse("Db1").unwrap());
        assert_eq!(Note::from_id(1 << 48).unwrap(), Note::parse("C4").unwrap());

        // Chords.

        assert_eq!(Note::id_mask(&[CZero, CSharpZero]), 1 << 0 | 1 << 1);
        assert_eq!(Note::id_mask(&[CZero, CSharpZero, DFlatZero]), 1 << 0 | 1 << 1);
        assert_eq!(Note::id_mask(&[CZero, CSharpZero, BZero]), 1 << 0 | 1 << 1 | 1 << 11);

        assert_eq!(Note::from_id_mask(1 << 0 | 1 << 1).unwrap(), vec![CZero, DFlatZero]);
        assert_eq!(Note::from_id_mask(1 << 0 | 1 << 1 | 1 << 11).unwrap(), vec![CZero, DFlatZero, BZero]);
        assert_eq!(Note::from_id_mask(1 << 13 | 1 << 48).unwrap(), vec![DFlatOne, CFour]);
    }

    #[test]
    fn test_universal() {
        assert_eq!(FSharpFive.to_universal(), Note::parse("Gb5").unwrap());
    }
}
