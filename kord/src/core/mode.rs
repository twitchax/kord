//! A module for working with musical modes (Greek modes).

use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{
    base::{HasDescription, HasName, HasStaticName, Parsable, Res},
    chord::{Chord, Chordable},
    scale::HasIntervals,
    interval::Interval,
    note::Note,
};

// Traits.

/// A trait for types that can be converted to a chord.
pub trait ToChord {
    /// Converts the implementor to a chord.
    fn to_chord(&self) -> Chord;
}

// Enum.

/// An enum representing musical mode kinds (Greek modes, harmonic minor modes, and melodic minor modes).
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ModeKind {
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

/// A struct representing a rooted mode instance.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Mode {
    /// The root note of the mode.
    root: Note,
    /// The mode.
    mode: ModeKind,
}

// Impls.

impl HasStaticName for ModeKind {
    fn static_name(&self) -> &'static str {
        match self {
            ModeKind::Ionian => "Ionian",
            ModeKind::Dorian => "Dorian",
            ModeKind::Phrygian => "Phrygian",
            ModeKind::Lydian => "Lydian",
            ModeKind::Mixolydian => "Mixolydian",
            ModeKind::Aeolian => "Aeolian",
            ModeKind::Locrian => "Locrian",
            ModeKind::LocrianNatural6 => "Locrian ♮6",
            ModeKind::IonianAugmented => "Ionian ♯5",
            ModeKind::DorianSharp4 => "Dorian ♯4",
            ModeKind::PhrygianDominant => "Phrygian Dominant",
            ModeKind::LydianSharp2 => "Lydian ♯2",
            ModeKind::Ultralocrian => "Ultralocrian",
            ModeKind::DorianFlat2 => "Dorian ♭2",
            ModeKind::LydianAugmented => "Lydian Augmented",
            ModeKind::LydianDominant => "Lydian Dominant",
            ModeKind::MixolydianFlat6 => "Mixolydian ♭6",
            ModeKind::LocrianNatural2 => "Locrian ♮2",
            ModeKind::Altered => "Altered",
        }
    }
}

impl HasDescription for ModeKind {
    fn description(&self) -> &'static str {
        match self {
            ModeKind::Ionian => "major scale, first mode of major scale",
            ModeKind::Dorian => "second mode of major scale, minor with raised 6th",
            ModeKind::Phrygian => "third mode of major scale, minor with lowered 2nd",
            ModeKind::Lydian => "fourth mode of major scale, major with raised 4th",
            ModeKind::Mixolydian => "fifth mode of major scale, major with lowered 7th",
            ModeKind::Aeolian => "natural minor scale, sixth mode of major scale",
            ModeKind::Locrian => "seventh mode of major scale, diminished scale with lowered 2nd and 5th",
            ModeKind::LocrianNatural6 => "2nd mode of harmonic minor, common over m7♭5(♮13) colors",
            ModeKind::IonianAugmented => "3rd mode of harmonic minor, major with sharp 5",
            ModeKind::DorianSharp4 => "4th mode of harmonic minor, minor with a Lydian bite",
            ModeKind::PhrygianDominant => "5th mode of harmonic minor, classic V in minor, Spanish sound",
            ModeKind::LydianSharp2 => "6th mode of harmonic minor, bright and exotic with sharp 2 and sharp 4",
            ModeKind::Ultralocrian => "7th mode of harmonic minor, rare, very unstable and dark",
            ModeKind::DorianFlat2 => "2nd mode of melodic minor, minor with spicy flat 2 but strong 6",
            ModeKind::LydianAugmented => "3rd mode of melodic minor, Lydian with sharp 5 sheen",
            ModeKind::LydianDominant => "4th mode of melodic minor, the dominant-with-sharp-11 sound, also known as the acoustic scale",
            ModeKind::MixolydianFlat6 => "5th mode of melodic minor, dominant that leans minor with flat 13",
            ModeKind::LocrianNatural2 => "6th mode of melodic minor, m7♭5 with a cleaner 2",
            ModeKind::Altered => "7th mode of melodic minor, the go-to for V7alt, also known as Super Locrian",
        }
    }
}

