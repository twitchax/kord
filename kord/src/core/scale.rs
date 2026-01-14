//! A module for working with musical scales.

use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{
    base::{HasDescription, HasName, HasStaticName, Parsable, Res},
    interval::Interval,
    note::Note,
};

// Traits.

/// A trait for types that have a static list of intervals.
pub trait HasIntervals {
    /// Returns the intervals of the type.
    fn intervals(&self) -> &'static [Interval];
}

// Enum.

/// An enum representing different kinds of musical scales.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ScaleKind {
    /// Major scale (Ionian mode).
    Major,
    /// Natural minor scale (Aeolian mode).
    NaturalMinor,
    /// Harmonic minor scale.
    HarmonicMinor,
    /// Melodic minor scale (jazz/ascending).
    MelodicMinor,
    /// Whole tone scale.
    WholeTone,
    /// Diminished scale (whole-half).
    DiminishedWholeHalf,
    /// Diminished scale (half-whole).
    DiminishedHalfWhole,
}

// Struct.

/// A struct representing a scale with a root note.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Scale {
    /// The root note of the scale.
    root: Note,
    /// The kind of scale.
    kind: ScaleKind,
}

// Impls.

impl HasStaticName for ScaleKind {
    fn static_name(&self) -> &'static str {
        match self {
            ScaleKind::Major => "Major",
            ScaleKind::NaturalMinor => "Natural Minor",
            ScaleKind::HarmonicMinor => "Harmonic Minor",
            ScaleKind::MelodicMinor => "Melodic Minor",
            ScaleKind::WholeTone => "Whole Tone",
            ScaleKind::DiminishedWholeHalf => "Diminished (W-H)",
            ScaleKind::DiminishedHalfWhole => "Diminished (H-W)",
        }
    }
}

impl HasDescription for ScaleKind {
    fn description(&self) -> &'static str {
        match self {
            ScaleKind::Major => "major scale, also known as the Ionian mode",
            ScaleKind::NaturalMinor => "natural minor scale, also known as the Aeolian mode",
            ScaleKind::HarmonicMinor => "harmonic minor scale, with raised 7th degree",
            ScaleKind::MelodicMinor => "melodic minor scale (jazz/ascending), with raised 6th and 7th degrees",
            ScaleKind::WholeTone => "whole tone scale, made entirely of whole steps",
            ScaleKind::DiminishedWholeHalf => "diminished scale with whole-half pattern",
            ScaleKind::DiminishedHalfWhole => "diminished scale with half-whole pattern",
        }
    }
}

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
        }
    }
}

impl Display for ScaleKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.static_name())
    }
}

impl Scale {
    /// Creates a new scale with a root note and kind.
    pub fn new(root: Note, kind: ScaleKind) -> Self {
        Self { root, kind }
    }

    /// Returns the root note of the scale.
    pub fn root(&self) -> Note {
        self.root
    }

    /// Returns the kind of the scale.
    pub fn kind(&self) -> ScaleKind {
        self.kind
    }

    /// Returns the intervals for this scale.
    pub fn intervals(&self) -> &'static [Interval] {
        self.kind.intervals()
    }

    /// Returns the notes for this scale with the root note.
    pub fn notes(&self) -> Vec<Note> {
        self.intervals().iter().map(|&i| self.root + i).collect()
    }
}

impl HasName for Scale {
    fn name(&self) -> String {
        format!("{} {}", self.root.static_name(), self.kind.static_name())
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let notes = self.notes().iter().map(HasStaticName::static_name).collect::<Vec<_>>().join(", ");
        write!(f, "{}\n   {}\n   {}", self.name(), self.kind.description(), notes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::note::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_scale_kind_names() {
        assert_eq!(ScaleKind::Major.static_name(), "Major");
        assert_eq!(ScaleKind::NaturalMinor.static_name(), "Natural Minor");
        assert_eq!(ScaleKind::HarmonicMinor.static_name(), "Harmonic Minor");
    }

    #[test]
    fn test_scale_intervals() {
        let major_intervals = ScaleKind::Major.intervals();
        assert_eq!(major_intervals.len(), 7);
        assert_eq!(major_intervals[0], Interval::PerfectUnison);
        assert_eq!(major_intervals[6], Interval::MajorSeventh);
    }

    #[test]
    fn test_scale_notes() {
        // C Major
        let c_major = Scale::new(C, ScaleKind::Major);
        assert_eq!(c_major.notes(), vec![C, D, E, F, G, A, B]);
        
        // A Natural Minor
        let a_minor = Scale::new(A, ScaleKind::NaturalMinor);
        assert_eq!(a_minor.notes(), vec![A, B, CFive, DFive, EFive, FFive, GFive]);
    }

    #[test]
    fn test_scale_display() {
        let scale = Scale::new(C, ScaleKind::Major);
        let display = format!("{}", scale);
        assert!(display.contains("C Major"));
        assert!(display.contains("C, D, E, F, G, A, B"));
    }
}
