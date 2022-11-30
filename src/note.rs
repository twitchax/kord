#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::ops::{Add, AddAssign};

use paste::paste;
use crate::{named_pitch::{NamedPitch}, interval::{Interval, HasDistance}, base::HasStaticName, chord::Chord};

// Macros.

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

macro_rules! define_octave {
    ($octave_num:ident, $octave:expr) => {
        define_note!(BSharp, NamedPitch::BSharp, $octave_num, $octave);
        define_note!(C, NamedPitch::C, $octave_num, $octave);
        define_note!(DDoubleFlat, NamedPitch::DDoubleFlat, $octave_num, $octave);
        define_note!(EQuadFlat, NamedPitch::EQuadFlat, $octave_num, $octave);

        define_note!(BDoubleSharp, NamedPitch::BDoubleSharp, $octave_num, $octave);
        define_note!(CSharp, NamedPitch::CSharp, $octave_num, $octave);
        define_note!(DFlat, NamedPitch::DFlat, $octave_num, $octave);
        define_note!(ETripleFlat, NamedPitch::ETripleFlat, $octave_num, $octave);

        define_note!(CDoubleSharp, NamedPitch::CDoubleSharp, $octave_num, $octave);
        define_note!(D, NamedPitch::D, $octave_num, $octave);
        define_note!(EDoubleFlat, NamedPitch::EDoubleFlat, $octave_num, $octave);
        define_note!(FTripleFlat, NamedPitch::FTripleFlat, $octave_num, $octave);

        define_note!(CTripleSharp, NamedPitch::CTripleSharp, $octave_num, $octave);
        define_note!(DSharp, NamedPitch::DSharp, $octave_num, $octave);
        define_note!(EFlat, NamedPitch::EFlat, $octave_num, $octave);
        define_note!(FDoubleFlat, NamedPitch::FDoubleFlat, $octave_num, $octave);

        define_note!(DDoubleSharp, NamedPitch::DDoubleSharp, $octave_num, $octave);
        define_note!(E, NamedPitch::E, $octave_num, $octave);
        define_note!(FFlat, NamedPitch::FFlat, $octave_num, $octave);
        define_note!(GTripleFlat, NamedPitch::GTripleFlat, $octave_num, $octave);

        define_note!(ESharp, NamedPitch::ESharp, $octave_num, $octave);
        define_note!(F, NamedPitch::F, $octave_num, $octave);
        define_note!(GDoubleFlat, NamedPitch::GDoubleFlat, $octave_num, $octave);
        define_note!(AQuadFlat, NamedPitch::AQuadFlat, $octave_num, $octave);

        define_note!(EDoubleSharp, NamedPitch::EDoubleSharp, $octave_num, $octave);
        define_note!(FSharp, NamedPitch::FSharp, $octave_num, $octave);
        define_note!(GFlat, NamedPitch::GFlat, $octave_num, $octave);
        define_note!(ATripleFlat, NamedPitch::ATripleFlat, $octave_num, $octave);

        define_note!(FDoubleSharp, NamedPitch::FDoubleSharp, $octave_num, $octave);
        define_note!(G, NamedPitch::G, $octave_num, $octave);
        define_note!(ADoubleFlat, NamedPitch::ADoubleFlat, $octave_num, $octave);
        define_note!(BQuadFlat, NamedPitch::BQuadFlat, $octave_num, $octave);

        define_note!(FTripleSharp, NamedPitch::FTripleSharp, $octave_num, $octave);
        define_note!(GSharp, NamedPitch::GSharp, $octave_num, $octave);
        define_note!(AFlat, NamedPitch::AFlat, $octave_num, $octave);
        define_note!(BTripleFlat, NamedPitch::BTripleFlat, $octave_num, $octave);

        define_note!(GDoubleSharp, NamedPitch::GDoubleSharp, $octave_num, $octave);
        define_note!(A, NamedPitch::A, $octave_num, $octave);
        define_note!(BDoubleFlat, NamedPitch::BDoubleFlat, $octave_num, $octave);
        define_note!(CTripleFlat, NamedPitch::CTripleFlat, $octave_num, $octave);

        define_note!(GTripleSharp, NamedPitch::GTripleSharp, $octave_num, $octave);
        define_note!(ASharp, NamedPitch::ASharp, $octave_num, $octave);
        define_note!(BFlat, NamedPitch::BFlat, $octave_num, $octave);
        define_note!(CDoubleFlat, NamedPitch::CDoubleFlat, $octave_num, $octave);

        define_note!(ADoubleSharp, NamedPitch::ADoubleSharp, $octave_num, $octave);
        define_note!(B, NamedPitch::B, $octave_num, $octave);
        define_note!(CFlat, NamedPitch::CFlat, $octave_num, $octave);
        define_note!(DTripleFlat, NamedPitch::DTripleFlat, $octave_num, $octave);
    };
}

