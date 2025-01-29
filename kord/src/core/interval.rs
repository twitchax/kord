//! A module for working with intervals.

use std::fmt::{Display, Error, Formatter};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::octave::{HasOctave, Octave};

// Traits.

/// A trait for types that have an enharmonic distance.
pub trait HasEnharmonicDistance {
    /// Returns the enharmonic distance of the type (most likely an interval).
    ///
    /// Due to the nature of enharmonic intervals, the distance is always an integer,
    /// and it looks a bit funky.  Basically, using the circle of fifths, the distance
    /// is the number of fifths between the two notes.  For example, a perfect fifth
    /// is 1 fifth away, and a major second is always two fifths away
    ///  (look at the implementation).  This preserves enharmonic correctness.
    fn enharmonic_distance(&self) -> i8;
}

/// A trait for types that can be "reduced" to a single "frame" (usually an interval, and usually within an octave).
pub trait CanReduceFrame {
    /// Returns the reduced frame of the type.
    fn reduce_frame(self) -> Self;
}

// Enum.

/// An enum representing the interval between two notes.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = KordInterval))]
pub enum Interval {
    /// A perfect unison interval.
    PerfectUnison,
    /// A diminished second interval.
    DiminishedSecond,

    /// An augmented unison interval.
    AugmentedUnison,
    /// A minor second interval.
    MinorSecond,

    /// A major second interval.
    MajorSecond,
    /// A diminished third interval.
    DiminishedThird,

    /// An augmented second interval.
    AugmentedSecond,
    /// A minor third interval.
    MinorThird,

    /// A major third interval.
    MajorThird,
    /// A diminished fourth interval.
    DiminishedFourth,

    /// An augmented third interval.
    AugmentedThird,
    /// A perfect fourth interval.
    PerfectFourth,

    /// An augmented fourth interval.
    AugmentedFourth,
    /// A diminished fifth interval.
    DiminishedFifth,

    /// A perfect fifth interval.
    PerfectFifth,
    /// A diminished sixth interval.
    DiminishedSixth,

    /// An augmented fifth interval.
    AugmentedFifth,
    /// A minor sixth interval.
    MinorSixth,

    /// A major sixth interval.
    MajorSixth,
    /// A diminished seventh interval.
    DiminishedSeventh,

    /// An augmented sixth interval.
    AugmentedSixth,
    /// A minor seventh interval.
    MinorSeventh,

    /// A major seventh interval.
    MajorSeventh,
    /// A diminished octave interval.
    DiminishedOctave,

    /// An augmented seventh interval.
    AugmentedSeventh,
    /// A perfect octave interval.
    PerfectOctave,

    /// An minor ninth interval.
    MinorNinth,
    /// A major ninth interval.
    MajorNinth,
    /// An augmented ninth interval.
    AugmentedNinth,

    /// A diminished eleventh interval.
    DiminishedEleventh,
    /// A perfect eleventh interval.
    PerfectEleventh,
    /// An augmented eleventh interval.
    AugmentedEleventh,

    /// A minor thirteenth interval.
    MinorThirteenth,
    /// A major thirteenth interval.
    MajorThirteenth,
    /// An augmented thirteenth interval.
    AugmentedThirteenth,

    /// A perfect octave and perfect fifth interval.
    PerfectOctaveAndPerfectFifth,
    /// Two perfect octaves.
    TwoPerfectOctaves,
    /// Two perfect octaves and a major third.
    TwoPerfectOctavesAndMajorThird,
    /// Two perfect octaves and a perfect fifth.
    TwoPerfectOctavesAndPerfectFifth,
    /// Two perfect octaves and a minor sixth.
    TwoPerfectOctavesAndMinorSeventh,
    /// Three perfect octaves.
    ThreePerfectOctaves,
    /// Three perfect octaves and a major second.
    ThreePerfectOctavesAndMajorSecond,
    /// Three perfect octaves and a major third.
    ThreePerfectOctavesAndMajorThird,
    /// Three perfect octaves and an augmented fourth.
    ThreePerfectOctavesAndAugmentedFourth,
    /// Three perfect octaves and a perfect fifth.
    ThreePerfectOctavesAndPerfectFifth,
    /// Three perfect octaves and a minor sixth.
    ThreePerfectOctavesAndMinorSixth,
    /// Three perfect octaves and a minor seventh.
    ThreePerfectOctavesAndMinorSeventh,
    /// Three perfect octaves and a major seventh.
    ThreePerfectOctavesAndMajorSeventh,
}

