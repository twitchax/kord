//! A module for working with known chords.

use crate::core::{
    base::{HasDescription, HasName, HasStaticName},
    interval::Interval,
    mode::Mode,
    mode_kind::ModeKind,
    modifier::Degree,
    note::Note,
    scale::Scale,
    scale_kind::ScaleKind,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Traits.

/// A trait for types that have a relative scale.
pub trait HasRelativeScale {
    /// Returns the relative scale of the type (usually a [`Chord`]).
    ///
    /// The relative scale is the scale that the chord is built on, using
    /// only the intervals, without any need for notes; e.g., a major chord
    /// is built with all the "major" and "perfect" intervals.
    fn relative_scale(&self) -> Vec<Interval>;
}

/// A trait for types that have a relative chord.
pub trait HasRelativeChord {
    /// Returns the relative chord of the type (usually a [`Chord`]).
    ///
    /// The relative chord is the chord that the chord is built on, using
    /// only the intervals, without any need for notes; e.g., a major chord
    /// is built with the major third and perfect fifth intervals.
    fn relative_chord(&self) -> Vec<Interval>;
}

/// A trait for types that can enumerate recommended scales and modes.
pub trait HasScaleCandidates {
    /// Returns a list of recommended scale/mode candidates for this chord,
    /// ranked by relevance.
    fn scale_candidates(&self) -> Vec<ScaleCandidate>;
}

// Structures.

/// Represents a recommended scale or mode for a chord.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScaleCandidate {
    /// A mode candidate
    Mode {
        /// The mode kind
        kind: ModeKind,
        /// Ranking (1 = most relevant)
        rank: u8,
        /// Reason why this mode fits the chord
        reason: &'static str,
    },
    /// A scale candidate
    Scale {
        /// The scale kind
        kind: ScaleKind,
        /// Ranking (1 = most relevant)
        rank: u8,
        /// Reason why this scale fits the chord
        reason: &'static str,
    },
}

impl ScaleCandidate {
    /// Returns the rank of this candidate
    pub fn rank(&self) -> u8 {
        match self {
            ScaleCandidate::Mode { rank, .. } => *rank,
            ScaleCandidate::Scale { rank, .. } => *rank,
        }
    }

    /// Returns the reason for this candidate
    pub fn reason(&self) -> &'static str {
        match self {
            ScaleCandidate::Mode { reason, .. } => reason,
            ScaleCandidate::Scale { reason, .. } => reason,
        }
    }

    /// Returns the name of this candidate
    pub fn name(&self) -> String {
        match self {
            ScaleCandidate::Mode { kind, .. } => kind.name(),
            ScaleCandidate::Scale { kind, .. } => kind.name(),
        }
    }

    /// Returns the description of this candidate
    pub fn description(&self) -> &'static str {
        match self {
            ScaleCandidate::Mode { kind, .. } => kind.description(),
            ScaleCandidate::Scale { kind, .. } => kind.description(),
        }
    }

    /// Returns the notes of this candidate rooted at the given note
    pub fn notes(&self, root: Note) -> Vec<Note> {
        match self {
            ScaleCandidate::Mode { kind, .. } => Mode::new(root, *kind).notes(),
            ScaleCandidate::Scale { kind, .. } => Scale::new(root, *kind).notes(),
        }
    }
}

/// Represents the kind of interval collection (mode or scale)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IntervalCollectionKind {
    /// A mode
    Mode(ModeKind),
    /// A scale
    Scale(ScaleKind),
}

/// Represents a scale or mode candidate in static storage format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntervalCandidate {
    /// The kind of interval collection
    pub kind: IntervalCollectionKind,
    /// Ranking (1 = most relevant)
    pub rank: u8,
    /// Reason why this fits the chord
    pub reason: &'static str,
}

impl IntervalCandidate {
    /// Converts this interval candidate to a rooted scale candidate
    pub fn to_scale_candidate(&self) -> ScaleCandidate {
        match self.kind {
            IntervalCollectionKind::Mode(kind) => ScaleCandidate::Mode {
                kind,
                rank: self.rank,
                reason: self.reason,
            },
            IntervalCollectionKind::Scale(kind) => ScaleCandidate::Scale {
                kind,
                rank: self.rank,
                reason: self.reason,
            },
        }
    }
}

// Enum.

/// An enum representing a known chord.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum KnownChord {
    /// An unknown chord.
    Unknown,
    /// A major chord.
    Major,
    /// A minor chord.
    Minor,
    /// A major 7 chord.
    Major7,
    /// A dominant chord.
    Dominant(Degree),
    /// A minor major 7 chord.
    MinorMajor7,
    /// A minor dominant chord with degree.
    MinorDominant(Degree),
    /// A dominant sharp 11 chord with degree.
    DominantSharp11(Degree),
    /// An augmented chord.
    Augmented,
    /// An augmented major 7 chord.
    AugmentedMajor7,
    /// An augmented chord.
    AugmentedDominant(Degree),
    /// A half diminished chord.
    HalfDiminished(Degree),
    /// A diminished chord.
    Diminished,
    /// A dominant flat 9 chord.
    DominantFlat9(Degree),
    /// A dominant sharp 9 chord.
    DominantSharp9(Degree),
    /// A dominant flat 9 sharp 5 chord.
    DominantFlat9Sharp5(Degree),
    /// A minor dominant flat 13 chord.
    MinorDominantFlat13(Degree),
    /// A minor dominant flat 9 flat 13 chord.
    MinorDominantFlat9Flat13(Degree),
    /// A sharp 11 chord.
    Sharp11,
}

