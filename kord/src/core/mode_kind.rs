//! A module for working with mode kinds.

use crate::core::{
    base::{HasDescription, HasName, HasStaticName},
    interval::{HasIntervals, Interval},
    scale_kind::ScaleKind,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Enum.

/// An enum representing a mode kind (type of mode).
///
/// Each mode kind has an **explicit** list of intervals that define the mode.
/// These intervals are NOT derived by rotation - they are the authoritative definition.
/// Parent scale information is included for documentation purposes only.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ModeKind {
    // Major scale modes (diatonic)
    /// Ionian mode (1st mode of major scale).
    Ionian,
    /// Dorian mode (2nd mode of major scale).
    Dorian,
    /// Phrygian mode (3rd mode of major scale).
    Phrygian,
    /// Lydian mode (4th mode of major scale).
    Lydian,
    /// Mixolydian mode (5th mode of major scale).
    Mixolydian,
    /// Aeolian mode (6th mode of major scale).
    Aeolian,
    /// Locrian mode (7th mode of major scale).
    Locrian,

    // Harmonic minor modes
    /// Locrian ♮6 mode (2nd mode of harmonic minor).
    LocrianNatural6,
    /// Ionian ♯5 mode (3rd mode of harmonic minor).
    IonianSharp5,
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
    /// Altered / Super Locrian mode (7th mode of melodic minor).
    Altered,
}

// Impls.

impl ModeKind {
    /// Returns the parent scale kind for this mode (for documentation only).
    ///
    /// This is metadata - the intervals are NOT derived from the parent.
    pub fn parent_scale(&self) -> ScaleKind {
        match self {
            ModeKind::Ionian
            | ModeKind::Dorian
            | ModeKind::Phrygian
            | ModeKind::Lydian
            | ModeKind::Mixolydian
            | ModeKind::Aeolian
            | ModeKind::Locrian => ScaleKind::Major,
            
            ModeKind::LocrianNatural6
            | ModeKind::IonianSharp5
            | ModeKind::DorianSharp4
            | ModeKind::PhrygianDominant
            | ModeKind::LydianSharp2
            | ModeKind::Ultralocrian => ScaleKind::HarmonicMinor,
            
            ModeKind::DorianFlat2
            | ModeKind::LydianAugmented
            | ModeKind::LydianDominant
            | ModeKind::MixolydianFlat6
            | ModeKind::LocrianNatural2
            | ModeKind::Altered => ScaleKind::MelodicMinor,
        }
    }

    /// Returns the degree of the parent scale that this mode starts on (for documentation only).
    ///
    /// This is metadata - the intervals are NOT derived from the parent.
    pub fn parent_degree(&self) -> u8 {
        match self {
            // Major scale modes
            ModeKind::Ionian => 1,
            ModeKind::Dorian => 2,
            ModeKind::Phrygian => 3,
            ModeKind::Lydian => 4,
            ModeKind::Mixolydian => 5,
            ModeKind::Aeolian => 6,
            ModeKind::Locrian => 7,
            
            // Harmonic minor modes
            ModeKind::LocrianNatural6 => 2,
            ModeKind::IonianSharp5 => 3,
            ModeKind::DorianSharp4 => 4,
            ModeKind::PhrygianDominant => 5,
            ModeKind::LydianSharp2 => 6,
            ModeKind::Ultralocrian => 7,
            
            // Melodic minor modes
            ModeKind::DorianFlat2 => 2,
            ModeKind::LydianAugmented => 3,
            ModeKind::LydianDominant => 4,
            ModeKind::MixolydianFlat6 => 5,
            ModeKind::LocrianNatural2 => 6,
            ModeKind::Altered => 7,
        }
    }
}