// Impls.

impl HasEnharmonicDistance for Interval {
    fn enharmonic_distance(&self) -> i8 {
        match self {
            Interval::PerfectUnison => 0,
            Interval::DiminishedSecond => -12,

            Interval::AugmentedUnison => 7,
            Interval::MinorSecond => -5,

            Interval::MajorSecond => 2,
            Interval::DiminishedThird => -10,

            Interval::AugmentedSecond => 9,
            Interval::MinorThird => -3,

            Interval::MajorThird => 4,
            Interval::DiminishedFourth => -8,

            Interval::AugmentedThird => 11,
            Interval::PerfectFourth => -1,

            Interval::AugmentedFourth => 6,
            Interval::DiminishedFifth => -6,

            Interval::PerfectFifth => 1,
            Interval::DiminishedSixth => -11,

            Interval::AugmentedFifth => 8,
            Interval::MinorSixth => -4,

            Interval::MajorSixth => 3,
            Interval::DiminishedSeventh => -9,

            Interval::AugmentedSixth => 10,
            Interval::MinorSeventh => -2,

            Interval::MajorSeventh => 5,
            Interval::DiminishedOctave => -7,

            Interval::AugmentedSeventh => 12,
            Interval::PerfectOctave => 0,

            Interval::MinorNinth => -5,
            Interval::MajorNinth => 2,
            Interval::AugmentedNinth => 9,

            Interval::DiminishedEleventh => -8,
            Interval::PerfectEleventh => -1,
            Interval::AugmentedEleventh => 6,

            Interval::MinorThirteenth => -4,
            Interval::MajorThirteenth => 3,
            Interval::AugmentedThirteenth => 10,

            Interval::PerfectOctaveAndPerfectFifth => 1,
            Interval::TwoPerfectOctaves => 0,
            Interval::TwoPerfectOctavesAndMajorThird => 4,
            Interval::TwoPerfectOctavesAndPerfectFifth => 1,
            Interval::TwoPerfectOctavesAndMinorSeventh => -2,
            Interval::ThreePerfectOctaves => 0,
            Interval::ThreePerfectOctavesAndMajorSecond => 2,
            Interval::ThreePerfectOctavesAndMajorThird => 4,
            Interval::ThreePerfectOctavesAndAugmentedFourth => 6,
            Interval::ThreePerfectOctavesAndPerfectFifth => 1,
            Interval::ThreePerfectOctavesAndMinorSixth => -4,
            Interval::ThreePerfectOctavesAndMinorSeventh => -2,
            Interval::ThreePerfectOctavesAndMajorSeventh => 5,
        }
    }
}