// Traits.

pub trait IntoChord {
    fn into_chord(self) -> Chord;
}

// Struct.

use crate::{pitch::{Pitch, HasPitch}, octave::{Octave, HasOctave}};

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, PartialOrd, Ord)]
pub struct Note {
    octave: Octave,
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

impl IntoChord for Note {
    fn into_chord(self) -> Chord {
        Chord::new(self)
    }
}

impl Add<Interval> for Note {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self::Output {
        let mut i = self.named_pitch.iter();

        for _ in 0..rhs.distance() {
            // SAFETY: The iterator is guaranteed to have at least `rhs.distance()` elements.
            i = i.next().unwrap();
        }

        Note {
            octave: self.octave + i.octaves,
            named_pitch: i.current,
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

pub const BSharp: Note = BSharpFour;
pub const C: Note = CFour;
pub const DDoubleFlat: Note = DDoubleFlatFour;
pub const EQuadFlat: Note = EQuadFlatFour;

pub const BDoubleSharp: Note = BDoubleSharpFour;
pub const CSharp: Note = CSharpFour;
pub const DFlat: Note = DFlatFour;
pub const ETripleFlat: Note = ETripleFlatFour;

pub const CDoubleSharp: Note = CDoubleSharpFour;
pub const D: Note = DFour;
pub const EDoubleFlat: Note = EDoubleFlatFour;
pub const FTripleFlat: Note = FTripleFlatFour;

pub const CTripleSharp: Note = CTripleSharpFour;
pub const DSharp: Note = DSharpFour;
pub const EFlat: Note = EFlatFour;
pub const FDoubleFlat: Note = FDoubleFlatFour;

pub const DDoubleSharp: Note = DDoubleSharpFour;
pub const E: Note = EFour;
pub const FFlat: Note = FFlatFour;
pub const GTripleFlat: Note = GTripleFlatFour;

pub const ESharp: Note = ESharpFour;
pub const F: Note = FFour;
pub const GDoubleFlat: Note = GDoubleFlatFour;
pub const ATripleFlat: Note = ATripleFlatFour;

pub const EDoubleSharp: Note = EDoubleSharpFour;
pub const FSharp: Note = FSharpFour;
pub const GFlat: Note = GFlatFour;
pub const AQuadFlat: Note = AQuadFlatFour;

pub const FDoubleSharp: Note = FDoubleSharpFour;
pub const G: Note = GFour;
pub const ADoubleFlat: Note = ADoubleFlatFour;
pub const BTripleFlat: Note = BTripleFlatFour;

pub const FTripleSharp: Note = FTripleSharpFour;
pub const GSharp: Note = GSharpFour;
pub const AFlat: Note = AFlatFour;
pub const BQuadFlat: Note = BQuadFlatFour;

pub const GDoubleSharp: Note = GDoubleSharpFour;
pub const A: Note = AFour;
pub const BDoubleFlat: Note = BDoubleFlatFour;
pub const CTripleFlat: Note = CTripleFlatFour;

pub const GTripleSharp: Note = GTripleSharpFour;
pub const ASharp: Note = ASharpFour;
pub const BFlat: Note = BFlatFour;
pub const CDoubleFlat: Note = CDoubleFlatFour;

pub const ADoubleSharp: Note = ADoubleSharpFour;
pub const B: Note = BFour;
pub const CFlat: Note = CFlatFour;
pub const DTripleFlat: Note = DTripleFlatFour;

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
        assert_eq!(C + Interval::DiminishedOctave, CFlat);

        assert_eq!(C + Interval::AugmentedSeventh, BSharpFive);
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
    }
}