// Impls.

impl HasDescription for KnownChord {
    fn description(&self) -> &'static str {
        match self {
            KnownChord::Unknown => panic!("KnownChord::Unknown should never be used in description()"),
            KnownChord::Major => "major",
            KnownChord::Minor => "minor",
            KnownChord::Major7 => "major 7",
            KnownChord::Dominant(_) => "dominant",
            KnownChord::MinorMajor7 => "minor major 7",
            KnownChord::MinorDominant(_) => "minor 7",
            KnownChord::DominantSharp11(_) => "dominant sharp 11",
            KnownChord::Augmented => "augmented",
            KnownChord::AugmentedMajor7 => "augmented major 7",
            KnownChord::AugmentedDominant(_) => "augmented dominant",
            KnownChord::HalfDiminished(_) => "half diminished",
            KnownChord::Diminished => "diminished",
            KnownChord::DominantFlat9(_) => "dominant flat 9",
            KnownChord::DominantSharp9(_) => "dominant sharp 9",
            KnownChord::DominantFlat9Sharp5(_) => "dominant flat 9 sharp 5",
            KnownChord::MinorDominantFlat13(_) => "minor dominant flat 13",
            KnownChord::MinorDominantFlat9Flat13(_) => "minor dominant flat 9 flat 13",
            KnownChord::Sharp11 => "sharp 11",
        }
    }
}