impl HasIntervals for ModeKind {
    fn intervals(&self) -> &'static [Interval] {
        match self {
            // MAJOR SCALE MODES
            
            // Ionian: W-W-H-W-W-W-H (same as major scale)
            ModeKind::Ionian => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            // Dorian: W-H-W-W-W-H-W (minor scale with raised 6th)
            ModeKind::Dorian => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            // Phrygian: H-W-W-W-H-W-W (minor scale with lowered 2nd)
            ModeKind::Phrygian => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            // Lydian: W-W-W-H-W-W-H (major scale with raised 4th)
            ModeKind::Lydian => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            // Mixolydian: W-W-H-W-W-H-W (major scale with lowered 7th)
            ModeKind::Mixolydian => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            // Aeolian: W-H-W-W-H-W-W (natural minor scale)
            ModeKind::Aeolian => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            // Locrian: H-W-W-H-W-W-W (diminished scale)
            ModeKind::Locrian => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            
            // HARMONIC MINOR MODES
            
            // Locrian ♮6: H-W-W-H-W+H-W (Locrian with natural 6th)
            // Example: B Locrian ♮6 = B C D E F G♯ A
            ModeKind::LocrianNatural6 => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            // Ionian ♯5: W-W-H-W+H-H-H (Major with augmented 5th)
            // Example: C Ionian ♯5 = C D E F G♯ A B
            ModeKind::IonianSharp5 => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::AugmentedFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            // Dorian ♯4: W-H-W+H-H-W-W (Dorian with augmented 4th)
            // Example: D Dorian ♯4 = D E F G♯ A B C
            ModeKind::DorianSharp4 => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            // Phrygian Dominant: H-W+H-H-W-H-W-W (Phrygian with major 3rd)
            // Example: E Phrygian Dominant = E F G♯ A B C D
            ModeKind::PhrygianDominant => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            // Lydian ♯2: W+H-H-W-H-W-W-H (Lydian with augmented 2nd)
            // Example: F Lydian ♯2 = F G♯ A B C D E
            ModeKind::LydianSharp2 => &[
                Interval::PerfectUnison,
                Interval::AugmentedSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            // Ultralocrian: H-W-H-W-W-W-W (Locrian ♭♭7)
            // Example: G♯ Ultralocrian = G♯ A B C D E F
            ModeKind::Ultralocrian => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::DiminishedFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::DiminishedSeventh,
            ],
            
            // MELODIC MINOR MODES
            
            // Dorian ♭2 (Phrygian ♮6): H-W-W-W-W-W-H (Dorian with flat 2nd)
            // Example: B Dorian ♭2 = B C D E F♯ G♯ A
            ModeKind::DorianFlat2 => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            // Lydian Augmented: W-W-W-W-H-W-H (Lydian with augmented 5th)
            // Example: C Lydian Augmented = C D E F♯ G♯ A B
            ModeKind::LydianAugmented => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::AugmentedFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            // Lydian Dominant (Acoustic): W-W-W-H-W-H-W (Mixolydian with sharp 4th)
            // Example: D Lydian Dominant = D E F♯ G♯ A B C
            ModeKind::LydianDominant => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            // Mixolydian ♭6 (Aeolian Dominant): W-W-H-W-H-W-W (Mixolydian with flat 6th)
            // Example: E Mixolydian ♭6 = E F♯ G♯ A B C D
            ModeKind::MixolydianFlat6 => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            // Locrian ♮2 (Half-diminished): W-H-W-H-W-W-W (Locrian with natural 2nd)
            // Example: F♯ Locrian ♮2 = F♯ G♯ A B C D E
            ModeKind::LocrianNatural2 => &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            // Altered (Super Locrian): H-W-H-W-W-W-W (All alterations)
            // Example: G♯ Altered = G♯ A B C D E F♯
            //
            // Theoretical interval spelling from the melodic minor parent would be:
            //   1, m2, A2, M3, dim5, dim6, m7
            //
            // However, we intentionally use enharmonic equivalents here:
            //   A2  → m3  (AugmentedSecond  → MinorThird)
            //   M3  → dim4 (MajorThird      → DiminishedFourth)
            //   dim6 → m6  (DiminishedSixth → MinorSixth)
            //
            // This keeps the internal interval representation free of double accidentals
            // and avoids reusing the same letter name multiple times when combined with
            // validate_spelling(), which checks for unique letter names. In other words,
            // these spellings are chosen for practical spelling/validation reasons while
            // remaining pitch‑equivalent to the theoretical Altered mode.
            ModeKind::Altered => &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::DiminishedFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
        }
    }
}

impl HasDescription for ModeKind {
    fn description(&self) -> &'static str {
        match self {
            // Major scale modes
            ModeKind::Ionian => "ionian, 1st mode of major scale, major scale",
            ModeKind::Dorian => "dorian, 2nd mode of major scale, minor with raised 6th",
            ModeKind::Phrygian => "phrygian, 3rd mode of major scale, minor with lowered 2nd",
            ModeKind::Lydian => "lydian, 4th mode of major scale, major with raised 4th",
            ModeKind::Mixolydian => "mixolydian, 5th mode of major scale, major with lowered 7th",
            ModeKind::Aeolian => "aeolian, 6th mode of major scale, natural minor",
            ModeKind::Locrian => "locrian, 7th mode of major scale, diminished, half-diminished chord scale",
            
            // Harmonic minor modes
            ModeKind::LocrianNatural6 => "locrian ♮6, 2nd mode of harmonic minor, m7♭5(♮13) color",
            ModeKind::IonianSharp5 => "ionian ♯5, 3rd mode of harmonic minor, augmented major",
            ModeKind::DorianSharp4 => "dorian ♯4, 4th mode of harmonic minor, minor with lydian bite",
            ModeKind::PhrygianDominant => "phrygian dominant, 5th mode of harmonic minor, spanish phrygian",
            ModeKind::LydianSharp2 => "lydian ♯2, 6th mode of harmonic minor, bright + exotic",
            ModeKind::Ultralocrian => "ultralocrian, 7th mode of harmonic minor, very unstable/dark",
            
            // Melodic minor modes
            ModeKind::DorianFlat2 => "dorian ♭2, 2nd mode of melodic minor, phrygian ♮6, minor with spicy ♭2",
            ModeKind::LydianAugmented => "lydian augmented, 3rd mode of melodic minor, lydian ♯5",
            ModeKind::LydianDominant => "lydian dominant, 4th mode of melodic minor, acoustic scale, dominant with ♯11",
            ModeKind::MixolydianFlat6 => "mixolydian ♭6, 5th mode of melodic minor, aeolian dominant, dominant with ♭13",
            ModeKind::LocrianNatural2 => "locrian ♮2, 6th mode of melodic minor, half-diminished ♮2",
            ModeKind::Altered => "altered, 7th mode of melodic minor, super locrian, V7alt scale",
        }
    }
}

