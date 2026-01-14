//! A module for working with scales.

use std::fmt::{Display, Error, Formatter};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{
    base::{HasDescription, HasName, HasPreciseName, HasStaticName},
    chord::HasRoot,
    interval::{HasIntervals, Interval},
    note::Note,
    scale_kind::ScaleKind,
};

// Traits.

/// A trait that represents a type that has a scale kind.
pub trait HasScaleKind {
    /// Returns the scale kind of the implementor (most likely a [`Scale`]).
    fn kind(&self) -> ScaleKind;
}

// Struct.

/// A scale with a root note.
///
/// This combines a root note with a scale kind to produce an actual scale
/// with specific notes.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Scale {
    /// The root note of the scale.
    root: Note,
    /// The kind of scale.
    kind: ScaleKind,
}

// Impls.

impl Scale {
    /// Creates a new scale with the given root note and scale kind.
    pub fn new(root: Note, kind: ScaleKind) -> Self {
        Self { root, kind }
    }

    /// Returns the intervals of this scale (delegates to the scale kind).
    pub fn intervals(&self) -> &'static [Interval] {
        self.kind.intervals()
    }

    /// Returns the notes of this scale (root + each interval).
    pub fn notes(&self) -> Vec<Note> {
        self.intervals().iter().map(|&interval| self.root + interval).collect()
    }
}

impl HasRoot for Scale {
    fn root(&self) -> Note {
        self.root
    }
}

impl HasScaleKind for Scale {
    fn kind(&self) -> ScaleKind {
        self.kind
    }
}

impl HasIntervals for Scale {
    fn intervals(&self) -> &'static [Interval] {
        self.kind.intervals()
    }
}

impl HasStaticName for Scale {
    fn static_name(&self) -> &'static str {
        self.kind.static_name()
    }
}

impl HasName for Scale {
    fn name(&self) -> String {
        format!("{} {}", self.root.static_name(), self.kind.static_name())
    }
}

impl HasPreciseName for Scale {
    fn precise_name(&self) -> String {
        format!("{} {}", self.root.name(), self.kind.static_name())
    }
}

impl HasDescription for Scale {
    fn description(&self) -> &'static str {
        self.kind.description()
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let notes = self.notes().iter().map(|n| n.static_name()).collect::<Vec<_>>().join(", ");
        write!(f, "{}\n   {}\n   {}", self.name(), self.description(), notes)
    }
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::note::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_scale_creation() {
        let scale = Scale::new(C, ScaleKind::Major);
        assert_eq!(scale.root(), C);
        assert_eq!(scale.kind(), ScaleKind::Major);
    }

    #[test]
    fn test_scale_intervals() {
        let scale = Scale::new(C, ScaleKind::Major);
        assert_eq!(scale.intervals().len(), 7);
        assert_eq!(
            scale.intervals(),
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
    }

    #[test]
    fn test_scale_notes() {
        // C major scale
        let scale = Scale::new(C, ScaleKind::Major);
        assert_eq!(scale.notes(), vec![C, D, E, F, G, A, B]);

        // D major scale
        let scale = Scale::new(D, ScaleKind::Major);
        assert_eq!(scale.notes(), vec![D, E, FSharp, G, A, B, CSharpFive]);

        // A natural minor scale
        let scale = Scale::new(A, ScaleKind::NaturalMinor);
        assert_eq!(scale.notes(), vec![A, B, CFive, DFive, EFive, FFive, GFive]);

        // A harmonic minor scale
        let scale = Scale::new(A, ScaleKind::HarmonicMinor);
        assert_eq!(scale.notes(), vec![A, B, CFive, DFive, EFive, FFive, GSharpFive]);

        // A melodic minor scale
        let scale = Scale::new(A, ScaleKind::MelodicMinor);
        assert_eq!(scale.notes(), vec![A, B, CFive, DFive, EFive, FSharpFive, GSharpFive]);

        // C whole tone scale
        let scale = Scale::new(C, ScaleKind::WholeTone);
        assert_eq!(scale.notes(), vec![C, D, E, FSharp, GSharp, ASharp]);

        // C chromatic scale
        let scale = Scale::new(C, ScaleKind::Chromatic);
        assert_eq!(scale.notes().len(), 12);
    }

    #[test]
    fn test_scale_names() {
        let scale = Scale::new(C, ScaleKind::Major);
        assert_eq!(scale.name(), "C major");
        assert_eq!(scale.static_name(), "major");

        let scale = Scale::new(DFlat, ScaleKind::HarmonicMinor);
        assert_eq!(scale.name(), "D♭ harmonic minor");

        let scale = Scale::new(FSharp, ScaleKind::WholeTone);
        assert_eq!(scale.name(), "F♯ whole tone");
    }

    #[test]
    fn test_scale_display() {
        let scale = Scale::new(C, ScaleKind::Major);
        let display = format!("{}", scale);
        assert!(display.contains("C major"));
        assert!(display.contains("C, D, E, F, G, A, B"));
    }

    #[test]
    fn test_different_roots() {
        // G major has one sharp (F#)
        let scale = Scale::new(G, ScaleKind::Major);
        assert_eq!(scale.notes(), vec![G, A, B, CFive, DFive, EFive, FSharpFive]);

        // F major has one flat (Bb)
        let scale = Scale::new(F, ScaleKind::Major);
        assert_eq!(scale.notes(), vec![F, G, A, BFlat, CFive, DFive, EFive]);

        // E natural minor
        let scale = Scale::new(E, ScaleKind::NaturalMinor);
        assert_eq!(scale.notes(), vec![E, FSharp, G, A, B, CFive, DFive]);
    }
}