impl HasRelativeScale for KnownChord {
    fn relative_scale(&self) -> Vec<Interval> {
        match self {
            KnownChord::Unknown => panic!("KnownChord::Unknown should never be used in relative_scale()"),
            KnownChord::Major => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            KnownChord::Minor => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            KnownChord::Major7 => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            KnownChord::Dominant(_) => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            KnownChord::MinorMajor7 => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            KnownChord::MinorDominant(_) => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            KnownChord::DominantSharp11(_) => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            KnownChord::Augmented => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::AugmentedFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            KnownChord::AugmentedMajor7 => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::AugmentedFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            KnownChord::AugmentedDominant(_) => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::AugmentedFifth,
                Interval::AugmentedSixth,
            ],
            KnownChord::HalfDiminished(_) => vec![
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            KnownChord::Diminished => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::DiminishedSeventh,
                Interval::MajorSeventh,
            ],
            KnownChord::DominantFlat9(_) => vec![
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MinorSeventh,
            ],
            KnownChord::DominantSharp9(_) => vec![
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::DiminishedFourth,
                Interval::DiminishedFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            KnownChord::DominantFlat9Sharp5(_) => vec![
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::AugmentedFifth,
                Interval::MinorSeventh,
            ],
            KnownChord::MinorDominantFlat13(_) => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            KnownChord::MinorDominantFlat9Flat13(_) => vec![
                Interval::PerfectUnison,
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            KnownChord::Sharp11 => vec![
                Interval::PerfectUnison,
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::AugmentedFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
        }
    }
}

impl HasRelativeChord for KnownChord {
    fn relative_chord(&self) -> Vec<Interval> {
        match self {
            KnownChord::Unknown => panic!("KnownChord::Unknown should never be used in relative_chord()"),
            KnownChord::Major => vec![Interval::PerfectUnison, Interval::MajorThird, Interval::PerfectFifth],
            KnownChord::Minor => vec![Interval::PerfectUnison, Interval::MinorThird, Interval::PerfectFifth],
            KnownChord::Major7 => vec![Interval::PerfectUnison, Interval::MajorThird, Interval::PerfectFifth, Interval::MajorSeventh],
            KnownChord::Dominant(_) => vec![Interval::PerfectUnison, Interval::MajorThird, Interval::PerfectFifth, Interval::MinorSeventh],
            KnownChord::MinorMajor7 => vec![Interval::PerfectUnison, Interval::MinorThird, Interval::PerfectFifth, Interval::MajorSeventh],
            KnownChord::MinorDominant(_) => vec![Interval::PerfectUnison, Interval::MinorThird, Interval::PerfectFifth, Interval::MinorSeventh],
            KnownChord::DominantSharp11(_) => vec![
                Interval::PerfectUnison,
                Interval::MajorThird,
                Interval::PerfectFifth,
                Interval::MinorSeventh,
                Interval::AugmentedEleventh,
            ],
            KnownChord::Augmented => vec![Interval::PerfectUnison, Interval::MajorThird, Interval::AugmentedFifth],
            KnownChord::AugmentedMajor7 => vec![Interval::PerfectUnison, Interval::MajorThird, Interval::AugmentedFifth, Interval::MajorSeventh],
            KnownChord::AugmentedDominant(_) => vec![Interval::PerfectUnison, Interval::MajorThird, Interval::AugmentedFifth, Interval::MinorSeventh],
            KnownChord::HalfDiminished(_) => vec![Interval::PerfectUnison, Interval::MinorThird, Interval::DiminishedFifth, Interval::MinorSeventh],
            KnownChord::Diminished => vec![Interval::PerfectUnison, Interval::MinorThird, Interval::DiminishedFifth, Interval::DiminishedSeventh],
            KnownChord::DominantFlat9(_) => vec![Interval::PerfectUnison, Interval::MajorThird, Interval::PerfectFifth, Interval::MinorSeventh, Interval::MinorNinth],
            KnownChord::DominantSharp9(_) => vec![Interval::PerfectUnison, Interval::MajorThird, Interval::PerfectFifth, Interval::MinorSeventh, Interval::AugmentedNinth],
            KnownChord::DominantFlat9Sharp5(_) => vec![Interval::PerfectUnison, Interval::MajorThird, Interval::AugmentedFifth, Interval::MinorSeventh, Interval::MinorNinth],
            KnownChord::MinorDominantFlat13(_) => vec![Interval::PerfectUnison, Interval::MinorThird, Interval::PerfectFifth, Interval::MinorSeventh, Interval::MinorThirteenth],
            KnownChord::MinorDominantFlat9Flat13(_) => vec![
                Interval::PerfectUnison,
                Interval::MinorThird,
                Interval::PerfectFifth,
                Interval::MinorSeventh,
                Interval::MinorNinth,
                Interval::MinorThirteenth,
            ],
            KnownChord::Sharp11 => vec![
                Interval::PerfectUnison,
                Interval::MajorThird,
                Interval::PerfectFifth,
                Interval::MajorSeventh,
                Interval::AugmentedEleventh,
            ],
        }
    }
}

impl HasName for KnownChord {
    fn name(&self) -> String {
        match self {
            KnownChord::Unknown => panic!("KnownChord::Unknown should never be used in name()"),
            KnownChord::Major => "".to_owned(),
            KnownChord::Minor => "m".to_owned(),
            KnownChord::Major7 => "maj7".to_owned(),
            KnownChord::Dominant(d) => d.static_name().to_owned(),
            KnownChord::MinorMajor7 => "m(maj7)".to_owned(),
            KnownChord::MinorDominant(d) => format!("m{}", d.static_name()),
            KnownChord::DominantSharp11(d) => format!("{}(♯11)", d.static_name()),
            KnownChord::Augmented => "+".to_owned(),
            KnownChord::AugmentedMajor7 => "+(maj7)".to_owned(),
            KnownChord::AugmentedDominant(d) => format!("+{}", d.static_name()),
            KnownChord::HalfDiminished(d) => format!("m{}(♭5)", d.static_name()),
            KnownChord::Diminished => "dim".to_owned(),
            KnownChord::DominantFlat9(d) => format!("{}(♭9)", d.static_name()),
            KnownChord::DominantSharp9(d) => format!("{}(♯9)", d.static_name()),
            KnownChord::DominantFlat9Sharp5(d) => format!("{}(♭9)(♯5)", d.static_name()),
            KnownChord::MinorDominantFlat13(d) => format!("m{}(♭13)", d.static_name()),
            KnownChord::MinorDominantFlat9Flat13(d) => format!("{}(♭9)(♭13)", d.static_name()),
            KnownChord::Sharp11 => "(♯11)".to_owned(),
        }
    }
}

impl KnownChord {
    /// Returns the static interval candidates for this chord
    pub fn scale_interval_candidates(&self) -> &'static [IntervalCandidate] {
        match self {
            KnownChord::Unknown => &[],
            KnownChord::Major => MAJOR_CANDIDATES,
            KnownChord::Minor => MINOR_CANDIDATES,
            KnownChord::Major7 => MAJOR7_CANDIDATES,
            KnownChord::Dominant(_) => DOMINANT_CANDIDATES,
            KnownChord::MinorMajor7 => MINOR_MAJOR7_CANDIDATES,
            KnownChord::MinorDominant(_) => MINOR_DOMINANT_CANDIDATES,
            KnownChord::DominantSharp11(_) => DOMINANT_SHARP11_CANDIDATES,
            KnownChord::Augmented => AUGMENTED_CANDIDATES,
            KnownChord::AugmentedMajor7 => AUGMENTED_MAJOR7_CANDIDATES,
            KnownChord::AugmentedDominant(_) => AUGMENTED_DOMINANT_CANDIDATES,
            KnownChord::HalfDiminished(_) => HALF_DIMINISHED_CANDIDATES,
            KnownChord::Diminished => DIMINISHED_CANDIDATES,
            KnownChord::DominantFlat9(_) => DOMINANT_FLAT9_CANDIDATES,
            KnownChord::DominantSharp9(_) => DOMINANT_SHARP9_CANDIDATES,
            KnownChord::DominantFlat9Sharp5(_) => DOMINANT_FLAT9_SHARP5_CANDIDATES,
            KnownChord::MinorDominantFlat13(_) => MINOR_DOMINANT_FLAT13_CANDIDATES,
            KnownChord::MinorDominantFlat9Flat13(_) => MINOR_DOMINANT_FLAT9_FLAT13_CANDIDATES,
            KnownChord::Sharp11 => SHARP11_CANDIDATES,
        }
    }
}

impl HasScaleCandidates for KnownChord {
    fn scale_candidates(&self) -> Vec<ScaleCandidate> {
        self.scale_interval_candidates()
            .iter()
            .map(IntervalCandidate::to_scale_candidate)
            .collect()
    }
}

// Static interval candidates for each KnownChord variant

static MAJOR_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Ionian),
        rank: 1,
        reason: "Default diatonic major scale for functional harmony in tonal contexts; works over I, IV, V progressions; avoid dwelling on the 4th scale degree as it can clash with the major 3rd in the triad",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::MajorPentatonic),
        rank: 2,
        reason: "Safe major melody that avoids the 4th and 7th scale degrees completely; eliminates avoid-note concerns; perfect for pop, rock, and country hooks where you want guaranteed consonance",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Lydian),
        rank: 3,
        reason: "Bright major sound with ♯11 sheen; major with ♯4 (4th mode of major scale); common in modern jazz and film scoring; lean into the ♯4/♯11 as the characteristic color tone that distinguishes it from Ionian",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Mixolydian),
        rank: 4,
        reason: "Adds ♭7 over a major triad for blues, rock, and modal flavor; creates a dominant-like quality without needing to resolve; stylistic choice for a looser, groovier feel",
    },
];

