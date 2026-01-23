//! A module for working with modes.

use std::fmt::{Display, Error, Formatter};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use pest::Parser;

use crate::core::{
    base::{HasDescription, HasName, HasPreciseName, HasStaticName, Parsable, Res},
    chord::HasRoot,
    interval::{HasIntervals, Interval},
    mode_kind::ModeKind,
    note::Note,
    parser::{mode_name_str_to_mode_kind, note_str_to_note, ChordParser, Rule},
};

// Traits.

/// A trait that represents a type that has a mode kind.
pub trait HasModeKind {
    /// Returns the mode kind of the implementor (most likely a [`Mode`]).
    fn kind(&self) -> ModeKind;
}

// Struct.

/// A mode with a root note.
///
/// This combines a root note with a mode kind to produce an actual mode
/// with specific notes.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Mode {
    /// The root note of the mode.
    root: Note,
    /// The kind of mode.
    kind: ModeKind,
}

// Impls.

impl Mode {
    /// Creates a new mode with the given root note and mode kind.
    pub fn new(root: Note, kind: ModeKind) -> Self {
        Self { root, kind }
    }

    /// Returns the intervals of this mode (delegates to the mode kind).
    pub fn intervals(&self) -> &'static [Interval] {
        self.kind.intervals()
    }

    /// Returns the notes of this mode (root + each interval).
    pub fn notes(&self) -> Vec<Note> {
        self.intervals().iter().map(|&interval| self.root + interval).collect()
    }
}

impl HasRoot for Mode {
    fn root(&self) -> Note {
        self.root
    }
}

impl HasModeKind for Mode {
    fn kind(&self) -> ModeKind {
        self.kind
    }
}

impl HasIntervals for Mode {
    fn intervals(&self) -> &'static [Interval] {
        self.kind.intervals()
    }
}

impl HasStaticName for Mode {
    fn static_name(&self) -> &'static str {
        self.kind.static_name()
    }
}

impl HasName for Mode {
    fn name(&self) -> String {
        format!("{} {}", self.root.static_name(), self.kind.static_name())
    }
}

impl HasPreciseName for Mode {
    fn precise_name(&self) -> String {
        format!("{} {}", self.root.name(), self.kind.static_name())
    }
}

impl HasDescription for Mode {
    fn description(&self) -> &'static str {
        self.kind.description()
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let notes = self.notes().iter().map(|n| n.static_name()).collect::<Vec<_>>().join(", ");
        write!(f, "{}\n   {}\n   {}", self.name(), self.description(), notes)
    }
}

impl Parsable for Mode {
    fn parse(input: &str) -> Res<Self>
    where
        Self: Sized,
    {
        let pairs = ChordParser::parse(Rule::mode, input)?;
        let root = pairs.clone().next().unwrap();

        assert_eq!(Rule::mode, root.as_rule());

        let mut components = root.into_inner();

        let note = components.next().unwrap();
        assert_eq!(Rule::note_atomic, note.as_rule());
        let root_note = note_str_to_note(note.as_str().trim())?;

        let mode_name = components.next().unwrap();
        assert_eq!(Rule::mode_name, mode_name.as_rule());
        let mode_kind = mode_name_str_to_mode_kind(mode_name.as_str())?;

        Ok(Mode::new(root_note, mode_kind))
    }
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::named_pitch::{HasLetter, HasNamedPitch};
    use crate::core::note::*;
    use pretty_assertions::assert_eq;

