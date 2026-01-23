//! A module for working with a unified notation type that can represent a chord, scale, or mode.

use std::fmt::{Display, Error, Formatter};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{
    base::{HasDescription, HasName, HasPreciseName, Parsable, Res},
    chord::{Chord, HasChord, HasRoot},
    interval::{HasIntervals, Interval},
    mode::Mode,
    note::Note,
    scale::Scale,
};

// Enum.

/// A unified notation type that can represent a chord, scale, or mode.
///
/// This type attempts to parse input in the following order:
/// 1. Scale (most distinctive keywords like "major pentatonic", "harmonic minor")
/// 2. Mode (keywords like "dorian", "lydian")
/// 3. Chord (fallback, most flexible grammar)
///
/// # Examples
///
/// ```rust
/// use klib::core::base::Parsable;
/// use klib::core::notation::Notation;
///
/// // Parses as a scale
/// let notation = Notation::parse("C major pentatonic").unwrap();
/// assert!(notation.is_scale());
///
/// // Parses as a mode
/// let notation = Notation::parse("D dorian").unwrap();
/// assert!(notation.is_mode());
///
/// // Parses as a chord
/// let notation = Notation::parse("Cmaj7").unwrap();
/// assert!(notation.is_chord());
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Notation {
    /// A chord notation.
    Chord(Chord),
    /// A scale notation.
    Scale(Scale),
    /// A mode notation.
    Mode(Mode),
}

// Impls.

impl Notation {
    /// Returns `true` if this notation is a chord.
    pub fn is_chord(&self) -> bool {
        matches!(self, Notation::Chord(_))
    }

    /// Returns `true` if this notation is a scale.
    pub fn is_scale(&self) -> bool {
        matches!(self, Notation::Scale(_))
    }

    /// Returns `true` if this notation is a mode.
    pub fn is_mode(&self) -> bool {
        matches!(self, Notation::Mode(_))
    }

    /// Returns a reference to the inner chord, if this is a chord.
    pub fn as_chord(&self) -> Option<&Chord> {
        match self {
            Notation::Chord(c) => Some(c),
            _ => None,
        }
    }

    /// Returns a reference to the inner scale, if this is a scale.
    pub fn as_scale(&self) -> Option<&Scale> {
        match self {
            Notation::Scale(s) => Some(s),
            _ => None,
        }
    }

    /// Returns a reference to the inner mode, if this is a mode.
    pub fn as_mode(&self) -> Option<&Mode> {
        match self {
            Notation::Mode(m) => Some(m),
            _ => None,
        }
    }

    /// Consumes the notation and returns the inner chord, if this is a chord.
    pub fn into_chord(self) -> Option<Chord> {
        match self {
            Notation::Chord(c) => Some(c),
            _ => None,
        }
    }

    /// Consumes the notation and returns the inner scale, if this is a scale.
    pub fn into_scale(self) -> Option<Scale> {
        match self {
            Notation::Scale(s) => Some(s),
            _ => None,
        }
    }

    /// Consumes the notation and returns the inner mode, if this is a mode.
    pub fn into_mode(self) -> Option<Mode> {
        match self {
            Notation::Mode(m) => Some(m),
            _ => None,
        }
    }

    /// Returns the notes of this notation.
    ///
    /// For chords, this returns the chord voicing.
    /// For scales and modes, this returns the scale/mode degrees.
    pub fn notes(&self) -> Vec<Note> {
        match self {
            Notation::Chord(c) => c.chord(),
            Notation::Scale(s) => s.notes(),
            Notation::Mode(m) => m.notes(),
        }
    }