impl HasIntervals for ModeKind {
    fn intervals(&self) -> &'static [Interval] {
        match self {
            ModeKind::Ionian => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            ModeKind::Dorian => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            ModeKind::Phrygian => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            ModeKind::Lydian => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            ModeKind::Mixolydian => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            ModeKind::Aeolian => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            ModeKind::Locrian => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            // Harmonic minor modes
            ModeKind::LocrianNatural6 => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            ModeKind::IonianAugmented => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::AugmentedFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            ModeKind::DorianSharp4 => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            ModeKind::PhrygianDominant => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            ModeKind::LydianSharp2 => &[
                Interval::PerfectUnison,
                Interval::AugmentedSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            ModeKind::Ultralocrian => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::DiminishedFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::DiminishedSeventh,
            ],
            // Melodic minor modes
            ModeKind::DorianFlat2 => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            ModeKind::LydianAugmented => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::AugmentedFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            ModeKind::LydianDominant => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            ModeKind::MixolydianFlat6 => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            ModeKind::LocrianNatural2 => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            ModeKind::Altered => &[
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

impl Display for ModeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.static_name())
    }
}

impl Parsable for ModeKind {
    fn parse(input: &str) -> Res<Self>
    where
        Self: Sized,
    {
        let lower = input.to_lowercase();
        match lower.as_str() {
            "ionian" => Ok(ModeKind::Ionian),
            "dorian" => Ok(ModeKind::Dorian),
            "phrygian" => Ok(ModeKind::Phrygian),
            "lydian" => Ok(ModeKind::Lydian),
            "mixolydian" => Ok(ModeKind::Mixolydian),
            "aeolian" => Ok(ModeKind::Aeolian),
            "locrian" => Ok(ModeKind::Locrian),
            // Harmonic minor modes
            "locrian natural 6" | "locrian nat6" | "locrian ♮6" => Ok(ModeKind::LocrianNatural6),
            "ionian augmented" | "ionian ♯5" | "major ♯5" | "augmented major" => Ok(ModeKind::IonianAugmented),
            "dorian ♯4" | "dorian sharp 4" => Ok(ModeKind::DorianSharp4),
            "phrygian dominant" | "spanish phrygian" | "phrygian major" => Ok(ModeKind::PhrygianDominant),
            "lydian ♯2" | "lydian sharp 2" => Ok(ModeKind::LydianSharp2),
            "ultralocrian" | "ultra locrian" => Ok(ModeKind::Ultralocrian),
            // Melodic minor modes
            "dorian ♭2" | "dorian flat 2" | "phrygian ♮6" => Ok(ModeKind::DorianFlat2),
            "lydian augmented" | "lydian ♯5" | "lydian sharp 5" => Ok(ModeKind::LydianAugmented),
            "lydian dominant" | "lydian ♭7" | "mixolydian ♯4" | "acoustic scale" => Ok(ModeKind::LydianDominant),
            "mixolydian ♭6" | "mixolydian flat 6" | "aeolian dominant" => Ok(ModeKind::MixolydianFlat6),
            "locrian ♮2" | "locrian natural 2" | "locrian ♯2" | "half-diminished ♮2" => Ok(ModeKind::LocrianNatural2),
            "altered" | "super locrian" | "altered scale" => Ok(ModeKind::Altered),
            _ => Err(anyhow::Error::msg(format!("Unknown mode: {}", input))),
        }
    }
}

impl Mode {
    /// Creates a new mode with a root note and kind.
    pub fn new(root: Note, kind: ModeKind) -> Self {
        Self { root, mode: kind }
    }

    /// Returns the root note of the mode.
    pub fn root(&self) -> Note {
        self.root
    }

    /// Returns the mode kind.
    pub fn kind(&self) -> ModeKind {
        self.mode
    }

    /// Returns the scale for this mode with the root note.
    pub fn scale(&self) -> Vec<Note> {
        self.mode.intervals().iter().map(|&i| self.root + i).collect()
    }
}

impl HasName for Mode {
    fn name(&self) -> String {
        format!("{} {}", self.root.static_name(), self.mode.static_name())
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scale = self.scale().iter().map(HasStaticName::static_name).collect::<Vec<_>>().join(", ");
        write!(f, "{}\n   {}\n   {}", self.name(), self.mode.description(), scale)
    }
}

impl Parsable for Mode {
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
        let mode = ModeKind::parse(mode_str.as_str())?;

        Ok(Mode::new(root, mode))
    }
}

// ToChord and ToMode implementations

