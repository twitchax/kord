//! A module for working with scales.

use std::fmt::{Display, Error, Formatter};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use pest::Parser;

use crate::core::{
    base::{HasDescription, HasName, HasPreciseName, HasStaticName, Parsable, Res},
    chord::HasRoot,
    interval::{HasIntervals, Interval},
    note::Note,
    parser::{note_str_to_note, scale_name_str_to_scale_kind, ChordParser, Rule},
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

    /// Validates that the scale has correct enharmonic spelling.
    /// 
    /// For 7-note scales (major, natural minor, harmonic minor, melodic minor, modes),
    /// each letter A-G should appear exactly once.
    /// For other scales, no letter should repeat unless it's a chromatic/octatonic/blues exception.
    /// Blues scale duplicates the 4th degree letter (e.g., F and F# in C blues).
    #[cfg(test)]
    pub(crate) fn validate_spelling(&self) -> Result<(), String> {
        use std::collections::HashMap;
        use crate::core::named_pitch::{HasLetter, HasNamedPitch};
        
        let notes = self.notes();
        let intervals_count = self.intervals().len();
        
        // For chromatic scale (12 notes), octatonic (8 notes), and blues (6 notes with ♯4 duplicating 4th degree), we allow letter repeats
        if intervals_count == 12 || intervals_count == 8 || self.kind() == ScaleKind::Blues {
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
            // For non-7-note collections (pentatonic, whole tone), just check no duplicates
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

impl Parsable for Scale {
    fn parse(input: &str) -> Res<Self>
    where
        Self: Sized,
    {
        let root = ChordParser::parse(Rule::scale, input)?.next().unwrap();

        assert_eq!(Rule::scale, root.as_rule());

        let mut components = root.into_inner();

        let note = components.next().unwrap();
        assert_eq!(Rule::note_atomic, note.as_rule());
        let root_note = note_str_to_note(note.as_str().trim())?;

        let scale_name = components.next().unwrap();
        assert_eq!(Rule::scale_name, scale_name.as_rule());
        let scale_kind = scale_name_str_to_scale_kind(scale_name.as_str())?;

        Ok(Scale::new(root_note, scale_kind))
    }
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::note::*;
    use crate::core::named_pitch::{HasNamedPitch, HasLetter};
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

    #[test]
    fn test_scale_parse() {
        // Test parsing various scales
        let scale = Scale::parse("C major").unwrap();
        assert_eq!(scale.root(), C);
        assert_eq!(scale.kind(), ScaleKind::Major);

        let scale = Scale::parse("A natural minor").unwrap();
        assert_eq!(scale.root(), A);
        assert_eq!(scale.kind(), ScaleKind::NaturalMinor);

        let scale = Scale::parse("A naturalminor").unwrap();
        assert_eq!(scale.root(), A);
        assert_eq!(scale.kind(), ScaleKind::NaturalMinor);

        let scale = Scale::parse("A harmonic minor").unwrap();
        assert_eq!(scale.root(), A);
        assert_eq!(scale.kind(), ScaleKind::HarmonicMinor);

        let scale = Scale::parse("A melodic minor").unwrap();
        assert_eq!(scale.root(), A);
        assert_eq!(scale.kind(), ScaleKind::MelodicMinor);

        let scale = Scale::parse("C whole tone").unwrap();
        assert_eq!(scale.root(), C);
        assert_eq!(scale.kind(), ScaleKind::WholeTone);

        let scale = Scale::parse("C chromatic").unwrap();
        assert_eq!(scale.root(), C);
        assert_eq!(scale.kind(), ScaleKind::Chromatic);

        // Test with accidentals
        let scale = Scale::parse("F# major").unwrap();
        assert_eq!(scale.root(), FSharp);
        assert_eq!(scale.kind(), ScaleKind::Major);

        let scale = Scale::parse("Bb harmonic minor").unwrap();
        assert_eq!(scale.root(), BFlat);
        assert_eq!(scale.kind(), ScaleKind::HarmonicMinor);

        // Test pentatonic scales
        let scale = Scale::parse("C major pentatonic").unwrap();
        assert_eq!(scale.root(), C);
        assert_eq!(scale.kind(), ScaleKind::MajorPentatonic);

        let scale = Scale::parse("C majorpentatonic").unwrap();
        assert_eq!(scale.root(), C);
        assert_eq!(scale.kind(), ScaleKind::MajorPentatonic);

        let scale = Scale::parse("A minor pentatonic").unwrap();
        assert_eq!(scale.root(), A);
        assert_eq!(scale.kind(), ScaleKind::MinorPentatonic);

        let scale = Scale::parse("A minorpentatonic").unwrap();
        assert_eq!(scale.root(), A);
        assert_eq!(scale.kind(), ScaleKind::MinorPentatonic);

        // Test blues scale
        let scale = Scale::parse("C blues").unwrap();
        assert_eq!(scale.root(), C);
        assert_eq!(scale.kind(), ScaleKind::Blues);

        let scale = Scale::parse("E blues").unwrap();
        assert_eq!(scale.root(), E);
        assert_eq!(scale.kind(), ScaleKind::Blues);
    }

    #[test]
    fn test_enharmonic_spelling_major_scales() {
        // Test major scales with various roots to ensure correct enharmonic spelling
        
        // C Major - all natural notes
        let scale = Scale::new(C, ScaleKind::Major);
        scale.validate_spelling().unwrap();
        
        // G Major - should be G A B C D E F#
        let scale = Scale::new(G, ScaleKind::Major);
        scale.validate_spelling().unwrap();
        let notes = scale.notes();
        assert_eq!(notes.len(), 7);
        
        // Db Major - should use flats: Db Eb F Gb Ab Bb C
        let scale = Scale::new(DFlat, ScaleKind::Major);
        scale.validate_spelling().unwrap();
        let notes = scale.notes();
        assert_eq!(notes[0].named_pitch().letter(), "D");
        assert_eq!(notes[1].named_pitch().letter(), "E");
        assert_eq!(notes[2].named_pitch().letter(), "F");
        assert_eq!(notes[3].named_pitch().letter(), "G");
        assert_eq!(notes[4].named_pitch().letter(), "A");
        assert_eq!(notes[5].named_pitch().letter(), "B");
        assert_eq!(notes[6].named_pitch().letter(), "C");
        
        // F# Major - should use sharps: F# G# A# B C# D# E#
        let scale = Scale::new(FSharp, ScaleKind::Major);
        scale.validate_spelling().unwrap();
        let notes = scale.notes();
        assert_eq!(notes[0].named_pitch().letter(), "F");
        assert_eq!(notes[1].named_pitch().letter(), "G");
        assert_eq!(notes[2].named_pitch().letter(), "A");
        assert_eq!(notes[3].named_pitch().letter(), "B");
        assert_eq!(notes[4].named_pitch().letter(), "C");
        assert_eq!(notes[5].named_pitch().letter(), "D");
        assert_eq!(notes[6].named_pitch().letter(), "E");
    }

    #[test]
    fn test_enharmonic_spelling_minor_scales() {
        // Test minor scales
        
        // A Natural Minor
        let scale = Scale::new(A, ScaleKind::NaturalMinor);
        scale.validate_spelling().unwrap();
        
        // A Harmonic Minor - A B C D E F G#
        let scale = Scale::new(A, ScaleKind::HarmonicMinor);
        scale.validate_spelling().unwrap();
        let notes = scale.notes();
        assert_eq!(notes[6].named_pitch().letter(), "G"); // Should be G#, not Ab
        
        // C# Harmonic Minor
        let scale = Scale::new(CSharp, ScaleKind::HarmonicMinor);
        scale.validate_spelling().unwrap();
        
        // F Melodic Minor
        let scale = Scale::new(F, ScaleKind::MelodicMinor);
        scale.validate_spelling().unwrap();
    }

    #[test]
    fn test_enharmonic_spelling_pentatonic_scales() {
        // Test pentatonic scales (5 notes, no letter repeats)
        
        // C Major Pentatonic - C D E G A
        let scale = Scale::new(C, ScaleKind::MajorPentatonic);
        scale.validate_spelling().unwrap();
        let notes = scale.notes();
        assert_eq!(notes.len(), 5);
        
        // A Minor Pentatonic - A C D E G
        let scale = Scale::new(A, ScaleKind::MinorPentatonic);
        scale.validate_spelling().unwrap();
        let notes = scale.notes();
        assert_eq!(notes.len(), 5);
        
        // F# Major Pentatonic
        let scale = Scale::new(FSharp, ScaleKind::MajorPentatonic);
        scale.validate_spelling().unwrap();
        
        // Bb Minor Pentatonic
        let scale = Scale::new(BFlat, ScaleKind::MinorPentatonic);
        scale.validate_spelling().unwrap();
    }

    #[test]
    fn test_enharmonic_spelling_blues_scale() {
        // Test blues scale (6 notes, allows letter duplication on 4th degree)
        
        // C Blues - C Eb F F# G Bb (F and F# both present)
        let scale = Scale::new(C, ScaleKind::Blues);
        scale.validate_spelling().unwrap();
        let notes = scale.notes();
        assert_eq!(notes.len(), 6);
        
        // E Blues
        let scale = Scale::new(E, ScaleKind::Blues);
        scale.validate_spelling().unwrap();
        
        // G Blues
        let scale = Scale::new(G, ScaleKind::Blues);
        scale.validate_spelling().unwrap();
    }

    #[test]
    fn test_enharmonic_spelling_whole_tone() {
        // Test whole tone scale (6 notes, no letter repeats)
        
        // C Whole Tone - C D E F# G# A#
        let scale = Scale::new(C, ScaleKind::WholeTone);
        scale.validate_spelling().unwrap();
        let notes = scale.notes();
        assert_eq!(notes.len(), 6);
        
        // Db Whole Tone
        let scale = Scale::new(DFlat, ScaleKind::WholeTone);
        scale.validate_spelling().unwrap();
    }

    #[test]
    fn test_enharmonic_spelling_diminished() {
        // Diminished scales are octatonic (8 notes), so we allow letter repeats
        
        // C Diminished Whole-Half
        let scale = Scale::new(C, ScaleKind::DiminishedWholeHalf);
        scale.validate_spelling().unwrap(); // Should pass even with repeats
        let notes = scale.notes();
        assert_eq!(notes.len(), 8);
        
        // C Diminished Half-Whole
        let scale = Scale::new(C, ScaleKind::DiminishedHalfWhole);
        scale.validate_spelling().unwrap(); // Should pass even with repeats
        let notes = scale.notes();
        assert_eq!(notes.len(), 8);
    }

    #[test]
    fn test_enharmonic_spelling_chromatic() {
        // Chromatic scale (12 notes), we allow letter repeats
        
        // C Chromatic
        let scale = Scale::new(C, ScaleKind::Chromatic);
        scale.validate_spelling().unwrap(); // Should pass even with repeats
        let notes = scale.notes();
        assert_eq!(notes.len(), 12);
    }

    #[test]
    fn test_enharmonic_spelling_all_roots() {
        // Test all scales with multiple root notes to ensure consistency
        for root in [C, CSharp, D, DFlat, E, F, FSharp, G, GFlat, A, AFlat, B, BFlat] {
            // Major
            let scale = Scale::new(root, ScaleKind::Major);
            scale.validate_spelling().unwrap_or_else(|e| panic!("Major spelling failed for {}: {}", root.static_name(), e));
            
            // Natural Minor
            let scale = Scale::new(root, ScaleKind::NaturalMinor);
            scale.validate_spelling().unwrap_or_else(|e| panic!("Natural Minor spelling failed for {}: {}", root.static_name(), e));
            
            // Harmonic Minor
            let scale = Scale::new(root, ScaleKind::HarmonicMinor);
            scale.validate_spelling().unwrap_or_else(|e| panic!("Harmonic Minor spelling failed for {}: {}", root.static_name(), e));
            
            // Major Pentatonic
            let scale = Scale::new(root, ScaleKind::MajorPentatonic);
            scale.validate_spelling().unwrap_or_else(|e| panic!("Major Pentatonic spelling failed for {}: {}", root.static_name(), e));
            
            // Blues
            let scale = Scale::new(root, ScaleKind::Blues);
            scale.validate_spelling().unwrap_or_else(|e| panic!("Blues spelling failed for {}: {}", root.static_name(), e));
        }
    }

    #[test]
    fn test_heptatonic_spelling() {
        let scale = Scale::new(DFlat, ScaleKind::Major);
        assert_eq!(
            scale.notes(),
            vec![DFlat, EFlat, F, GFlat, AFlat, BFlat, CFive],
            "Db major scale spelling incorrect"
        );
        scale.validate_spelling().unwrap();
        
        let scale = Scale::new(CSharp, ScaleKind::Major);
        assert_eq!(
            scale.notes(),
            vec![CSharp, DSharp, ESharp, FSharp, GSharp, ASharp, BSharp],
            "C# major scale spelling incorrect - should use sharps consistently"
        );
        scale.validate_spelling().unwrap();
    }
    
    #[test]
    fn test_whole_tone_spelling() {
        let scale = Scale::new(A, ScaleKind::WholeTone);
        assert_eq!(
            scale.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::AugmentedFifth,
                Interval::AugmentedSixth,
            ],
            "Whole tone scale intervals should use augmented intervals, not respelled for prettiness"
        );
        
        let notes = scale.notes();
        assert_eq!(notes.len(), 6, "Whole tone scale should have 6 notes");
        scale.validate_spelling().unwrap();
        
        let scale = Scale::new(FSharp, ScaleKind::WholeTone);
        let notes = scale.notes();
        assert_eq!(notes.len(), 6, "F# whole tone scale should have 6 notes");
        scale.validate_spelling().unwrap();
    }
    
    #[test]
    fn test_octatonic_spelling() {
        let scale = Scale::new(A, ScaleKind::DiminishedHalfWhole);
        let notes = scale.notes();
        assert_eq!(
            scale.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ]
        );
        assert_eq!(notes.len(), 8, "Diminished half-whole should have 8 notes");
        
        let scale = Scale::new(A, ScaleKind::DiminishedWholeHalf);
        let notes = scale.notes();
        assert_eq!(
            scale.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::DiminishedSeventh,
                Interval::MajorSeventh,
            ]
        );
        assert_eq!(notes.len(), 8, "Diminished whole-half should have 8 notes");
    }
    
    #[test]
    fn test_pentatonic_blues_spelling() {
        let scale = Scale::new(DFlat, ScaleKind::MajorPentatonic);
        assert_eq!(
            scale.notes(),
            vec![DFlat, EFlat, F, AFlat, BFlat],
            "Db major pentatonic spelling incorrect"
        );
        scale.validate_spelling().unwrap();
        
        let scale = Scale::new(A, ScaleKind::MinorPentatonic);
        assert_eq!(
            scale.notes(),
            vec![A, CFive, DFive, EFive, GFive],
            "A minor pentatonic spelling incorrect"
        );
        scale.validate_spelling().unwrap();
        
        let scale = Scale::new(FSharp, ScaleKind::Blues);
        let notes = scale.notes();
        assert_eq!(
            notes,
            vec![FSharp, A, B, BSharp, CSharpFive, EFive],
            "F# blues scale spelling incorrect - should use B# (augmented 4th), not C (diminished 5th)"
        );
        assert_eq!(
            scale.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MinorSeventh,
            ]
        );
        scale.validate_spelling().unwrap();
    }
}