impl HasOctave for Interval {
    fn octave(&self) -> Octave {
        match self {
            Interval::PerfectUnison => Octave::Zero,
            Interval::DiminishedSecond => Octave::Zero,

            Interval::AugmentedUnison => Octave::Zero,
            Interval::MinorSecond => Octave::Zero,

            Interval::MajorSecond => Octave::Zero,
            Interval::DiminishedThird => Octave::Zero,

            Interval::AugmentedSecond => Octave::Zero,
            Interval::MinorThird => Octave::Zero,

            Interval::MajorThird => Octave::Zero,
            Interval::DiminishedFourth => Octave::Zero,

            Interval::AugmentedThird => Octave::Zero,
            Interval::PerfectFourth => Octave::Zero,

            Interval::AugmentedFourth => Octave::Zero,
            Interval::DiminishedFifth => Octave::Zero,

            Interval::PerfectFifth => Octave::Zero,
            Interval::DiminishedSixth => Octave::Zero,

            Interval::AugmentedFifth => Octave::Zero,
            Interval::MinorSixth => Octave::Zero,

            Interval::MajorSixth => Octave::Zero,
            Interval::DiminishedSeventh => Octave::Zero,

            Interval::AugmentedSixth => Octave::Zero,
            Interval::MinorSeventh => Octave::Zero,

            Interval::MajorSeventh => Octave::Zero,
            Interval::DiminishedOctave => Octave::Zero,

            Interval::AugmentedSeventh => Octave::One,
            Interval::PerfectOctave => Octave::One,

            Interval::MinorNinth => Octave::One,
            Interval::MajorNinth => Octave::One,
            Interval::AugmentedNinth => Octave::One,

            Interval::DiminishedEleventh => Octave::One,
            Interval::PerfectEleventh => Octave::One,
            Interval::AugmentedEleventh => Octave::One,

            Interval::MinorThirteenth => Octave::One,
            Interval::MajorThirteenth => Octave::One,
            Interval::AugmentedThirteenth => Octave::One,

            Interval::PerfectOctaveAndPerfectFifth => Octave::One,
            Interval::TwoPerfectOctaves => Octave::Two,
            Interval::TwoPerfectOctavesAndMajorThird => Octave::Two,
            Interval::TwoPerfectOctavesAndPerfectFifth => Octave::Two,
            Interval::TwoPerfectOctavesAndMinorSeventh => Octave::Two,
            Interval::ThreePerfectOctaves => Octave::Three,
            Interval::ThreePerfectOctavesAndMajorSecond => Octave::Three,
            Interval::ThreePerfectOctavesAndMajorThird => Octave::Three,
            Interval::ThreePerfectOctavesAndAugmentedFourth => Octave::Three,
            Interval::ThreePerfectOctavesAndPerfectFifth => Octave::Three,
            Interval::ThreePerfectOctavesAndMinorSixth => Octave::Three,
            Interval::ThreePerfectOctavesAndMinorSeventh => Octave::Three,
            Interval::ThreePerfectOctavesAndMajorSeventh => Octave::Three,
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Interval::PerfectUnison => write!(f, "PerfectUnison"),
            Interval::DiminishedSecond => write!(f, "DiminishedSecond"),

            Interval::AugmentedUnison => write!(f, "AugmentedUnison"),
            Interval::MinorSecond => write!(f, "MinorSecond"),

            Interval::MajorSecond => write!(f, "MajorSecond"),
            Interval::DiminishedThird => write!(f, "DiminishedThird"),

            Interval::AugmentedSecond => write!(f, "AugmentedSecond"),
            Interval::MinorThird => write!(f, "MinorThird"),

            Interval::MajorThird => write!(f, "MajorThird"),
            Interval::DiminishedFourth => write!(f, "DiminishedFourth"),

            Interval::AugmentedThird => write!(f, "AugmentedThird"),
            Interval::PerfectFourth => write!(f, "PerfectFourth"),

            Interval::AugmentedFourth => write!(f, "AugmentedFourth"),
            Interval::DiminishedFifth => write!(f, "DiminishedFifth"),

            Interval::PerfectFifth => write!(f, "PerfectFifth"),
            Interval::DiminishedSixth => write!(f, "DiminishedSixth"),

            Interval::AugmentedFifth => write!(f, "AugmentedFifth"),
            Interval::MinorSixth => write!(f, "MinorSixth"),

            Interval::MajorSixth => write!(f, "MajorSixth"),
            Interval::DiminishedSeventh => write!(f, "DiminishedSeventh"),

            Interval::AugmentedSixth => write!(f, "AugmentedSixth"),
            Interval::MinorSeventh => write!(f, "MinorSeventh"),

            Interval::MajorSeventh => write!(f, "MajorSeventh"),
            Interval::DiminishedOctave => write!(f, "DiminishedOctave"),

            Interval::AugmentedSeventh => write!(f, "AugmentedSeventh"),
            Interval::PerfectOctave => write!(f, "PerfectOctave"),

            Interval::MinorNinth => write!(f, "MinorNinth"),
            Interval::MajorNinth => write!(f, "MajorNinth"),
            Interval::AugmentedNinth => write!(f, "AugmentedNinth"),

            Interval::DiminishedEleventh => write!(f, "DiminishedEleventh"),
            Interval::PerfectEleventh => write!(f, "PerfectEleventh"),
            Interval::AugmentedEleventh => write!(f, "AugmentedEleventh"),

            Interval::MinorThirteenth => write!(f, "MinorThirteenth"),
            Interval::MajorThirteenth => write!(f, "MajorThirteenth"),
            Interval::AugmentedThirteenth => write!(f, "AugmentedThirteenth"),

            Interval::PerfectOctaveAndPerfectFifth => write!(f, "PerfectOctaveAndPerfectFifth"),
            Interval::TwoPerfectOctaves => write!(f, "TwoPerfectOctaves"),
            Interval::TwoPerfectOctavesAndMajorThird => write!(f, "TwoPerfectOctavesAndMajorThird"),
            Interval::TwoPerfectOctavesAndPerfectFifth => write!(f, "TwoPerfectOctavesAndPerfectFifth"),
            Interval::TwoPerfectOctavesAndMinorSeventh => write!(f, "TwoPerfectOctavesAndMinorSeventh"),
            Interval::ThreePerfectOctaves => write!(f, "ThreePerfectOctaves"),
            Interval::ThreePerfectOctavesAndMajorSecond => write!(f, "ThreePerfectOctavesAndMajorSecond"),
            Interval::ThreePerfectOctavesAndMajorThird => write!(f, "ThreePerfectOctavesAndMajorThird"),
            Interval::ThreePerfectOctavesAndAugmentedFourth => write!(f, "ThreePerfectOctavesAndAugmentedFourth"),
            Interval::ThreePerfectOctavesAndPerfectFifth => write!(f, "ThreePerfectOctavesAndPerfectFifth"),
            Interval::ThreePerfectOctavesAndMinorSixth => write!(f, "ThreePerfectOctavesAndMinorSixth"),
            Interval::ThreePerfectOctavesAndMinorSeventh => write!(f, "ThreePerfectOctavesAndMinorSeventh"),
            Interval::ThreePerfectOctavesAndMajorSeventh => write!(f, "ThreePerfectOctavesAndMajorSeventh"),
        }
    }
}