impl ToChord for Mode {
    fn to_chord(&self) -> Chord {
        let chord = Chord::new(self.root);
        
        match self.mode {
            // Major scale modes
            ModeKind::Ionian => chord, // Just major
            ModeKind::Dorian => chord.minor().seven(),
            ModeKind::Phrygian => chord.minor().seven().flat9().flat13(),
            ModeKind::Lydian => chord.major7().sharp11(),
            ModeKind::Mixolydian => chord.seven(),
            ModeKind::Aeolian => chord.minor().seven().flat13(),
            ModeKind::Locrian => chord.minor().seven().flat5(),
            
            // Harmonic minor modes
            ModeKind::LocrianNatural6 => chord.minor().seven().flat5().add13(),
            ModeKind::IonianAugmented => chord.major7().augmented(),
            ModeKind::DorianSharp4 => chord.minor().seven().sharp11(),
            ModeKind::PhrygianDominant => chord.seven().flat9(),
            ModeKind::LydianSharp2 => chord.major7().sharp11(),
            ModeKind::Ultralocrian => chord.diminished().seven(),
            
            // Melodic minor modes
            ModeKind::DorianFlat2 => chord.minor().seven().flat9(),
            ModeKind::LydianAugmented => chord.major7().augmented().sharp11(),
            ModeKind::LydianDominant => chord.seven().sharp11(),
            ModeKind::MixolydianFlat6 => chord.seven().flat13(),
            ModeKind::LocrianNatural2 => chord.minor().seven().flat5(),
            ModeKind::Altered => chord.seven().flat9().sharp9(), // 7alt chord
        }
    }
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::chord::ToMode;
    use crate::core::note::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_mode_names() {
        assert_eq!(ModeKind::Ionian.static_name(), "Ionian");
        assert_eq!(ModeKind::Dorian.static_name(), "Dorian");
        assert_eq!(ModeKind::Phrygian.static_name(), "Phrygian");
        assert_eq!(ModeKind::Lydian.static_name(), "Lydian");
        assert_eq!(ModeKind::Mixolydian.static_name(), "Mixolydian");
        assert_eq!(ModeKind::Aeolian.static_name(), "Aeolian");
        assert_eq!(ModeKind::Locrian.static_name(), "Locrian");
    }

    #[test]
    fn test_mode_scales() {
        // Ionian (C major)
        assert_eq!(Mode::new(C, ModeKind::Ionian).scale(), &[C, D, E, F, G, A, B]);
        
        // Dorian (C Dorian)
        assert_eq!(Mode::new(C, ModeKind::Dorian).scale(), &[C, D, EFlat, F, G, A, BFlat]);
        
        // Phrygian (E Phrygian = notes of C major starting on E)
        assert_eq!(Mode::new(E, ModeKind::Phrygian).scale(), &[E, F, G, A, B, CFive, DFive]);
        
        // Lydian (F Lydian = notes of C major starting on F)
        assert_eq!(Mode::new(F, ModeKind::Lydian).scale(), &[F, G, A, B, CFive, DFive, EFive]);
        
        // Mixolydian (G Mixolydian = notes of C major starting on G)
        assert_eq!(Mode::new(G, ModeKind::Mixolydian).scale(), &[G, A, B, CFive, DFive, EFive, FFive]);
        
        // Aeolian (A Aeolian = A natural minor)
        assert_eq!(Mode::new(A, ModeKind::Aeolian).scale(), &[A, B, CFive, DFive, EFive, FFive, GFive]);
        
        // Locrian (B Locrian = notes of C major starting on B)
        assert_eq!(Mode::new(B, ModeKind::Locrian).scale(), &[B, CFive, DFive, EFive, FFive, GFive, AFive]);
    }

    #[test]
    fn test_mode_parse() {
        assert_eq!(ModeKind::parse("Ionian").unwrap(), ModeKind::Ionian);
        assert_eq!(ModeKind::parse("ionian").unwrap(), ModeKind::Ionian);
        assert_eq!(ModeKind::parse("Dorian").unwrap(), ModeKind::Dorian);
        assert_eq!(ModeKind::parse("dorian").unwrap(), ModeKind::Dorian);
        assert_eq!(ModeKind::parse("Phrygian").unwrap(), ModeKind::Phrygian);
        assert_eq!(ModeKind::parse("Lydian").unwrap(), ModeKind::Lydian);
        assert_eq!(ModeKind::parse("Mixolydian").unwrap(), ModeKind::Mixolydian);
        assert_eq!(ModeKind::parse("Aeolian").unwrap(), ModeKind::Aeolian);
        assert_eq!(ModeKind::parse("Locrian").unwrap(), ModeKind::Locrian);
    }

