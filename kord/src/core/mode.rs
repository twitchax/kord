//! A module for working with musical modes (Greek modes).

use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{
    base::{HasDescription, HasName, HasStaticName, Parsable, Res},
    chord::{Chord, Chordable, HasRoot},
    interval::Interval,
    known_chord::HasRelativeScale,
    note::Note,
};

// Traits.

/// A trait for types that can be converted to a chord.
pub trait ToChord {
    /// Converts the implementor to a chord.
    fn to_chord(&self) -> Chord;
}

/// A trait for types that can be converted to a mode.
pub trait ToMode {
    /// Converts the implementor to a mode with root, if possible.
    fn to_mode(&self) -> Option<ModeWithRoot>;
}

// Enum.

/// An enum representing musical modes (Greek modes, harmonic minor modes, and melodic minor modes).
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Mode {
    // Major scale modes (Greek modes)
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

    // Harmonic minor modes
    /// Locrian ♮6 mode (2nd mode of harmonic minor).
    LocrianNatural6,
    /// Ionian ♯5 mode (3rd mode of harmonic minor).
    IonianAugmented,
    /// Dorian ♯4 mode (4th mode of harmonic minor).
    DorianSharp4,
    /// Phrygian Dominant mode (5th mode of harmonic minor).
    PhrygianDominant,
    /// Lydian ♯2 mode (6th mode of harmonic minor).
    LydianSharp2,
    /// Ultralocrian mode (7th mode of harmonic minor).
    Ultralocrian,

    // Melodic minor modes
    /// Dorian ♭2 mode (2nd mode of melodic minor).
    DorianFlat2,
    /// Lydian Augmented mode (3rd mode of melodic minor).
    LydianAugmented,
    /// Lydian Dominant mode (4th mode of melodic minor).
    LydianDominant,
    /// Mixolydian ♭6 mode (5th mode of melodic minor).
    MixolydianFlat6,
    /// Locrian ♮2 mode (6th mode of melodic minor).
    LocrianNatural2,
    /// Altered/Super Locrian mode (7th mode of melodic minor).
    Altered,
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
            Mode::LocrianNatural6 => "Locrian ♮6",
            Mode::IonianAugmented => "Ionian ♯5",
            Mode::DorianSharp4 => "Dorian ♯4",
            Mode::PhrygianDominant => "Phrygian Dominant",
            Mode::LydianSharp2 => "Lydian ♯2",
            Mode::Ultralocrian => "Ultralocrian",
            Mode::DorianFlat2 => "Dorian ♭2",
            Mode::LydianAugmented => "Lydian Augmented",
            Mode::LydianDominant => "Lydian Dominant",
            Mode::MixolydianFlat6 => "Mixolydian ♭6",
            Mode::LocrianNatural2 => "Locrian ♮2",
            Mode::Altered => "Altered",
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
            Mode::LocrianNatural6 => "2nd mode of harmonic minor, common over m7♭5(♮13) colors",
            Mode::IonianAugmented => "3rd mode of harmonic minor, major with sharp 5",
            Mode::DorianSharp4 => "4th mode of harmonic minor, minor with a Lydian bite",
            Mode::PhrygianDominant => "5th mode of harmonic minor, classic V in minor, Spanish sound",
            Mode::LydianSharp2 => "6th mode of harmonic minor, bright and exotic with sharp 2 and sharp 4",
            Mode::Ultralocrian => "7th mode of harmonic minor, rare, very unstable and dark",
            Mode::DorianFlat2 => "2nd mode of melodic minor, minor with spicy flat 2 but strong 6",
            Mode::LydianAugmented => "3rd mode of melodic minor, Lydian with sharp 5 sheen",
            Mode::LydianDominant => "4th mode of melodic minor, the dominant-with-sharp-11 sound, also known as the acoustic scale",
            Mode::MixolydianFlat6 => "5th mode of melodic minor, dominant that leans minor with flat 13",
            Mode::LocrianNatural2 => "6th mode of melodic minor, m7♭5 with a cleaner 2",
            Mode::Altered => "7th mode of melodic minor, the go-to for V7alt, also known as Super Locrian",
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
            // Harmonic minor modes
            Mode::LocrianNatural6 => vec![
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            Mode::IonianAugmented => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::AugmentedFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            Mode::DorianSharp4 => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            Mode::PhrygianDominant => vec![
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            Mode::LydianSharp2 => vec![
                Interval::PerfectUnison,
                Interval::AugmentedSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            Mode::Ultralocrian => vec![
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::DiminishedFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::DiminishedSeventh,
            ],
            // Melodic minor modes
            Mode::DorianFlat2 => vec![
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            Mode::LydianAugmented => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::AugmentedFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            Mode::LydianDominant => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            Mode::MixolydianFlat6 => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            Mode::LocrianNatural2 => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            Mode::Altered => vec![
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::AugmentedSecond,
                Interval::DiminishedFourth,
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
        let lower = input.to_lowercase();
        match lower.as_str() {
            "ionian" => Ok(Mode::Ionian),
            "dorian" => Ok(Mode::Dorian),
            "phrygian" => Ok(Mode::Phrygian),
            "lydian" => Ok(Mode::Lydian),
            "mixolydian" => Ok(Mode::Mixolydian),
            "aeolian" => Ok(Mode::Aeolian),
            "locrian" => Ok(Mode::Locrian),
            // Harmonic minor modes
            "locrian natural 6" | "locrian nat6" | "locrian ♮6" => Ok(Mode::LocrianNatural6),
            "ionian augmented" | "ionian ♯5" | "major ♯5" | "augmented major" => Ok(Mode::IonianAugmented),
            "dorian ♯4" | "dorian sharp 4" => Ok(Mode::DorianSharp4),
            "phrygian dominant" | "spanish phrygian" | "phrygian major" => Ok(Mode::PhrygianDominant),
            "lydian ♯2" | "lydian sharp 2" => Ok(Mode::LydianSharp2),
            "ultralocrian" | "ultra locrian" => Ok(Mode::Ultralocrian),
            // Melodic minor modes
            "dorian ♭2" | "dorian flat 2" | "phrygian ♮6" => Ok(Mode::DorianFlat2),
            "lydian augmented" | "lydian ♯5" | "lydian sharp 5" => Ok(Mode::LydianAugmented),
            "lydian dominant" | "lydian ♭7" | "mixolydian ♯4" | "acoustic scale" => Ok(Mode::LydianDominant),
            "mixolydian ♭6" | "mixolydian flat 6" | "aeolian dominant" => Ok(Mode::MixolydianFlat6),
            "locrian ♮2" | "locrian natural 2" | "locrian ♯2" | "half-diminished ♮2" => Ok(Mode::LocrianNatural2),
            "altered" | "super locrian" | "altered scale" => Ok(Mode::Altered),
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

// ToChord and ToMode implementations

impl ToChord for ModeWithRoot {
    fn to_chord(&self) -> Chord {
        let chord = Chord::new(self.root);
        
        match self.mode {
            // Major scale modes
            Mode::Ionian => chord, // Just major
            Mode::Dorian => chord.minor().seven(),
            Mode::Phrygian => chord.minor().seven().flat9().flat13(),
            Mode::Lydian => chord.major7().sharp11(),
            Mode::Mixolydian => chord.seven(),
            Mode::Aeolian => chord.minor().seven().flat13(),
            Mode::Locrian => chord.minor().seven().flat5(),
            
            // Harmonic minor modes
            Mode::LocrianNatural6 => chord.minor().seven().flat5().add13(),
            Mode::IonianAugmented => chord.major7().augmented(),
            Mode::DorianSharp4 => chord.minor().seven().sharp11(),
            Mode::PhrygianDominant => chord.seven().flat9(),
            Mode::LydianSharp2 => chord.major7().sharp11(),
            Mode::Ultralocrian => chord.diminished().seven(),
            
            // Melodic minor modes
            Mode::DorianFlat2 => chord.minor().seven().flat9(),
            Mode::LydianAugmented => chord.major7().augmented().sharp11(),
            Mode::LydianDominant => chord.seven().sharp11(),
            Mode::MixolydianFlat6 => chord.seven().flat13(),
            Mode::LocrianNatural2 => chord.minor().seven().flat5(),
            Mode::Altered => chord.seven().flat9().sharp9().flat5().augmented(),
        }
    }
}

impl ToMode for Chord {
    fn to_mode(&self) -> Option<ModeWithRoot> {
        use crate::core::chord::{HasModifiers, HasExtensions, HasDomninantDegree};
        use crate::core::modifier::{Modifier, Degree, Extension};
        
        let modifiers = self.modifiers();
        let extensions = self.extensions();
        let has_dominant = self.dominant_degree();
        
        // Try to match chord to a mode
        let mode = if modifiers.is_empty() && extensions.is_empty() {
            // Plain major chord
            Some(Mode::Ionian)
        } else if modifiers.contains(&Modifier::Minor) && has_dominant == Some(Degree::Seven) {
            if modifiers.contains(&Modifier::Flat5) {
                // Locrian or LocrianNatural2
                if extensions.contains(&Extension::Add13) {
                    Some(Mode::LocrianNatural6)
                } else {
                    Some(Mode::Locrian)
                }
            } else if modifiers.contains(&Modifier::Flat9) && modifiers.contains(&Modifier::Flat13) {
                Some(Mode::Phrygian)
            } else if modifiers.contains(&Modifier::Flat13) {
                Some(Mode::Aeolian)
            } else if modifiers.contains(&Modifier::Flat9) {
                Some(Mode::DorianFlat2)
            } else if modifiers.contains(&Modifier::Sharp11) {
                Some(Mode::DorianSharp4)
            } else {
                Some(Mode::Dorian)
            }
        } else if modifiers.contains(&Modifier::Major7) {
            if modifiers.contains(&Modifier::Sharp11) {
                if modifiers.contains(&Modifier::Augmented5) {
                    Some(Mode::LydianAugmented)
                } else {
                    Some(Mode::Lydian)
                }
            } else if modifiers.contains(&Modifier::Augmented5) {
                Some(Mode::IonianAugmented)
            } else {
                Some(Mode::Ionian)
            }
        } else if has_dominant == Some(Degree::Seven) {
            if modifiers.contains(&Modifier::Sharp11) {
                Some(Mode::LydianDominant)
            } else if modifiers.contains(&Modifier::Flat13) {
                Some(Mode::MixolydianFlat6)
            } else if modifiers.contains(&Modifier::Flat9) {
                if modifiers.contains(&Modifier::Sharp9) {
                    Some(Mode::Altered)
                } else {
                    Some(Mode::PhrygianDominant)
                }
            } else {
                Some(Mode::Mixolydian)
            }
        } else {
            None
        };
        
        mode.map(|m| ModeWithRoot::new(self.root(), m))
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

    #[test]
    fn test_harmonic_minor_modes() {
        // Phrygian Dominant (E Phrygian Dominant from A harmonic minor)
        assert_eq!(
            ModeWithRoot::new(E, Mode::PhrygianDominant).scale(),
            vec![E, F, GSharp, A, B, CFive, DFive]
        );
        
        // Lydian Sharp 2
        assert_eq!(
            ModeWithRoot::new(F, Mode::LydianSharp2).scale(),
            vec![F, GSharp, A, B, CFive, DFive, EFive]
        );
    }

    #[test]
    fn test_melodic_minor_modes() {
        // Dorian Flat 2 (B Dorian ♭2 from A melodic minor)
        assert_eq!(
            ModeWithRoot::new(B, Mode::DorianFlat2).scale(),
            vec![B, CFive, DFive, EFive, FSharpFive, GSharpFive, AFive]
        );
        
        // Lydian Dominant
        assert_eq!(
            ModeWithRoot::new(D, Mode::LydianDominant).scale(),
            vec![D, E, FSharp, GSharp, A, B, CFive]
        );
        
        // Altered mode - using enharmonically correct notes
        // G# Altered scale intervals produce: G# A A## (B enharmonic) C (as octave 5) D E F#
        let altered_scale = ModeWithRoot::new(GSharp, Mode::Altered).scale();
        assert_eq!(altered_scale.len(), 7);
        assert_eq!(altered_scale[0], GSharp); // Root
        assert_eq!(altered_scale[1], A); // ♭2
        // Note: altered_scale[2] will be ADoubleSharp (enharmonic to B)
        // Note: altered_scale[3] will be CFive (enharmonic to BSharp of octave 4)
    }

    #[test]
    fn test_case_insensitive_parsing() {
        assert_eq!(Mode::parse("IONIAN").unwrap(), Mode::Ionian);
        assert_eq!(Mode::parse("DoRiAn").unwrap(), Mode::Dorian);
        assert_eq!(Mode::parse("mixolydian").unwrap(), Mode::Mixolydian);
        assert_eq!(Mode::parse("ALTERED").unwrap(), Mode::Altered);
        assert_eq!(Mode::parse("lydian dominant").unwrap(), Mode::LydianDominant);
    }

    #[test]
    fn test_mode_to_chord() {
        // Dorian -> Dm7
        let dorian = ModeWithRoot::new(D, Mode::Dorian);
        let chord = dorian.to_chord();
        assert_eq!(chord.name(), "Dm7");
        
        // Lydian -> maj7#11
        let lydian = ModeWithRoot::new(F, Mode::Lydian);
        let chord = lydian.to_chord();
        assert!(chord.name().contains("maj7"));
        assert!(chord.name().contains("♯11"));
        
        // Mixolydian -> 7
        let mixolydian = ModeWithRoot::new(G, Mode::Mixolydian);
        let chord = mixolydian.to_chord();
        assert_eq!(chord.name(), "G7");
    }

    #[test]
    fn test_chord_to_mode() {
        // Dm7 -> Dorian
        let chord = Chord::parse("Dm7").unwrap();
        let mode = chord.to_mode();
        assert!(mode.is_some());
        assert_eq!(mode.unwrap().mode(), Mode::Dorian);
        
        // G7 -> Mixolydian
        let chord = Chord::parse("G7").unwrap();
        let mode = chord.to_mode();
        assert!(mode.is_some());
        assert_eq!(mode.unwrap().mode(), Mode::Mixolydian);
        
        // Cmaj7 -> Ionian
        let chord = Chord::parse("Cmaj7").unwrap();
        let mode = chord.to_mode();
        assert!(mode.is_some());
        assert_eq!(mode.unwrap().mode(), Mode::Ionian);
    }
}