    impl Mode {
        /// Validates that the mode has correct enharmonic spelling.
        ///
        /// For 7-note modes (most modes), each letter A-G should appear exactly once.
        /// For other modes, no letter should repeat unless it's a chromatic/octatonic exception.
        pub(crate) fn validate_spelling(&self) -> Result<(), String> {
            use crate::core::named_pitch::{HasLetter, HasNamedPitch};
            use std::collections::HashMap;

            let notes = self.notes();
            let intervals_count = self.intervals().len();

            // For chromatic scale (12 notes), we allow letter repeats
            if intervals_count == 12 {
                return Ok(());
            }

            // Check for letter uniqueness
            let mut letter_counts: HashMap<&str, usize> = HashMap::new();
            for note in &notes {
                let letter = note.named_pitch().letter();
                *letter_counts.entry(letter).or_insert(0) += 1;
            }

            // For 7-note collections, we expect exactly one of each letter
            if intervals_count == 7 {
                if letter_counts.len() != 7 {
                    return Err(format!(
                        "{} {} has {} unique letters, expected 7. Letters: {:?}",
                        self.root().static_name(),
                        self.kind().static_name(),
                        letter_counts.len(),
                        notes.iter().map(|n| n.static_name()).collect::<Vec<_>>()
                    ));
                }

                for (letter, count) in &letter_counts {
                    if *count != 1 {
                        return Err(format!(
                            "{} {} has letter {} appearing {} times, expected 1. Notes: {:?}",
                            self.root().static_name(),
                            self.kind().static_name(),
                            letter,
                            count,
                            notes.iter().map(|n| n.static_name()).collect::<Vec<_>>()
                        ));
                    }
                }
            } else {
                // For non-7-note collections (pentatonic, etc.), just check no duplicates
                for (letter, count) in &letter_counts {
                    if *count > 1 {
                        return Err(format!(
                            "{} {} has letter {} appearing {} times. Notes: {:?}",
                            self.root().static_name(),
                            self.kind().static_name(),
                            letter,
                            count,
                            notes.iter().map(|n| n.static_name()).collect::<Vec<_>>()
                        ));
                    }
                }
            }

            Ok(())
        }
    }

    #[test]
    fn test_mode_creation() {
        let mode = Mode::new(D, ModeKind::Dorian);
        assert_eq!(mode.root(), D);
        assert_eq!(mode.kind(), ModeKind::Dorian);
    }