static MINOR_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Aeolian),
        rank: 1,
        reason: "Default natural minor scale for functional harmony; 6th mode of major scale; provides the classic melancholic, sad tonality with ♭6; contains ♭3, ♭6, and ♭7 for full minor character",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::MinorPentatonic),
        rank: 2,
        reason: "Safe minor melody that avoids the ♭2 and ♭6 scale degrees; the go-to scale for blues, rock, and pentatonic-based improvisation; simple and effective with no avoid-note concerns",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::Blues),
        rank: 3,
        reason: "Essential blues vocabulary built on minor pentatonic with added ♯4 blue note; phrasing-driven rather than theoretical; creates that characteristic blues 'bend' and expressive quality",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Dorian),
        rank: 4,
        reason: "Minor mode with raised 6th (♮6) that provides a brighter, jazzier lift compared to Aeolian; 2nd mode of major scale; less sad than natural minor; common choice for jazz and funk minor sounds",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Phrygian),
        rank: 5,
        reason: "Minor mode with a lowered 2nd (♭2) that creates an exotic, Spanish, or Flamenco flavor; 3rd mode of major scale; dark and modal with a distinctly non-functional character",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::HarmonicMinor),
        rank: 6,
        reason: "Classical minor scale with raised 7th (♮7) for strong V-i resolution; the ♭6 creates tension and augmented-second interval; essential for traditional minor key harmony",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::MelodicMinor),
        rank: 7,
        reason: "Modern jazz minor with both ♮6 and ♮7 for smooth ascending melodic motion; eliminates the augmented second of harmonic minor; bright and sophisticated minor color",
    },
];

static MAJOR7_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Ionian),
        rank: 1,
        reason: "Default major 7 scale from functional harmony; Ionian mode provides the natural maj7 chord tones; watch out for the 4th scale degree which can clash as an avoid-note over the maj7 chord",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Lydian),
        rank: 2,
        reason: "Bright major 7 sound with ♯11 sheen; major 7 with ♯4 (4th mode of major scale); the modern jazz choice for maj7 chords; the ♯4/♯11 is the color tone that makes this sparkle; eliminates the avoid-note issue of the natural 4th",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::MajorPentatonic),
        rank: 3,
        reason: "Safe major 7 melody that completely avoids both the 4th and 7th scale degrees; provides consonant, hook-friendly melodic material with zero avoid-note concerns",
    },
];

static DOMINANT_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Mixolydian),
        rank: 1,
        reason: "Default dominant 7 scale; 5th mode of major scale; provides major 3rd with ♭7 for functional V chord resolution; the bread-and-butter choice for tonal dominant chords in ii-V-I progressions",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::Blues),
        rank: 2,
        reason: "Blues vocabulary over dominant 7 chords; includes the ♯4 blue note for characteristic blues phrasing; phrasing-driven rather than theoretical; perfect for blues turnarounds and rock contexts",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::LydianDominant),
        rank: 3,
        reason: "Dominant 7 with ♯11 color (Mixolydian with ♯4); 4th mode of melodic minor; sophisticated modern jazz sound; use when you want bright ♯11 upper-structure tension on your V chord; also called Acoustic scale",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::MixolydianFlat6),
        rank: 4,
        reason: "Dominant scale with ♭13 (♭6) for darker tension; 5th mode of melodic minor; creates a minor-leaning dominant sound; useful when the V chord needs to lean toward a minor resolution",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::WholeTone),
        rank: 5,
        reason: "Symmetrical whole tone scale for augmented dominant (#5) color; creates dreamy, floating, harmonically ambiguous sound; every note is a whole step apart; use for suspended, unresolved V7#5 or V+7 sounds",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::DiminishedHalfWhole),
        rank: 6,
        reason: "Classic dominant diminished (half-whole octatonic); supports ♭9, ♯9, ♯11, and 13 simultaneously; use when the V chord feels 'hot' and needs maximum upper-structure tension; distinct from Altered mode",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Altered),
        rank: 7,
        reason: "The ultimate altered dominant tool (7th mode of melodic minor); provides maximum tension with ♭9, ♯9, ♭5, and ♯5 (♭13) all available; strongest when resolving to minor i or in modern jazz contexts needing maximum harmonic pull",
    },
];