impl HasStaticName for ModeKind {
    fn static_name(&self) -> &'static str {
        match self {
            // Major scale modes
            ModeKind::Ionian => "ionian",
            ModeKind::Dorian => "dorian",
            ModeKind::Phrygian => "phrygian",
            ModeKind::Lydian => "lydian",
            ModeKind::Mixolydian => "mixolydian",
            ModeKind::Aeolian => "aeolian",
            ModeKind::Locrian => "locrian",
            
            // Harmonic minor modes
            ModeKind::LocrianNatural6 => "locrian ♮6",
            ModeKind::IonianSharp5 => "ionian ♯5",
            ModeKind::DorianSharp4 => "dorian ♯4",
            ModeKind::PhrygianDominant => "phrygian dominant",
            ModeKind::LydianSharp2 => "lydian ♯2",
            ModeKind::Ultralocrian => "ultralocrian",
            
            // Melodic minor modes
            ModeKind::DorianFlat2 => "dorian ♭2",
            ModeKind::LydianAugmented => "lydian augmented",
            ModeKind::LydianDominant => "lydian dominant",
            ModeKind::MixolydianFlat6 => "mixolydian ♭6",
            ModeKind::LocrianNatural2 => "locrian ♮2",
            ModeKind::Altered => "altered",
        }
    }
}

impl HasName for ModeKind {
    fn name(&self) -> String {
        self.static_name().to_owned()
    }
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_mode_intervals_explicit() {
        // All modes should have 7 notes
        assert_eq!(ModeKind::Ionian.intervals().len(), 7);
        assert_eq!(ModeKind::Dorian.intervals().len(), 7);
        assert_eq!(ModeKind::Phrygian.intervals().len(), 7);
        assert_eq!(ModeKind::Lydian.intervals().len(), 7);
        assert_eq!(ModeKind::Mixolydian.intervals().len(), 7);
        assert_eq!(ModeKind::Aeolian.intervals().len(), 7);
        assert_eq!(ModeKind::Locrian.intervals().len(), 7);

        // Ionian = Major
        assert_eq!(
            ModeKind::Ionian.intervals(),
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

        // Dorian has characteristic major 6th in minor context
        assert_eq!(
            ModeKind::Dorian.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,      // Characteristic raised 6th
                Interval::MinorSeventh,
            ]
        );

        // Phrygian has characteristic minor 2nd
        assert_eq!(
            ModeKind::Phrygian.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MinorSecond,     // Characteristic lowered 2nd
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ]
        );

        // Lydian has characteristic augmented 4th
        assert_eq!(
            ModeKind::Lydian.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,  // Characteristic raised 4th
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ]
        );

        // Locrian has characteristic diminished 5th
        assert_eq!(
            ModeKind::Locrian.intervals(),
            &[
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth, // Characteristic diminished 5th
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ]
        );
    }

    #[test]
    fn test_mode_names() {
        assert_eq!(ModeKind::Ionian.static_name(), "ionian");
        assert_eq!(ModeKind::Dorian.static_name(), "dorian");
        assert_eq!(ModeKind::Phrygian.static_name(), "phrygian");
        assert_eq!(ModeKind::Lydian.static_name(), "lydian");
        assert_eq!(ModeKind::Mixolydian.static_name(), "mixolydian");
        assert_eq!(ModeKind::Aeolian.static_name(), "aeolian");
        assert_eq!(ModeKind::Locrian.static_name(), "locrian");
    }

    #[test]
    fn test_mode_parent_metadata() {
        // All major scale modes have Major as parent
        assert_eq!(ModeKind::Ionian.parent_scale(), ScaleKind::Major);
        assert_eq!(ModeKind::Dorian.parent_scale(), ScaleKind::Major);
        assert_eq!(ModeKind::Locrian.parent_scale(), ScaleKind::Major);

        // Degrees should be 1-7
        assert_eq!(ModeKind::Ionian.parent_degree(), 1);
        assert_eq!(ModeKind::Dorian.parent_degree(), 2);
        assert_eq!(ModeKind::Phrygian.parent_degree(), 3);
        assert_eq!(ModeKind::Lydian.parent_degree(), 4);
        assert_eq!(ModeKind::Mixolydian.parent_degree(), 5);
        assert_eq!(ModeKind::Aeolian.parent_degree(), 6);
        assert_eq!(ModeKind::Locrian.parent_degree(), 7);
    }

    #[test]
    fn test_mode_descriptions() {
        assert_eq!(ModeKind::Ionian.description(), "ionian, 1st mode of major scale, major scale");
        assert_eq!(ModeKind::Dorian.description(), "dorian, 2nd mode of major scale, minor with raised 6th");
        assert_eq!(ModeKind::Lydian.description(), "lydian, 4th mode of major scale, major with raised 4th");
    }
}