    /// Returns a static string describing what kind of notation this is.
    pub fn kind(&self) -> &'static str {
        match self {
            Notation::Chord(_) => "chord",
            Notation::Scale(_) => "scale",
            Notation::Mode(_) => "mode",
        }
    }

    /// Parses a symbol into a Notation, optionally forcing a specific type.
    ///
    /// If `notation_type` is `Some`, it must be one of `"chord"`, `"scale"`, or `"mode"`.
    /// If `None`, auto-detection is used (scale → mode → chord priority).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use klib::core::notation::Notation;
    ///
    /// // Auto-detect
    /// let notation = Notation::parse_with_type("D dorian", None).unwrap();
    /// assert!(notation.is_mode());
    ///
    /// // Force chord interpretation
    /// let notation = Notation::parse_with_type("C", Some("chord")).unwrap();
    /// assert!(notation.is_chord());
    /// ```
    pub fn parse_with_type(symbol: &str, notation_type: Option<&str>) -> Res<Self> {
        match notation_type {
            Some("chord") => Ok(Notation::Chord(Chord::parse(symbol)?)),
            Some("scale") => Ok(Notation::Scale(Scale::parse(symbol)?)),
            Some("mode") => Ok(Notation::Mode(Mode::parse(symbol)?)),
            Some(other) => Err(anyhow::anyhow!("Unknown notation type '{}'. Use 'chord', 'scale', or 'mode'.", other)),
            None => Notation::parse(symbol),
        }
    }

    /// Returns a verbose string representation of this notation.
    ///
    /// For chords, this includes recommended scales/modes.
    /// For scales and modes, this is equivalent to `Display`.
    pub fn format_verbose(&self) -> String {
        match self {
            Notation::Chord(chord) => chord.format_with_scale_candidates(),
            Notation::Scale(scale) => format!("{}", scale),
            Notation::Mode(mode) => format!("{}", mode),
        }
    }
}

impl HasRoot for Notation {
    fn root(&self) -> Note {
        match self {
            Notation::Chord(c) => c.root(),
            Notation::Scale(s) => s.root(),
            Notation::Mode(m) => m.root(),
        }
    }
}

impl HasIntervals for Notation {
    fn intervals(&self) -> &'static [Interval] {
        match self {
            Notation::Chord(c) => c.intervals(),
            Notation::Scale(s) => s.intervals(),
            Notation::Mode(m) => m.intervals(),
        }
    }
}

impl HasName for Notation {
    fn name(&self) -> String {
        match self {
            Notation::Chord(c) => c.name(),
            Notation::Scale(s) => s.name(),
            Notation::Mode(m) => m.name(),
        }
    }
}

impl HasPreciseName for Notation {
    fn precise_name(&self) -> String {
        match self {
            Notation::Chord(c) => c.precise_name(),
            Notation::Scale(s) => s.precise_name(),
            Notation::Mode(m) => m.precise_name(),
        }
    }
}

impl HasDescription for Notation {
    fn description(&self) -> &'static str {
        match self {
            Notation::Chord(c) => c.description(),
            Notation::Scale(s) => s.description(),
            Notation::Mode(m) => m.description(),
        }
    }
}

impl Display for Notation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Notation::Chord(c) => write!(f, "{}", c),
            Notation::Scale(s) => write!(f, "{}", s),
            Notation::Mode(m) => write!(f, "{}", m),
        }
    }
}

impl Parsable for Notation {
    /// Attempts to parse the input as a scale, then a mode, then a chord.
    ///
    /// The order is intentional: scales have the most distinctive keywords,
    /// modes have specific keywords, and chords are the fallback.
    fn parse(input: &str) -> Res<Self>
    where
        Self: Sized,
    {
        // Try scale first (most distinctive keywords).
        if let Ok(scale) = Scale::parse(input) {
            return Ok(Notation::Scale(scale));
        }

        // Try mode second.
        if let Ok(mode) = Mode::parse(input) {
            return Ok(Notation::Mode(mode));
        }

        // Fallback to chord.
        let chord = Chord::parse(input)?;
        Ok(Notation::Chord(chord))
    }
}

impl From<Chord> for Notation {
    fn from(chord: Chord) -> Self {
        Notation::Chord(chord)
    }
}

impl From<Scale> for Notation {
    fn from(scale: Scale) -> Self {
        Notation::Scale(scale)
    }
}

impl From<Mode> for Notation {
    fn from(mode: Mode) -> Self {
        Notation::Mode(mode)
    }
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::mode::HasModeKind;
    use crate::core::mode_kind::ModeKind;
    use crate::core::note::*;
    use crate::core::scale::HasScaleKind;
    use crate::core::scale_kind::ScaleKind;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_as_scale() {
        let notation = Notation::parse("C major pentatonic").unwrap();
        assert!(notation.is_scale());
        assert!(!notation.is_chord());
        assert!(!notation.is_mode());
        assert_eq!(notation.kind(), "scale");

        let scale = notation.as_scale().unwrap();
        assert_eq!(scale.root(), C);
        assert_eq!(scale.kind(), ScaleKind::MajorPentatonic);
    }

    #[test]
    fn test_parse_as_mode() {
        let notation = Notation::parse("D dorian").unwrap();
        assert!(notation.is_mode());
        assert!(!notation.is_chord());
        assert!(!notation.is_scale());
        assert_eq!(notation.kind(), "mode");

        let mode = notation.as_mode().unwrap();
        assert_eq!(mode.root(), D);
        assert_eq!(mode.kind(), ModeKind::Dorian);
    }