static MINOR_MAJOR7_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::MelodicMinor),
        rank: 1,
        reason: "Primary source scale for minor-major 7 chords; melodic minor provides the ♮6 and ♮7 needed for this chord quality; creates smooth ascending melodic lines in classical and jazz contexts",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::HarmonicMinor),
        rank: 2,
        reason: "Alternative source for mMaj7 with a more exotic flavor; harmonic minor provides the ♮7 but retains the ♭6 creating characteristic tension and the augmented second interval between ♭6 and ♮7",
    },
];

static MINOR_DOMINANT_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Dorian),
        rank: 1,
        reason: "Default minor 7 scale with raised 6th (♮6) that lifts and brightens the sound; 2nd mode of major scale; the classic jazz ii chord sound in ii-V-I progressions; more optimistic than Aeolian",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::MinorPentatonic),
        rank: 2,
        reason: "Safe minor 7 melody that avoids the ♭2 and ♭6 scale degrees for guaranteed consonance; simple and effective choice; the go-to for straightforward, bluesy minor 7 improvisation",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::Blues),
        rank: 3,
        reason: "Blues vocabulary over minor 7 chords with the characteristic ♯4 blue note; stylistic phrasing choice for blues, rock, and R&B contexts; phrasing-driven rather than purely harmonic",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Aeolian),
        rank: 4,
        reason: "Natural minor over m7 chords; includes the ♭6 for darker, more melancholic color compared to Dorian; functional and traditional; creates a more classically minor feel",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Phrygian),
        rank: 5,
        reason: "Modal minor 7 with lowered 2nd (♭2) for exotic, Spanish, or Flamenco flavor; 3rd mode of major scale; creates a distinctly modal, non-functional character; dark and mysterious",
    },
];

static DOMINANT_SHARP11_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::LydianDominant),
        rank: 1,
        reason: "The defining scale for V7♯11 chords; dominant 7 with ♯11 (Mixolydian with ♯4); 4th mode of melodic minor; sophisticated modern jazz sound for dominant chords with upper-structure tension; also called Acoustic scale",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Mixolydian),
        rank: 2,
        reason: "Basic dominant fallback that de-emphasizes the ♯11 extension; provides functional V7 sound (dominant 7 with ♭7); when you want simpler, more traditional dominant approach without the ♯11 color",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::DiminishedHalfWhole),
        rank: 3,
        reason: "Adds even more tension options on top of the V7♯11; half-whole diminished (dominant diminished) provides ♭9, ♯9, ♯11, and 13 simultaneously; use for hot, dense dominant sound with maximum upper-structure complexity",
    },
];

static AUGMENTED_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::IonianSharp5),
        rank: 1,
        reason: "Major scale with raised 5th (♯5); 3rd mode of harmonic minor; provides a functional augmented triad sound while maintaining major scale characteristics; the classic augmented triad source",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::WholeTone),
        rank: 2,
        reason: "Symmetrical whole tone scale where every augmented triad shares the same notes; creates a dreamy, floating, harmonically ambiguous quality; all notes are whole steps apart; perfect for suspended augmented sounds",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::LydianAugmented),
        rank: 3,
        reason: "Major scale with both ♯4 and ♯5; 3rd mode of melodic minor; provides a bright, exotic augmented color with Lydian characteristics; sophisticated choice for modern jazz augmented triads",
    },
];

static AUGMENTED_MAJOR7_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::LydianAugmented),
        rank: 1,
        reason: "3rd mode of melodic minor; provides major 7 with both ♯4/♯11 and ♯5; bright, exotic, and sophisticated augmented major 7 sound; the modern jazz choice for this chord quality",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::IonianSharp5),
        rank: 2,
        reason: "Major 7 with raised 5th from harmonic minor; more functional and traditional approach to augmented major 7; retains natural 4th unlike Lydian Augmented",
    },
];

static AUGMENTED_DOMINANT_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::WholeTone),
        rank: 1,
        reason: "Primary scale for augmented dominant 7 (V+7 or V7♯5) chords; symmetrical whole tone scale naturally provides the ♯5 augmented quality; creates dreamy, floating dominant color; perfect for unresolved, suspended V+7 sounds",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::LydianDominant),
        rank: 2,
        reason: "Dominant 7 with ♯11 (Mixolydian with ♯4) that can accommodate or bend toward ♯5; flexible modern dominant sound from melodic minor; use when you want V7 character with option to lean into augmented implications",
    },
];

static HALF_DIMINISHED_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Locrian),
        rank: 1,
        reason: "Default half-diminished (m7♭5) scale; 7th mode of major scale; provides the functional ii°7 sound in minor key ii°-V-i progressions; classic diminished 5th with minor 7th",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::LocrianNatural2),
        rank: 2,
        reason: "Half-diminished with raised 2nd (♮2) for smoother melodic motion; 6th mode of melodic minor; eliminates the difficult ♭2 interval; modern jazz choice for m7♭5 chords",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::LocrianNatural6),
        rank: 3,
        reason: "Half-diminished with raised 6th (♮6 or ♮13) for a brighter color; 2nd mode of harmonic minor; provides major 6th/13th extension while maintaining the ♭5; exotic flavor",
    },
];