// Statics.

/// All known [`Interval`]s.
pub static ALL_INTERVALS: [Interval; 48] = [
    Interval::PerfectUnison,
    Interval::DiminishedSecond,
    Interval::AugmentedUnison,
    Interval::MinorSecond,
    Interval::MajorSecond,
    Interval::DiminishedThird,
    Interval::AugmentedSecond,
    Interval::MinorThird,
    Interval::MajorThird,
    Interval::DiminishedFourth,
    Interval::AugmentedThird,
    Interval::PerfectFourth,
    Interval::AugmentedFourth,
    Interval::DiminishedFifth,
    Interval::PerfectFifth,
    Interval::DiminishedSixth,
    Interval::AugmentedFifth,
    Interval::MinorSixth,
    Interval::MajorSixth,
    Interval::DiminishedSeventh,
    Interval::AugmentedSixth,
    Interval::MinorSeventh,
    Interval::MajorSeventh,
    Interval::DiminishedOctave,
    Interval::AugmentedSeventh,
    Interval::PerfectOctave,
    Interval::MinorNinth,
    Interval::MajorNinth,
    Interval::AugmentedNinth,
    Interval::DiminishedEleventh,
    Interval::PerfectEleventh,
    Interval::AugmentedEleventh,
    Interval::MinorThirteenth,
    Interval::MajorThirteenth,
    Interval::AugmentedThirteenth,
    Interval::PerfectOctaveAndPerfectFifth,
    Interval::TwoPerfectOctaves,
    Interval::TwoPerfectOctavesAndMajorThird,
    Interval::TwoPerfectOctavesAndPerfectFifth,
    Interval::TwoPerfectOctavesAndMinorSeventh,
    Interval::ThreePerfectOctaves,
    Interval::ThreePerfectOctavesAndMajorSecond,
    Interval::ThreePerfectOctavesAndMajorThird,
    Interval::ThreePerfectOctavesAndAugmentedFourth,
    Interval::ThreePerfectOctavesAndPerfectFifth,
    Interval::ThreePerfectOctavesAndMinorSixth,
    Interval::ThreePerfectOctavesAndMinorSeventh,
    Interval::ThreePerfectOctavesAndMajorSeventh,
];

/// The primary (first 13) harmonic series expressed as [`Interval`]s.
pub static PRIMARY_HARMONIC_SERIES: [Interval; 13] = [
    Interval::PerfectOctave,
    Interval::PerfectOctaveAndPerfectFifth,
    Interval::TwoPerfectOctaves,
    Interval::TwoPerfectOctavesAndMajorThird,
    Interval::TwoPerfectOctavesAndPerfectFifth,
    Interval::TwoPerfectOctavesAndMinorSeventh,
    Interval::ThreePerfectOctavesAndMajorSecond,
    Interval::ThreePerfectOctavesAndMajorThird,
    Interval::ThreePerfectOctavesAndAugmentedFourth,
    Interval::ThreePerfectOctavesAndPerfectFifth,
    Interval::ThreePerfectOctavesAndMinorSixth,
    Interval::ThreePerfectOctavesAndMinorSeventh,
    Interval::ThreePerfectOctavesAndMajorSeventh,
];