    #[test]
    fn test_mode_with_root_parse() {
        let mode = Mode::parse("C Dorian").unwrap();
        assert_eq!(mode.root(), C);
        assert_eq!(mode.kind(), ModeKind::Dorian);
        assert_eq!(mode.scale(), &[C, D, EFlat, F, G, A, BFlat]);

        let mode = Mode::parse("D Phrygian").unwrap();
        assert_eq!(mode.root(), D);
        assert_eq!(mode.kind(), ModeKind::Phrygian);
    }

    #[test]
    fn test_mode_display() {
        let mode = Mode::new(C, ModeKind::Dorian);
        let display = format!("{}", mode);
        assert!(display.contains("C Dorian"));
        assert!(display.contains("C, D, E♭, F, G, A, B♭"));
    }

    #[test]
    fn test_harmonic_minor_modes() {
        // Phrygian Dominant (E Phrygian Dominant from A harmonic minor)
        assert_eq!(
            Mode::new(E, ModeKind::PhrygianDominant).scale(),
            &[E, F, GSharp, A, B, CFive, DFive]
        );
        
        // Lydian Sharp 2
        assert_eq!(
            Mode::new(F, ModeKind::LydianSharp2).scale(),
            &[F, GSharp, A, B, CFive, DFive, EFive]
        );
    }

    #[test]
    fn test_melodic_minor_modes() {
        // Dorian Flat 2 (B Dorian ♭2 from A melodic minor)
        assert_eq!(
            Mode::new(B, ModeKind::DorianFlat2).scale(),
            &[B, CFive, DFive, EFive, FSharpFive, GSharpFive, AFive]
        );
        
        // Lydian Dominant
        assert_eq!(
            Mode::new(D, ModeKind::LydianDominant).scale(),
            &[D, E, FSharp, GSharp, A, B, CFive]
        );
        
        // Altered mode - using enharmonically correct notes
        // G# Altered scale intervals produce: G# A A## (B enharmonic) C (as octave 5) D E F#
        let altered_scale = Mode::new(GSharp, ModeKind::Altered).scale();
        assert_eq!(altered_scale.len(), 7);
        assert_eq!(altered_scale[0], GSharp); // Root
        assert_eq!(altered_scale[1], A); // ♭2
        // Note: altered_scale[2] will be ADoubleSharp (enharmonic to B)
        // Note: altered_scale[3] will be CFive (enharmonic to BSharp of octave 4)
    }

    #[test]
    fn test_case_insensitive_parsing() {
        assert_eq!(ModeKind::parse("IONIAN").unwrap(), ModeKind::Ionian);
        assert_eq!(ModeKind::parse("DoRiAn").unwrap(), ModeKind::Dorian);
        assert_eq!(ModeKind::parse("mixolydian").unwrap(), ModeKind::Mixolydian);
        assert_eq!(ModeKind::parse("ALTERED").unwrap(), ModeKind::Altered);
        assert_eq!(ModeKind::parse("lydian dominant").unwrap(), ModeKind::LydianDominant);
    }

    #[test]
    fn test_mode_to_chord() {
        // Dorian -> Dm7
        let dorian = Mode::new(D, ModeKind::Dorian);
        let chord = dorian.to_chord();
        assert_eq!(chord.name(), "Dm7");
        
        // Lydian -> maj7#11
        let lydian = Mode::new(F, ModeKind::Lydian);
        let chord = lydian.to_chord();
        assert!(chord.name().contains("maj7"));
        assert!(chord.name().contains("♯11"));
        
        // Mixolydian -> 7
        let mixolydian = Mode::new(G, ModeKind::Mixolydian);
        let chord = mixolydian.to_chord();
        assert_eq!(chord.name(), "G7");
    }

    #[test]
    fn test_chord_to_mode() {
        // Dm7 -> Dorian
        let chord = Chord::parse("Dm7").unwrap();
        let mode = chord.to_mode();
        assert!(mode.is_some());
        assert_eq!(mode.unwrap().kind(), ModeKind::Dorian);
        
        // G7 -> Mixolydian
        let chord = Chord::parse("G7").unwrap();
        let mode = chord.to_mode();
        assert!(mode.is_some());
        assert_eq!(mode.unwrap().kind(), ModeKind::Mixolydian);
        
        // Cmaj7 -> Ionian
        let chord = Chord::parse("Cmaj7").unwrap();
        let mode = chord.to_mode();
        assert!(mode.is_some());
        assert_eq!(mode.unwrap().kind(), ModeKind::Ionian);
    }
}