static DIMINISHED_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::DiminishedWholeHalf),
        rank: 1,
        reason: "Symmetrical diminished 7 scale with whole-half step pattern; primary choice for fully diminished 7 chords; every diminished 7 chord shares three others a minor third apart in this scale",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::DiminishedHalfWhole),
        rank: 2,
        reason: "Alternative diminished scale with half-whole step pattern; more commonly used for dominant 7 chords but can work as a passing chord option for dim7; provides different color than W-H pattern",
    },
];

static DOMINANT_FLAT9_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::DiminishedHalfWhole),
        rank: 1,
        reason: "Primary scale for V7♭9 chords using symmetrical half-whole diminished (dominant diminished) pattern; provides ♭9, ♯9, ♯11, and 13 simultaneously; rich dominant tension palette for modern jazz and classical harmony",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::PhrygianDominant),
        rank: 2,
        reason: "Phrygian with major 3rd creates Spanish or Phrygian dominant sound; 5th mode of harmonic minor; characteristic ♭2/♭9 with major 3rd fingerprint; exotic, Middle Eastern, or Flamenco flavor; strong V in minor keys",
    },
];

static DOMINANT_SHARP9_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Altered),
        rank: 1,
        reason: "The altered dominant scale (7th mode of melodic minor); provides all alterations including ♭9, ♯9, ♭5, and ♯5 (♭13); maximum tension with strongest resolution pull; the ultimate V7♯9 choice for modern jazz and chromatic harmony",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::DiminishedHalfWhole),
        rank: 2,
        reason: "Symmetrical half-whole diminished (dominant diminished) providing ♭9, ♯9, ♯11, and 13; comprehensive dominant tension palette; hot V7 sound with dense upper-structure options; works when you need maximum harmonic complexity",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::Blues),
        rank: 3,
        reason: "Blues vocabulary that naturally includes the ♯9 sound through the ♯4 blue note; phrasing-driven stylistic choice rather than theoretical; perfect for blues and rock contexts where ♯9 is part of the blues language",
    },
];

static DOMINANT_FLAT9_SHARP5_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Altered),
        rank: 1,
        reason: "Primary scale for V7(♭9)(♯5) chords; the altered dominant scale (7th mode of melodic minor) provides all necessary alterations including ♭9, ♯5/♭13, and ♭5; maximum tension with both flat 9 and sharp 5 present; creates the strongest resolution pull in modern jazz and chromatic harmony",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::DiminishedHalfWhole),
        rank: 2,
        reason: "Symmetrical half-whole diminished (dominant diminished) that supports V7(♭9)(♯5); provides ♭9, ♯9, ♯11, and 13 simultaneously; rich dominant tension palette; works well when you need the ♭9 with option to imply or approach the ♯5",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::PhrygianDominant),
        rank: 3,
        reason: "Spanish or Phrygian dominant sound (5th mode of harmonic minor) emphasizing the characteristic ♭9; exotic, Middle Eastern, or Flamenco flavor; while it has natural 5th, it can approach or bend toward ♯5 in phrasing; strong V in minor keys",
    },
];

static MINOR_DOMINANT_FLAT13_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Aeolian),
        rank: 1,
        reason: "Natural minor scale over minor 7 chords; provides the ♭6/♭13 extension naturally; functional sound for minor dominant chords where you need the flat 13 characteristic clearly present",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Phrygian),
        rank: 2,
        reason: "Modal minor 7 with both ♭2 and ♭6/♭13; 3rd mode of major scale; creates an exotic, darker minor dominant with Spanish or modal flavor; both characteristic tensions present",
    },
];

static MINOR_DOMINANT_FLAT9_FLAT13_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Phrygian),
        rank: 1,
        reason: "Phrygian mode provides both characteristic tensions: ♭2/♭9 and ♭6/♭13; 3rd mode of major scale; creates exotic, modal, Spanish or Flamenco flavor; dark and mysterious minor dominant sound",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::Blues),
        rank: 2,
        reason: "Blues scale offers simpler, more phrasing-driven vocabulary; stylistic choice rather than literal interval match; use when you want blues vocabulary over this complex minor dominant chord symbol",
    },
];