    #[test]
    fn test_parse_as_chord() {
        let notation = Notation::parse("Cmaj7").unwrap();
        assert!(notation.is_chord());
        assert!(!notation.is_scale());
        assert!(!notation.is_mode());
        assert_eq!(notation.kind(), "chord");

        let chord = notation.as_chord().unwrap();
        assert_eq!(chord.root(), C);
    }

    #[test]
    fn test_parse_complex_chord() {
        let notation = Notation::parse("Em7b9b13").unwrap();
        assert!(notation.is_chord());
        assert_eq!(notation.root(), E);
    }

    #[test]
    fn test_parse_harmonic_minor() {
        let notation = Notation::parse("A harmonic minor").unwrap();
        assert!(notation.is_scale());

        let scale = notation.as_scale().unwrap();
        assert_eq!(scale.kind(), ScaleKind::HarmonicMinor);
    }

    #[test]
    fn test_parse_lydian_mode() {
        let notation = Notation::parse("F lydian").unwrap();
        assert!(notation.is_mode());

        let mode = notation.as_mode().unwrap();
        assert_eq!(mode.kind(), ModeKind::Lydian);
    }

    #[test]
    fn test_into_inner() {
        let notation = Notation::parse("Cm7").unwrap();
        let chord = notation.into_chord().unwrap();
        assert_eq!(chord.root(), C);

        let notation = Notation::parse("G mixolydian").unwrap();
        let mode = notation.into_mode().unwrap();
        assert_eq!(mode.root(), G);

        let notation = Notation::parse("E blues").unwrap();
        let scale = notation.into_scale().unwrap();
        assert_eq!(scale.root(), E);
    }

    #[test]
    fn test_notes() {
        let notation = Notation::parse("C major").unwrap();
        assert!(notation.is_scale());
        let notes = notation.notes();
        assert_eq!(notes.len(), 7);
        assert_eq!(notes[0], C);

        let notation = Notation::parse("A aeolian").unwrap();
        assert!(notation.is_mode());
        let notes = notation.notes();
        assert_eq!(notes.len(), 7);
        assert_eq!(notes[0], A);
    }

    #[test]
    fn test_intervals() {
        // Scale intervals
        let notation = Notation::parse("C major").unwrap();
        assert!(!notation.intervals().is_empty());

        // Mode intervals
        let notation = Notation::parse("D dorian").unwrap();
        assert!(!notation.intervals().is_empty());

        // Chord intervals (chord tones from KnownChord)
        let notation = Notation::parse("Cmaj7").unwrap();
        assert!(!notation.intervals().is_empty());
        assert_eq!(notation.intervals().len(), 4); // Root, 3rd, 5th, 7th
    }

    #[test]
    fn test_has_name() {
        let notation = Notation::parse("C major pentatonic").unwrap();
        assert_eq!(notation.name(), "C major pentatonic");

        let notation = Notation::parse("D dorian").unwrap();
        assert_eq!(notation.name(), "D dorian");

        let notation = Notation::parse("Cmaj7").unwrap();
        assert!(notation.name().contains("C"));
    }

    #[test]
    fn test_from_impls() {
        let chord = Chord::parse("Cm7").unwrap();
        let notation: Notation = chord.into();
        assert!(notation.is_chord());

        let scale = Scale::parse("C major").unwrap();
        let notation: Notation = scale.into();
        assert!(notation.is_scale());

        let mode = Mode::parse("D dorian").unwrap();
        let notation: Notation = mode.into();
        assert!(notation.is_mode());
    }

    #[test]
    fn test_display() {
        let notation = Notation::parse("C major").unwrap();
        let display = format!("{}", notation);
        assert!(display.contains("C major"));

        let notation = Notation::parse("D dorian").unwrap();
        let display = format!("{}", notation);
        assert!(display.contains("D dorian"));
    }

    #[test]
    fn test_priority_scale_over_chord() {
        // "C major" should parse as scale, not chord
        let notation = Notation::parse("C major").unwrap();
        assert!(notation.is_scale(), "Expected 'C major' to parse as scale, got {:?}", notation.kind());
    }

    #[test]
    fn test_priority_mode_over_chord() {
        // "C ionian" should parse as mode, not chord
        let notation = Notation::parse("C ionian").unwrap();
        assert!(notation.is_mode(), "Expected 'C ionian' to parse as mode, got {:?}", notation.kind());
    }
}
