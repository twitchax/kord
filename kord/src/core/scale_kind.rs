//! A module for working with scale kinds.

use crate::core::{
    base::{HasDescription, HasName, HasStaticName},
    interval::{HasIntervals, Interval},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Enum.

/// An enum representing a scale kind (type of scale).
///
/// Each scale kind has an explicit list of intervals that define the scale.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ScaleKind {
    /// A major scale (Ionian mode root scale).
    Major,
    /// A natural minor scale (Aeolian mode root scale).
    NaturalMinor,
    /// A harmonic minor scale.
    HarmonicMinor,
    /// A melodic minor scale (ascending).
    MelodicMinor,
    /// A whole tone scale.
    WholeTone,
    /// A chromatic scale (all 12 semitones).
    Chromatic,
    /// A diminished (whole-half) scale.
    DiminishedWholeHalf,
    /// A diminished (half-whole) scale.
    DiminishedHalfWhole,
    /// A major pentatonic scale.
    MajorPentatonic,
    /// A minor pentatonic scale.
    MinorPentatonic,
    /// A blues scale.
    Blues,
}

// Impls.

impl HasIntervals for ScaleKind {
    fn intervals(&self) -> &'static [Interval] {
        match self {
            ScaleKind::Major => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            ScaleKind::NaturalMinor => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            ScaleKind::HarmonicMinor => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MajorSeventh,
            ],
            ScaleKind::MelodicMinor => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            ScaleKind::WholeTone => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::AugmentedFifth,
                Interval::AugmentedSixth,
            ],
            ScaleKind::Chromatic => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
                Interval::MajorSeventh,
            ],
            ScaleKind::DiminishedWholeHalf => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::DiminishedSeventh,
                Interval::MajorSeventh,
            ],
            ScaleKind::DiminishedHalfWhole => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            // Major Pentatonic: 1, 2, 3, 5, 6 (no 4th or 7th)
            ScaleKind::MajorPentatonic => &[Interval::PerfectUnison, Interval::MajorSecond, Interval::MajorThird, Interval::PerfectFifth, Interval::MajorSixth],
            // Minor Pentatonic: 1, ♭3, 4, 5, ♭7 (no 2nd or 6th)
            ScaleKind::MinorPentatonic => &[Interval::PerfectUnison, Interval::MinorThird, Interval::PerfectFourth, Interval::PerfectFifth, Interval::MinorSeventh],
            // Blues: 1, ♭3, 4, ♯4, 5, ♭7 (minor pentatonic + ♯4)
            ScaleKind::Blues => &[
                Interval::PerfectUnison,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::AugmentedFourth, // #4 blue note – chromatic passing tone between the 4th and 5th
                Interval::PerfectFifth,
                Interval::MinorSeventh,
            ],
        }
    }
}

impl HasDescription for ScaleKind {
    fn description(&self) -> &'static str {
        match self {
            ScaleKind::Major => "major scale, ionian mode parent",
            ScaleKind::NaturalMinor => "natural minor scale, aeolian mode parent",
            ScaleKind::HarmonicMinor => "harmonic minor scale, raised seventh degree",
            ScaleKind::MelodicMinor => "melodic minor scale, raised sixth and seventh degrees",
            ScaleKind::WholeTone => "whole tone scale, all whole steps",
            ScaleKind::Chromatic => "chromatic scale, all twelve semitones",
            ScaleKind::DiminishedWholeHalf => "diminished scale, whole-half (W-H) pattern, fully diminished 7th chord parent",
            ScaleKind::DiminishedHalfWhole => "diminished scale, half-whole (H-W) pattern, dominant 7♭9 (flat 9) chord parent",
            ScaleKind::MajorPentatonic => "major pentatonic scale, five-note major scale without 4th and 7th",
            ScaleKind::MinorPentatonic => "minor pentatonic scale, five-note minor scale without 2nd and 6th",
            ScaleKind::Blues => "blues scale, minor pentatonic with added ♯4 (blue note)",
        }
    }
}

impl HasStaticName for ScaleKind {
    fn static_name(&self) -> &'static str {
        match self {
            ScaleKind::Major => "major",
            ScaleKind::NaturalMinor => "natural minor",
            ScaleKind::HarmonicMinor => "harmonic minor",
            ScaleKind::MelodicMinor => "melodic minor",
            ScaleKind::WholeTone => "whole tone",
            ScaleKind::Chromatic => "chromatic",
            ScaleKind::DiminishedWholeHalf => "diminished (whole-half)",
            ScaleKind::DiminishedHalfWhole => "diminished (half-whole)",
            ScaleKind::MajorPentatonic => "major pentatonic",
            ScaleKind::MinorPentatonic => "minor pentatonic",
            ScaleKind::Blues => "blues",
        }
    }
}

