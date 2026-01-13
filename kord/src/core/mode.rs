//! A module for working with musical modes (Greek modes).

use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{
    base::{HasDescription, HasName, HasStaticName, Parsable, Res},
    interval::Interval,
    known_chord::HasRelativeScale,
    note::Note,
};

// Enum.

/// An enum representing the seven Greek modes.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Mode {
    /// Ionian mode (major scale).
    Ionian,
    /// Dorian mode.
    Dorian,
    /// Phrygian mode.
    Phrygian,
    /// Lydian mode.
    Lydian,
    /// Mixolydian mode.
    Mixolydian,
    /// Aeolian mode (natural minor scale).
    Aeolian,
    /// Locrian mode.
    Locrian,
}

// Struct.

/// A struct representing a mode with a root note.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ModeWithRoot {
    /// The root note of the mode.
    root: Note,
    /// The mode.
    mode: Mode,
}

// Impls.

impl HasStaticName for Mode {
    fn static_name(&self) -> &'static str {
        match self {
            Mode::Ionian => "Ionian",
            Mode::Dorian => "Dorian",
            Mode::Phrygian => "Phrygian",
            Mode::Lydian => "Lydian",
            Mode::Mixolydian => "Mixolydian",
            Mode::Aeolian => "Aeolian",
            Mode::Locrian => "Locrian",
        }
    }
}

impl HasDescription for Mode {
    fn description(&self) -> &'static str {
        match self {
            Mode::Ionian => "major scale, first mode of major scale",
            Mode::Dorian => "second mode of major scale, minor with raised 6th",
            Mode::Phrygian => "third mode of major scale, minor with lowered 2nd",
            Mode::Lydian => "fourth mode of major scale, major with raised 4th",
            Mode::Mixolydian => "fifth mode of major scale, major with lowered 7th",
            Mode::Aeolian => "natural minor scale, sixth mode of major scale",
            Mode::Locrian => "seventh mode of major scale, diminished scale with lowered 2nd and 5th",
        }
    }
}

impl HasRelativeScale for Mode {
    fn relative_scale(&self) -> Vec<Interval> {
        match self {
            Mode::Ionian => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            Mode::Dorian => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            Mode::Phrygian => vec![
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            Mode::Lydian => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            Mode::Mixolydian => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            Mode::Aeolian => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            Mode::Locrian => vec![
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.static_name())
    }
}

impl Parsable for Mode {
    fn parse(input: &str) -> Res<Self>
    where
        Self: Sized,
    {
        match input {
            "Ionian" | "ionian" => Ok(Mode::Ionian),
            "Dorian" | "dorian" => Ok(Mode::Dorian),
            "Phrygian" | "phrygian" => Ok(Mode::Phrygian),
            "Lydian" | "lydian" => Ok(Mode::Lydian),
            "Mixolydian" | "mixolydian" => Ok(Mode::Mixolydian),
            "Aeolian" | "aeolian" => Ok(Mode::Aeolian),
            "Locrian" | "locrian" => Ok(Mode::Locrian),
            _ => Err(anyhow::Error::msg(format!("Unknown mode: {}", input))),
        }
    }
}

impl ModeWithRoot {
    /// Creates a new mode with a root note.
    pub fn new(root: Note, mode: Mode) -> Self {
        Self { root, mode }
    }

    /// Returns the root note of the mode.
    pub fn root(&self) -> Note {
        self.root
    }

    /// Returns the mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Returns the scale for this mode with the root note.
    pub fn scale(&self) -> Vec<Note> {
        self.mode.relative_scale().into_iter().map(|i| self.root + i).collect()
    }
}

impl HasName for ModeWithRoot {
    fn name(&self) -> String {
        format!("{} {}", self.root.static_name(), self.mode.static_name())
    }
}

impl Display for ModeWithRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scale = self.scale().iter().map(HasStaticName::static_name).collect::<Vec<_>>().join(", ");
        write!(f, "{}\n   {}\n   {}", self.name(), self.mode.description(), scale)
    }
}

