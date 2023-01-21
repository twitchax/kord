use once_cell::sync::Lazy;

use crate::octave::{HasOctave, Octave};

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
#[repr(u8)]
pub enum Interval {
    PerfectUnison,
    DiminishedSecond,

    AugmentedUnison,
    MinorSecond,

    MajorSecond,
    DiminishedThird,

    AugmentedSecond,
    MinorThird,

    MajorThird,
    DiminishedFourth,

    AugmentedThird,
    PerfectFourth,

    AugmentedFourth,
    DiminishedFifth,

    PerfectFifth,
    DiminishedSixth,

    AugmentedFifth,
    MinorSixth,

    MajorSixth,
    DiminishedSeventh,

    AugmentedSixth,
    MinorSeventh,

    MajorSeventh,
    DiminishedOctave,

    AugmentedSeventh,
    PerfectOctave,

    MinorNinth,
    MajorNinth,
    AugmentedNinth,

    DiminishedEleventh,
    PerfectEleventh,
    AugmentedEleventh,

    MinorThirteenth,
    MajorThirteenth,
    AugmentedThirteenth,

    PerfectOctaveAndPerfectFifth,
    TwoPerfectOctaves,
    TwoPerfectOctavesAndMajorThird,
    TwoPerfectOctavesAndPerfectFifth,
    TwoPerfectOctavesAndMinorSeventh,
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
        }
    }
}

// Statics.

pub(crate) static PRIMARY_HARMONIC_SERIES: Lazy<[Interval; 6]> = Lazy::new(|| {
    [
        Interval::PerfectOctave,
        Interval::PerfectOctaveAndPerfectFifth,
        Interval::TwoPerfectOctaves,
        Interval::TwoPerfectOctavesAndMajorThird,
        Interval::TwoPerfectOctavesAndPerfectFifth,
        Interval::TwoPerfectOctavesAndMinorSeventh,
    ]
});