impl HasName for ScaleKind {
    fn name(&self) -> String {
        self.static_name().to_owned()
    }
}

impl ScaleKind {
    /// Returns the ASCII name of the scale (using 'b', '#', 'nat' instead of Unicode symbols).
    pub fn ascii_name(&self) -> &'static str {
        match self {
            ScaleKind::Major => "major",
            ScaleKind::NaturalMinor => "natural minor",
            ScaleKind::HarmonicMinor => "harmonic minor",
            ScaleKind::MelodicMinor => "melodic minor",
            ScaleKind::WholeTone => "whole tone",
            ScaleKind::Chromatic => "chromatic",
            ScaleKind::DiminishedWholeHalf => "diminished whole-half",
            ScaleKind::DiminishedHalfWhole => "diminished half-whole",
            ScaleKind::MajorPentatonic => "major pentatonic",
            ScaleKind::MinorPentatonic => "minor pentatonic",
            ScaleKind::Blues => "blues",
        }
    }
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_scale_intervals() {
        // Major scale: W-W-H-W-W-W-H
        assert_eq!(ScaleKind::Major.intervals().len(), 7);
        assert_eq!(
            ScaleKind::Major.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ]
        );

        // Natural minor: W-H-W-W-H-W-W
        assert_eq!(ScaleKind::NaturalMinor.intervals().len(), 7);
        assert_eq!(
            ScaleKind::NaturalMinor.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ]
        );

        // Harmonic minor: W-H-W-W-H-W+H-H
        assert_eq!(ScaleKind::HarmonicMinor.intervals().len(), 7);

        // Whole tone: W-W-W-W-W-W
        assert_eq!(ScaleKind::WholeTone.intervals().len(), 6);

        // Chromatic: all 12 semitones
        assert_eq!(ScaleKind::Chromatic.intervals().len(), 12);

        // Diminished scales: 8 notes
        assert_eq!(ScaleKind::DiminishedWholeHalf.intervals().len(), 8);
        assert_eq!(ScaleKind::DiminishedHalfWhole.intervals().len(), 8);

        // Pentatonic scales: 5 notes
        assert_eq!(ScaleKind::MajorPentatonic.intervals().len(), 5);
        assert_eq!(
            ScaleKind::MajorPentatonic.intervals(),
            &[Interval::PerfectUnison, Interval::MajorSecond, Interval::MajorThird, Interval::PerfectFifth, Interval::MajorSixth,]
        );

        assert_eq!(ScaleKind::MinorPentatonic.intervals().len(), 5);
        assert_eq!(
            ScaleKind::MinorPentatonic.intervals(),
            &[Interval::PerfectUnison, Interval::MinorThird, Interval::PerfectFourth, Interval::PerfectFifth, Interval::MinorSeventh,]
        );

        // Blues scale: 6 notes (minor pentatonic + ♯4)
        assert_eq!(ScaleKind::Blues.intervals().len(), 6);
        assert_eq!(
            ScaleKind::Blues.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MinorSeventh,
            ]
        );
    }

    #[test]
    fn test_scale_names() {
        assert_eq!(ScaleKind::Major.static_name(), "major");
        assert_eq!(ScaleKind::NaturalMinor.static_name(), "natural minor");
        assert_eq!(ScaleKind::HarmonicMinor.static_name(), "harmonic minor");
        assert_eq!(ScaleKind::MelodicMinor.static_name(), "melodic minor");
        assert_eq!(ScaleKind::WholeTone.static_name(), "whole tone");
        assert_eq!(ScaleKind::MajorPentatonic.static_name(), "major pentatonic");
        assert_eq!(ScaleKind::MinorPentatonic.static_name(), "minor pentatonic");
        assert_eq!(ScaleKind::Blues.static_name(), "blues");
    }

    #[test]
    fn test_scale_descriptions() {
        assert_eq!(ScaleKind::Major.description(), "major scale, ionian mode parent");
        assert_eq!(ScaleKind::NaturalMinor.description(), "natural minor scale, aeolian mode parent");
        assert_eq!(ScaleKind::MajorPentatonic.description(), "major pentatonic scale, five-note major scale without 4th and 7th");
        assert_eq!(ScaleKind::MinorPentatonic.description(), "minor pentatonic scale, five-note minor scale without 2nd and 6th");
        assert_eq!(ScaleKind::Blues.description(), "blues scale, minor pentatonic with added ♯4 (blue note)");
    }
}
