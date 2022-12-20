#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::ops::{Add, AddAssign};

use paste::paste;
use crate::{named_pitch::{NamedPitch, HasNamedPitch}, interval::{Interval, HasEnharmonicDistance}, base::HasStaticName, chord::Chord, pitch::{HasFrequency, HasBaseFrequency, Pitch, HasPitch}, octave::{Octave, HasOctave}};

// Macros.

/// Defines a note from a [`NamedPitch`].
macro_rules! define_note {
    ( $name:ident, $named_pitch:expr, $octave_num:ident, $octave:expr) => {
        paste! {
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

// Struct.

/// A note type.
/// 
/// This is a named pitch with an octave.  This type allows for correctly attributing octave changes
/// across an interval from one [`Note`] to another.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, PartialOrd, Ord)]
pub struct Note {
    /// The octave of the note.
    octave: Octave,
    /// The named pitch of the note.
    named_pitch: NamedPitch,
}

// Impls.

impl Note {
    pub fn new(pitch: NamedPitch, octave: Octave) -> Self {
        Self { named_pitch: pitch, octave }
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

impl HasFrequency for Note
{
    fn frequency(&self) -> f32 {
        let mut octave = self.octave();
        let base_frequency = self.pitch().base_frequency();

        match self.named_pitch {
            NamedPitch::ATripleSharp | NamedPitch::BTripleSharp | NamedPitch::BDoubleSharp | NamedPitch::BSharp => {
                octave += 1;
            },
            NamedPitch::DTripleFlat | NamedPitch::CTripleFlat | NamedPitch::CDoubleFlat | NamedPitch::CFlat => {
                octave -= 1;
            },
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

impl Add<Interval> for Note {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self::Output {
        let new_pitch = self.named_pitch() + rhs.enharmonic_distance();

        // Compute whether or not we "crossed" an octave.
        let wrapping_octave = if new_pitch.pitch() < self.pitch() {
            Octave::One
        } else {
            Octave::Zero
        };

        // There is a "special wrap" for `Cb`, and `Dbbb`, since they don't technically loop; and, for B#, etc., on the other side.
        let special_octave = if new_pitch == NamedPitch::CFlat || new_pitch == NamedPitch::DTripleFlat {
            1
        } else if new_pitch == NamedPitch::BSharp || new_pitch == NamedPitch::BDoubleSharp || new_pitch == NamedPitch::BTripleSharp || new_pitch == NamedPitch::ATripleSharp {
            -1
        } else {
            0
        };

        // Get whether or not the interval itself contains an octave.
        let interval_octave = rhs.octave();

        Note {
            octave: self.octave + wrapping_octave + special_octave + interval_octave,
            named_pitch: new_pitch,
        }
    }
}

impl AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
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

pub const FTripleFlat: Note = FTripleFlatFour;
pub const CTripleFlat: Note = CTripleFlatFour;
pub const GTripleFlat: Note = GTripleFlatFour;
pub const DTripleFlat: Note = DTripleFlatFour;
pub const ATripleFlat: Note = ATripleFlatFour;
pub const ETripleFlat: Note = ETripleFlatFour;
pub const BTripleFlat: Note = BTripleFlatFour;

pub const FDoubleFlat: Note = FDoubleFlatFour;
pub const CDoubleFlat: Note = CDoubleFlatFour;
pub const GDoubleFlat: Note = GDoubleFlatFour;
pub const DDoubleFlat: Note = DDoubleFlatFour;
pub const ADoubleFlat: Note = ADoubleFlatFour;
pub const EDoubleFlat: Note = EDoubleFlatFour;
pub const BDoubleFlat: Note = BDoubleFlatFour;

pub const FFlat: Note = FFlatFour;
pub const CFlat: Note = CFlatFour;
pub const GFlat: Note = GFlatFour;
pub const DFlat: Note = DFlatFour;
pub const AFlat: Note = AFlatFour;
pub const EFlat: Note = EFlatFour;
pub const BFlat: Note = BFlatFour;

pub const F: Note = FFour;
pub const C: Note = CFour;
pub const G: Note = GFour;
pub const D: Note = DFour;
pub const A: Note = AFour;
pub const E: Note = EFour;
pub const B: Note = BFour;

pub const FSharp: Note = FSharpFour;
pub const CSharp: Note = CSharpFour;
pub const GSharp: Note = GSharpFour;
pub const DSharp: Note = DSharpFour;
pub const ASharp: Note = ASharpFour;
pub const ESharp: Note = ESharpFour;
pub const BSharp: Note = BSharpFour;

pub const FDoubleSharp: Note = FDoubleSharpFour;
pub const CDoubleSharp: Note = CDoubleSharpFour;
pub const GDoubleSharp: Note = GDoubleSharpFour;
pub const DDoubleSharp: Note = DDoubleSharpFour;
pub const ADoubleSharp: Note = ADoubleSharpFour;
pub const EDoubleSharp: Note = EDoubleSharpFour;
pub const BDoubleSharp: Note = BDoubleSharpFour;

pub const FTripleSharp: Note = FTripleSharpFour;
pub const CTripleSharp: Note = CTripleSharpFour;
pub const GTripleSharp: Note = GTripleSharpFour;
pub const DTripleSharp: Note = DTripleSharpFour;
pub const ATripleSharp: Note = ATripleSharpFour;
pub const ETripleSharp: Note = ETripleSharpFour;
pub const BTripleSharp: Note = BTripleSharpFour;

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};

    #[test]
    fn test_intervals() {
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

        // Special cases to check.
        assert_eq!(C + Interval::DiminishedOctave, CFlatFive);
        assert_eq!(BFlat + Interval::MinorNinth, CFlatSix);
        assert_eq!(BFlatThree + Interval::MinorNinth, CFlatFive);
        assert_eq!(A + Interval::AugmentedNinth, BSharpFive);
        assert_eq!(CSharp + Interval::AugmentedSeventh, BDoubleSharp);
    }

    #[test]
    fn test_pitch() {
        assert_eq!(C.frequency(), (CThree + Interval::PerfectOctave).frequency());
        assert_eq!(CFlatFour.frequency(), BThree.frequency());
        assert_eq!(BSharp.frequency(), CFive.frequency());
        assert_eq!(DTripleFlatFive.frequency(), B.frequency());
    }
}