static SHARP11_CANDIDATES: &[IntervalCandidate] = &[
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Lydian),
        rank: 1,
        reason: "Major 7 with ♯11 extension; major with ♯4 (4th mode of major scale); the bright, modern sound where ♯4/♯11 is the defining color tone; sophisticated jazz and film score choice; eliminates the avoid-note issue of natural 4th",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Mode(ModeKind::Ionian),
        rank: 2,
        reason: "Major 7 fallback that de-emphasizes the ♯11 extension; returns to functional harmony; use when you want to downplay the ♯11 color or need a simpler, more traditional major 7 approach",
    },
    IntervalCandidate {
        kind: IntervalCollectionKind::Scale(ScaleKind::MajorPentatonic),
        rank: 3,
        reason: "Safe major 7 melody that avoids both the 4th and 7th scale degrees; provides consonant melodic material with zero avoid-note concerns; perfect for hooks and melodic lines over maj7♯11 chords",
    },
];

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::modifier::Degree;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_dominant_chord_candidates() {
        let candidates = KnownChord::Dominant(Degree::Seven).scale_candidates();
        assert!(!candidates.is_empty(), "G7 should have scale candidates");
        
        match &candidates[0] {
            ScaleCandidate::Mode { kind, rank, .. } => {
                assert_eq!(*kind, ModeKind::Mixolydian);
                assert_eq!(*rank, 1);
            }
            _ => panic!("First candidate for G7 should be a Mode"),
        }
        
        match &candidates[1] {
            ScaleCandidate::Scale { kind, rank, .. } => {
                assert_eq!(*kind, ScaleKind::Blues);
                assert_eq!(*rank, 2);
            }
            _ => panic!("Second candidate for G7 should be a Scale"),
        }
        
        match &candidates[2] {
            ScaleCandidate::Mode { kind, rank, .. } => {
                assert_eq!(*kind, ModeKind::LydianDominant);
                assert_eq!(*rank, 3);
            }
            _ => panic!("Third candidate for G7 should be a Mode"),
        }
    }
    
    #[test]
    fn test_dominant_sharp11_candidates() {
        let candidates = KnownChord::DominantSharp11(Degree::Seven).scale_candidates();
        assert!(!candidates.is_empty(), "G7#11 should have scale candidates");
        
        match &candidates[0] {
            ScaleCandidate::Mode { kind, rank, .. } => {
                assert_eq!(*kind, ModeKind::LydianDominant);
                assert_eq!(*rank, 1);
            }
            _ => panic!("First candidate for G7#11 should be a Mode"),
        }
    }
    
    #[test]
    fn test_dominant_flat9_candidates() {
        let candidates = KnownChord::DominantFlat9(Degree::Seven).scale_candidates();
        assert!(!candidates.is_empty(), "G7b9 should have scale candidates");
        
        match &candidates[0] {
            ScaleCandidate::Scale { kind, rank, .. } => {
                assert_eq!(*kind, ScaleKind::DiminishedHalfWhole);
                assert_eq!(*rank, 1);
            }
            ScaleCandidate::Mode { kind, rank, .. } => {
                assert_eq!(*kind, ModeKind::PhrygianDominant);
                assert_eq!(*rank, 1);
            }
        }
    }
    
    #[test]
    fn test_dominant_sharp9_candidates() {
        let candidates = KnownChord::DominantSharp9(Degree::Seven).scale_candidates();
        assert!(!candidates.is_empty(), "G7#9 should have scale candidates");
        
        match &candidates[0] {
            ScaleCandidate::Mode { kind, rank, .. } => {
                assert_eq!(*kind, ModeKind::Altered);
                assert_eq!(*rank, 1);
            }
            _ => panic!("First candidate for G7#9 should be a Mode"),
        }
    }
    
    #[test]
    fn test_dominant_flat9_sharp5_candidates() {
        let candidates = KnownChord::DominantFlat9Sharp5(Degree::Seven).scale_candidates();
        assert!(!candidates.is_empty(), "G7(b9)(#5) should have scale candidates");
        
        match &candidates[0] {
            ScaleCandidate::Mode { kind, rank, .. } => {
                assert_eq!(*kind, ModeKind::Altered);
                assert_eq!(*rank, 1);
            }
            _ => panic!("First candidate for G7(b9)(#5) should be a Mode"),
        }
        
        match &candidates[1] {
            ScaleCandidate::Scale { kind, rank, .. } => {
                assert_eq!(*kind, ScaleKind::DiminishedHalfWhole);
                assert_eq!(*rank, 2);
            }
            _ => panic!("Second candidate for G7(b9)(#5) should be a Scale"),
        }
    }
    
    #[test]
    fn test_half_diminished_candidates() {
        let candidates = KnownChord::HalfDiminished(Degree::Seven).scale_candidates();
        assert!(!candidates.is_empty(), "Cm7b5 should have scale candidates");
        
        match &candidates[0] {
            ScaleCandidate::Mode { kind, rank, .. } => {
                assert_eq!(*kind, ModeKind::Locrian);
                assert_eq!(*rank, 1);
            }
            _ => panic!("First candidate for Cm7b5 should be a Mode"),
        }
    }
    
    #[test]
    fn test_augmented_dominant_candidates() {
        let candidates = KnownChord::AugmentedDominant(Degree::Seven).scale_candidates();
        assert!(!candidates.is_empty(), "Augmented dominant should have scale candidates");
        
        match &candidates[0] {
            ScaleCandidate::Scale { kind, rank, .. } => {
                assert_eq!(*kind, ScaleKind::WholeTone);
                assert_eq!(*rank, 1);
            }
            _ => panic!("First candidate for augmented dominant should be a Scale"),
        }
    }
    
    #[test]
    fn test_major_chord_candidates() {
        let candidates = KnownChord::Major.scale_candidates();
        assert!(candidates.len() >= 3, "Major chord should have at least 3 candidates");
        
        match &candidates[0] {
            ScaleCandidate::Mode { kind, rank, .. } => {
                assert_eq!(*kind, ModeKind::Ionian);
                assert_eq!(*rank, 1);
            }
            _ => panic!("First candidate for Major should be Ionian mode"),
        }
        
        match &candidates[1] {
            ScaleCandidate::Scale { kind, rank, .. } => {
                assert_eq!(*kind, ScaleKind::MajorPentatonic);
                assert_eq!(*rank, 2);
            }
            _ => panic!("Second candidate for Major should be MajorPentatonic scale"),
        }
        
        match &candidates[2] {
            ScaleCandidate::Mode { kind, rank, .. } => {
                assert_eq!(*kind, ModeKind::Lydian);
                assert_eq!(*rank, 3);
            }
            _ => panic!("Third candidate for Major should be Lydian mode"),
        }
    }
    
    #[test]
    fn test_minor_chord_candidates() {
        let candidates = KnownChord::Minor.scale_candidates();
        assert!(candidates.len() >= 3, "Minor chord should have at least 3 candidates");
        
        match &candidates[0] {
            ScaleCandidate::Mode { kind, rank, .. } => {
                assert_eq!(*kind, ModeKind::Aeolian);
                assert_eq!(*rank, 1);
            }
            _ => panic!("First candidate for Minor should be Aeolian mode"),
        }
        
        match &candidates[1] {
            ScaleCandidate::Scale { kind, rank, .. } => {
                assert_eq!(*kind, ScaleKind::MinorPentatonic);
                assert_eq!(*rank, 2);
            }
            _ => panic!("Second candidate for Minor should be MinorPentatonic scale"),
        }
        
        match &candidates[2] {
            ScaleCandidate::Scale { kind, rank, .. } => {
                assert_eq!(*kind, ScaleKind::Blues);
                assert_eq!(*rank, 3);
            }
            _ => panic!("Third candidate for Minor should be Blues scale"),
        }
    }
    
    #[test]
    fn test_minor_dominant_candidates() {
        let candidates = KnownChord::MinorDominant(Degree::Seven).scale_candidates();
        assert!(candidates.len() >= 3, "Minor dominant should have at least 3 candidates");
        
        match &candidates[0] {
            ScaleCandidate::Mode { kind, rank, .. } => {
                assert_eq!(*kind, ModeKind::Dorian);
                assert_eq!(*rank, 1);
            }
            _ => panic!("First candidate for Minor dominant should be Dorian mode"),
        }
        
        match &candidates[1] {
            ScaleCandidate::Scale { kind, rank, .. } => {
                assert_eq!(*kind, ScaleKind::MinorPentatonic);
                assert_eq!(*rank, 2);
            }
            _ => panic!("Second candidate for Minor dominant should be MinorPentatonic scale"),
        }
        
        match &candidates[2] {
            ScaleCandidate::Scale { kind, rank, .. } => {
                assert_eq!(*kind, ScaleKind::Blues);
                assert_eq!(*rank, 3);
            }
            _ => panic!("Third candidate for Minor dominant should be Blues scale"),
        }
    }
    
    #[test]
    fn test_interval_candidates_kinds_and_order() {
        // Test that all KnownChord variants return properly ordered interval candidates
        let all_variants = vec![
            KnownChord::Unknown,
            KnownChord::Major,
            KnownChord::Minor,
            KnownChord::Major7,
            KnownChord::Dominant(Degree::Seven),
            KnownChord::MinorMajor7,
            KnownChord::MinorDominant(Degree::Seven),
            KnownChord::DominantSharp11(Degree::Seven),
            KnownChord::Augmented,
            KnownChord::AugmentedMajor7,
            KnownChord::AugmentedDominant(Degree::Seven),
            KnownChord::HalfDiminished(Degree::Seven),
            KnownChord::Diminished,
            KnownChord::DominantFlat9(Degree::Seven),
            KnownChord::DominantSharp9(Degree::Seven),
            KnownChord::DominantFlat9Sharp5(Degree::Seven),
            KnownChord::MinorDominantFlat13(Degree::Seven),
            KnownChord::MinorDominantFlat9Flat13(Degree::Seven),
            KnownChord::Sharp11,
        ];
        
        for known_chord in all_variants {
            let candidates = known_chord.scale_interval_candidates();
            
            // Verify ranks are sequential starting at 1
            for (i, candidate) in candidates.iter().enumerate() {
                let expected_rank = (i + 1) as u8;
                assert_eq!(
                    candidate.rank, expected_rank,
                    "{:?}: Expected rank {} at position {}, got {}",
                    known_chord, expected_rank, i, candidate.rank
                );
                
                // Verify reason is non-empty
                assert!(
                    !candidate.reason.is_empty(),
                    "{:?}: Candidate at position {} has empty reason",
                    known_chord, i
                );
                
                // Verify kind matches what scale_candidates() returns
                let scale_candidates = known_chord.scale_candidates();
                if i < scale_candidates.len() {
                    match (&candidate.kind, &scale_candidates[i]) {
                        (IntervalCollectionKind::Mode(mk), ScaleCandidate::Mode { kind, .. }) => {
                            assert_eq!(mk, kind, "{:?}: Mode kind mismatch at position {}", known_chord, i);
                        }
                        (IntervalCollectionKind::Scale(sk), ScaleCandidate::Scale { kind, .. }) => {
                            assert_eq!(sk, kind, "{:?}: Scale kind mismatch at position {}", known_chord, i);
                        }
                        _ => panic!("{:?}: Kind type mismatch at position {}", known_chord, i),
                    }
                }
            }
            
            // Verify scale_candidates() matches interval_candidates()
            let scale_candidates = known_chord.scale_candidates();
            assert_eq!(
                candidates.len(),
                scale_candidates.len(),
                "{:?}: Mismatch in candidate count",
                known_chord
            );
        }
    }
}