impl Parsable for ModeWithRoot {
    fn parse(input: &str) -> Res<Self>
    where
        Self: Sized,
    {
        use crate::core::parser::{note_str_to_note, ChordParser, Rule};
        use pest::Parser;

        let result = ChordParser::parse(Rule::mode_with_root, input)?.next().unwrap();

        assert_eq!(Rule::mode_with_root, result.as_rule());

        let mut components = result.into_inner();

        let note = components.next().unwrap();
        assert_eq!(Rule::note, note.as_rule());
        let root = note_str_to_note(note.into_inner().as_str())?;

        let mode_str = components.next().unwrap();
        assert_eq!(Rule::mode_name, mode_str.as_rule());
        let mode = Mode::parse(mode_str.as_str())?;

        Ok(ModeWithRoot::new(root, mode))
    }
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::note::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_mode_names() {
        assert_eq!(Mode::Ionian.static_name(), "Ionian");
        assert_eq!(Mode::Dorian.static_name(), "Dorian");
        assert_eq!(Mode::Phrygian.static_name(), "Phrygian");
        assert_eq!(Mode::Lydian.static_name(), "Lydian");
        assert_eq!(Mode::Mixolydian.static_name(), "Mixolydian");
        assert_eq!(Mode::Aeolian.static_name(), "Aeolian");
        assert_eq!(Mode::Locrian.static_name(), "Locrian");
    }

    #[test]
    fn test_mode_scales() {
        // Ionian (C major)
        assert_eq!(ModeWithRoot::new(C, Mode::Ionian).scale(), vec![C, D, E, F, G, A, B]);
        
        // Dorian (C Dorian)
        assert_eq!(ModeWithRoot::new(C, Mode::Dorian).scale(), vec![C, D, EFlat, F, G, A, BFlat]);
        
        // Phrygian (E Phrygian = notes of C major starting on E)
        assert_eq!(ModeWithRoot::new(E, Mode::Phrygian).scale(), vec![E, F, G, A, B, CFive, DFive]);
        
        // Lydian (F Lydian = notes of C major starting on F)
        assert_eq!(ModeWithRoot::new(F, Mode::Lydian).scale(), vec![F, G, A, B, CFive, DFive, EFive]);
        
        // Mixolydian (G Mixolydian = notes of C major starting on G)
        assert_eq!(ModeWithRoot::new(G, Mode::Mixolydian).scale(), vec![G, A, B, CFive, DFive, EFive, FFive]);
        
        // Aeolian (A Aeolian = A natural minor)
        assert_eq!(ModeWithRoot::new(A, Mode::Aeolian).scale(), vec![A, B, CFive, DFive, EFive, FFive, GFive]);
        
        // Locrian (B Locrian = notes of C major starting on B)
        assert_eq!(ModeWithRoot::new(B, Mode::Locrian).scale(), vec![B, CFive, DFive, EFive, FFive, GFive, AFive]);
    }

    #[test]
    fn test_mode_parse() {
        assert_eq!(Mode::parse("Ionian").unwrap(), Mode::Ionian);
        assert_eq!(Mode::parse("ionian").unwrap(), Mode::Ionian);
        assert_eq!(Mode::parse("Dorian").unwrap(), Mode::Dorian);
        assert_eq!(Mode::parse("dorian").unwrap(), Mode::Dorian);
        assert_eq!(Mode::parse("Phrygian").unwrap(), Mode::Phrygian);
        assert_eq!(Mode::parse("Lydian").unwrap(), Mode::Lydian);
        assert_eq!(Mode::parse("Mixolydian").unwrap(), Mode::Mixolydian);
        assert_eq!(Mode::parse("Aeolian").unwrap(), Mode::Aeolian);
        assert_eq!(Mode::parse("Locrian").unwrap(), Mode::Locrian);
    }

    #[test]
    fn test_mode_with_root_parse() {
        let mode = ModeWithRoot::parse("C Dorian").unwrap();
        assert_eq!(mode.root(), C);
        assert_eq!(mode.mode(), Mode::Dorian);
        assert_eq!(mode.scale(), vec![C, D, EFlat, F, G, A, BFlat]);

        let mode = ModeWithRoot::parse("D Phrygian").unwrap();
        assert_eq!(mode.root(), D);
        assert_eq!(mode.mode(), Mode::Phrygian);
    }

    #[test]
    fn test_mode_display() {
        let mode = ModeWithRoot::new(C, Mode::Dorian);
        let display = format!("{}", mode);
        assert!(display.contains("C Dorian"));
        assert!(display.contains("C, D, E♭, F, G, A, B♭"));
    }
}