    #[test]
    fn test_mode_intervals() {
        let mode = Mode::new(D, ModeKind::Dorian);
        assert_eq!(mode.intervals().len(), 7);
        assert_eq!(
            mode.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ]
        );
    }

    #[test]
    fn test_mode_notes() {
        // D Dorian
        let mode = Mode::new(D, ModeKind::Dorian);
        assert_eq!(mode.notes(), vec![D, E, F, G, A, B, CFive]);

        // C Ionian (same as C major)
        let mode = Mode::new(C, ModeKind::Ionian);
        assert_eq!(mode.notes(), vec![C, D, E, F, G, A, B]);

        // E Phrygian
        let mode = Mode::new(E, ModeKind::Phrygian);
        assert_eq!(mode.notes(), vec![E, F, G, A, B, CFive, DFive]);

        // F Lydian
        let mode = Mode::new(F, ModeKind::Lydian);
        assert_eq!(mode.notes(), vec![F, G, A, B, CFive, DFive, EFive]);

        // G Mixolydian
        let mode = Mode::new(G, ModeKind::Mixolydian);
        assert_eq!(mode.notes(), vec![G, A, B, CFive, DFive, EFive, FFive]);

        // A Aeolian (natural minor)
        let mode = Mode::new(A, ModeKind::Aeolian);
        assert_eq!(mode.notes(), vec![A, B, CFive, DFive, EFive, FFive, GFive]);

        // B Locrian
        let mode = Mode::new(B, ModeKind::Locrian);
        assert_eq!(mode.notes(), vec![B, CFive, DFive, EFive, FFive, GFive, AFive]);
    }

    #[test]
    fn test_mode_names() {
        let mode = Mode::new(D, ModeKind::Dorian);
        assert_eq!(mode.name(), "D dorian");
        assert_eq!(mode.static_name(), "dorian");

        let mode = Mode::new(FSharp, ModeKind::Lydian);
        assert_eq!(mode.name(), "F♯ lydian");

        let mode = Mode::new(BFlat, ModeKind::Mixolydian);
        assert_eq!(mode.name(), "B♭ mixolydian");
    }

    #[test]
    fn test_mode_display() {
        let mode = Mode::new(D, ModeKind::Dorian);
        let display = format!("{}", mode);
        assert!(display.contains("D dorian"));
        assert!(display.contains("D, E, F, G, A, B, C"));
        assert!(display.contains("dorian"));
    }

    #[test]
    fn test_all_modes_of_c_major() {
        // All modes of C major scale should contain the same note classes (C, D, E, F, G, A, B)
        // but starting from different degrees. Notes may be in different octaves.

        let c_ionian = Mode::new(C, ModeKind::Ionian);
        assert_eq!(c_ionian.notes(), vec![C, D, E, F, G, A, B]);

        let d_dorian = Mode::new(D, ModeKind::Dorian);
        assert_eq!(d_dorian.notes(), vec![D, E, F, G, A, B, CFive]);

        let e_phrygian = Mode::new(E, ModeKind::Phrygian);
        assert_eq!(e_phrygian.notes(), vec![E, F, G, A, B, CFive, DFive]);

        let f_lydian = Mode::new(F, ModeKind::Lydian);
        assert_eq!(f_lydian.notes(), vec![F, G, A, B, CFive, DFive, EFive]);

        let g_mixolydian = Mode::new(G, ModeKind::Mixolydian);
        assert_eq!(g_mixolydian.notes(), vec![G, A, B, CFive, DFive, EFive, FFive]);

        let a_aeolian = Mode::new(A, ModeKind::Aeolian);
        assert_eq!(a_aeolian.notes(), vec![A, B, CFive, DFive, EFive, FFive, GFive]);

        let b_locrian = Mode::new(B, ModeKind::Locrian);
        assert_eq!(b_locrian.notes(), vec![B, CFive, DFive, EFive, FFive, GFive, AFive]);
    }

    #[test]
    fn test_mode_characteristic_intervals() {
        // D Dorian characteristic: major 6th (B) in minor context
        let mode = Mode::new(D, ModeKind::Dorian);
        let notes = mode.notes();
        assert_eq!(notes[5], B); // 6th degree is major 6th

        // E Phrygian characteristic: minor 2nd (F)
        let mode = Mode::new(E, ModeKind::Phrygian);
        let notes = mode.notes();
        assert_eq!(notes[1], F); // 2nd degree is minor 2nd

        // F Lydian characteristic: augmented 4th (B)
        let mode = Mode::new(F, ModeKind::Lydian);
        let notes = mode.notes();
        assert_eq!(notes[3], B); // 4th degree is augmented 4th

        // B Locrian characteristic: diminished 5th (F)
        let mode = Mode::new(B, ModeKind::Locrian);
        let notes = mode.notes();
        assert_eq!(notes[4], FFive); // 5th degree is diminished 5th
    }

    #[test]
    fn test_mode_parse() {
        // Test parsing various modes
        let mode = Mode::parse("D dorian").unwrap();
        assert_eq!(mode.root(), D);
        assert_eq!(mode.kind(), ModeKind::Dorian);

        let mode = Mode::parse("C ionian").unwrap();
        assert_eq!(mode.root(), C);
        assert_eq!(mode.kind(), ModeKind::Ionian);

        let mode = Mode::parse("E phrygian").unwrap();
        assert_eq!(mode.root(), E);
        assert_eq!(mode.kind(), ModeKind::Phrygian);

        let mode = Mode::parse("F lydian").unwrap();
        assert_eq!(mode.root(), F);
        assert_eq!(mode.kind(), ModeKind::Lydian);

        let mode = Mode::parse("G mixolydian").unwrap();
        assert_eq!(mode.root(), G);
        assert_eq!(mode.kind(), ModeKind::Mixolydian);

        let mode = Mode::parse("A aeolian").unwrap();
        assert_eq!(mode.root(), A);
        assert_eq!(mode.kind(), ModeKind::Aeolian);

        let mode = Mode::parse("B locrian").unwrap();
        assert_eq!(mode.root(), B);
        assert_eq!(mode.kind(), ModeKind::Locrian);

        // Test with accidentals
        let mode = Mode::parse("F# dorian").unwrap();
        assert_eq!(mode.root(), FSharp);
        assert_eq!(mode.kind(), ModeKind::Dorian);

        let mode = Mode::parse("Bb lydian").unwrap();
        assert_eq!(mode.root(), BFlat);
        assert_eq!(mode.kind(), ModeKind::Lydian);
    }

    #[test]
    fn test_harmonic_minor_modes_parse() {
        // Test harmonic minor modes
        let mode = Mode::parse("B locrian nat6").unwrap();
        assert_eq!(mode.kind(), ModeKind::LocrianNatural6);

        let mode = Mode::parse("C ionian #5").unwrap();
        assert_eq!(mode.kind(), ModeKind::IonianSharp5);

        let mode = Mode::parse("D dorian sharp 4").unwrap();
        assert_eq!(mode.kind(), ModeKind::DorianSharp4);

        let mode = Mode::parse("E phrygian dominant").unwrap();
        assert_eq!(mode.kind(), ModeKind::PhrygianDominant);

        let mode = Mode::parse("F lydian #2").unwrap();
        assert_eq!(mode.kind(), ModeKind::LydianSharp2);

        let mode = Mode::parse("G# ultralocrian").unwrap();
        assert_eq!(mode.kind(), ModeKind::Ultralocrian);
    }

    #[test]
    fn test_melodic_minor_modes_parse() {
        // Test melodic minor modes
        let mode = Mode::parse("B dorian b2").unwrap();
        assert_eq!(mode.kind(), ModeKind::DorianFlat2);

        let mode = Mode::parse("C lydian augmented").unwrap();
        assert_eq!(mode.kind(), ModeKind::LydianAugmented);

        let mode = Mode::parse("D lydian dominant").unwrap();
        assert_eq!(mode.kind(), ModeKind::LydianDominant);

        let mode = Mode::parse("E mixolydian b6").unwrap();
        assert_eq!(mode.kind(), ModeKind::MixolydianFlat6);

        let mode = Mode::parse("F# locrian nat2").unwrap();
        assert_eq!(mode.kind(), ModeKind::LocrianNatural2);

        let mode = Mode::parse("G# altered").unwrap();
        assert_eq!(mode.kind(), ModeKind::Altered);
    }

    #[test]
    fn test_enharmonic_spelling_diatonic_modes() {
        // Test all diatonic modes with various root notes to ensure correct enharmonic spelling
        // Each 7-note mode should use each letter A-G exactly once

        // C Ionian - all natural notes
        let mode = Mode::new(C, ModeKind::Ionian);
        mode.validate_spelling().unwrap();

        // F# Dorian - should use sharps, not flats
        let mode = Mode::new(FSharp, ModeKind::Dorian);
        mode.validate_spelling().unwrap();
        let notes = mode.notes();
        // F# Dorian: F# G# A B C# D E#
        assert_eq!(notes[0].named_pitch().letter(), "F");
        assert_eq!(notes[1].named_pitch().letter(), "G");
        assert_eq!(notes[2].named_pitch().letter(), "A");
        assert_eq!(notes[3].named_pitch().letter(), "B");
        assert_eq!(notes[4].named_pitch().letter(), "C");
        assert_eq!(notes[5].named_pitch().letter(), "D");
        assert_eq!(notes[6].named_pitch().letter(), "E");

        // Db Lydian - should use flats
        let mode = Mode::new(DFlat, ModeKind::Lydian);
        mode.validate_spelling().unwrap();
        let notes = mode.notes();
        // Db Lydian: Db Eb F G Ab Bb C
        assert_eq!(notes[0].named_pitch().letter(), "D");
        assert_eq!(notes[1].named_pitch().letter(), "E");
        assert_eq!(notes[2].named_pitch().letter(), "F");
        assert_eq!(notes[3].named_pitch().letter(), "G");
        assert_eq!(notes[4].named_pitch().letter(), "A");
        assert_eq!(notes[5].named_pitch().letter(), "B");
        assert_eq!(notes[6].named_pitch().letter(), "C");

        // Bb Mixolydian
        let mode = Mode::new(BFlat, ModeKind::Mixolydian);
        mode.validate_spelling().unwrap();

        // E Locrian
        let mode = Mode::new(E, ModeKind::Locrian);
        mode.validate_spelling().unwrap();
    }

    #[test]
    fn test_enharmonic_spelling_harmonic_minor_modes() {
        // Test harmonic minor modes

        // F# Locrian Natural 6 - should spell with F# G A B C# D E#
        let mode = Mode::new(FSharp, ModeKind::LocrianNatural6);
        mode.validate_spelling().unwrap();

        // C Ionian #5
        let mode = Mode::new(C, ModeKind::IonianSharp5);
        mode.validate_spelling().unwrap();

        // D Dorian #4
        let mode = Mode::new(D, ModeKind::DorianSharp4);
        mode.validate_spelling().unwrap();

        // E Phrygian Dominant
        let mode = Mode::new(E, ModeKind::PhrygianDominant);
        mode.validate_spelling().unwrap();

        // F Lydian #2
        let mode = Mode::new(F, ModeKind::LydianSharp2);
        mode.validate_spelling().unwrap();

        // G# Ultralocrian
        let mode = Mode::new(GSharp, ModeKind::Ultralocrian);
        mode.validate_spelling().unwrap();
    }

    #[test]
    fn test_enharmonic_spelling_melodic_minor_modes() {
        // Test melodic minor modes

        // B Dorian b2
        let mode = Mode::new(B, ModeKind::DorianFlat2);
        mode.validate_spelling().unwrap();

        // C Lydian Augmented
        let mode = Mode::new(C, ModeKind::LydianAugmented);
        mode.validate_spelling().unwrap();

        // D Lydian Dominant
        let mode = Mode::new(D, ModeKind::LydianDominant);
        mode.validate_spelling().unwrap();

        // E Mixolydian b6
        let mode = Mode::new(E, ModeKind::MixolydianFlat6);
        mode.validate_spelling().unwrap();

        // F# Locrian natural 2
        let mode = Mode::new(FSharp, ModeKind::LocrianNatural2);
        mode.validate_spelling().unwrap();

        // G# Altered
        let mode = Mode::new(GSharp, ModeKind::Altered);
        mode.validate_spelling().unwrap();
    }

    #[test]
    fn test_enharmonic_spelling_all_roots() {
        // Test a few modes with all 12 root notes to ensure consistency
        for root in [C, CSharp, D, DFlat, DSharp, E, EFlat, F, FSharp, G, GFlat, GSharp, A, AFlat, ASharp, B, BFlat] {
            // Ionian (Major)
            let mode = Mode::new(root, ModeKind::Ionian);
            mode.validate_spelling().unwrap_or_else(|e| panic!("Ionian spelling failed for {}: {}", root.static_name(), e));

            // Dorian
            let mode = Mode::new(root, ModeKind::Dorian);
            mode.validate_spelling().unwrap_or_else(|e| panic!("Dorian spelling failed for {}: {}", root.static_name(), e));

            // Lydian
            let mode = Mode::new(root, ModeKind::Lydian);
            mode.validate_spelling().unwrap_or_else(|e| panic!("Lydian spelling failed for {}: {}", root.static_name(), e));
        }
    }

    #[test]
    fn test_mode_spelling() {
        let mode = Mode::new(E, ModeKind::PhrygianDominant);
        assert_eq!(mode.notes(), vec![E, F, GSharp, A, B, CFive, DFive], "E phrygian dominant spelling incorrect");
        mode.validate_spelling().unwrap();

        let mode = Mode::new(B, ModeKind::LocrianNatural6);
        assert_eq!(mode.notes(), vec![B, CFive, DFive, EFive, FFive, GSharpFive, AFive], "B locrian nat6 spelling incorrect");
        mode.validate_spelling().unwrap();

        let mode = Mode::new(D, ModeKind::LydianDominant);
        assert_eq!(mode.notes(), vec![D, E, FSharp, GSharp, A, B, CFive], "D lydian dominant spelling incorrect");
        mode.validate_spelling().unwrap();

        assert_eq!(
            mode.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ]
        );
    }
}
