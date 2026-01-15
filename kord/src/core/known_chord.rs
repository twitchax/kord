//! A module for working with known chords.

use crate::core::{
    base::{HasDescription, HasName, HasStaticName},
    interval::Interval,
    mode_kind::ModeKind,
    modifier::Degree,
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
            KnownChord::Major7 => "major 7, ionian, first mode of major scale",
            KnownChord::Dominant(_) => "dominant, mixolydian, fifth mode of major scale, major with flat seven",
            KnownChord::MinorMajor7 => "minor major 7, melodic minor, major with flat third",
            KnownChord::MinorDominant(_) => "minor 7, dorian, second mode of major scale, major with flat third and flat seven",
            KnownChord::DominantSharp11(_) => "dominant sharp 11, lydian dominant, lyxian, major with sharp four and flat seven",
            KnownChord::Augmented => "augmented, major with sharp five",
            KnownChord::AugmentedMajor7 => "augmented major 7, major with sharp four and five, third mode of melodic minor",
            KnownChord::AugmentedDominant(_) => "augmented dominant, whole tone",
            KnownChord::HalfDiminished(_) => "half diminished, locrian, minor seven flat five, seventh mode of major scale, major scale starting one half step up",
            KnownChord::Diminished => "fully diminished (whole first), diminished seventh, whole/half/whole diminished",
            KnownChord::DominantFlat9(_) => "dominant flat 9, fully diminished (half first), half/whole/half diminished",
            KnownChord::DominantSharp9(_) => "dominant sharp 9, altered, altered dominant, super locrian, diminished whole tone, seventh mode of a melodic minor scale, melodic minor up a half step",
            KnownChord::MinorDominantFlat13(_) => "minor dominant flat 13, aeolian, sixth mode of major scale",
            KnownChord::MinorDominantFlat9Flat13(_) => "dominant flat 9 flat 13, phrygian, third mode of a major scale",
            KnownChord::Sharp11 => "sharp 11, lydian, fourth mode of a major scale",
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
            KnownChord::MinorDominantFlat13(d) => format!("m{}(♭13)", d.static_name()),
            KnownChord::MinorDominantFlat9Flat13(d) => format!("{}(♭9)(♭13)", d.static_name()),
            KnownChord::Sharp11 => "(♯11)".to_owned(),
        }
    }
}

impl HasScaleCandidates for KnownChord {
    fn scale_candidates(&self) -> Vec<ScaleCandidate> {
        match self {
            KnownChord::Unknown => vec![],
            KnownChord::Major => vec![
                ScaleCandidate::Mode {
                    kind: ModeKind::Ionian,
                    rank: 1,
                    reason: "Primary major scale - natural fit for major triad",
                },
                ScaleCandidate::Scale {
                    kind: ScaleKind::MajorPentatonic,
                    rank: 2,
                    reason: "Five-note major sound - safe, consonant choice",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::Lydian,
                    rank: 3,
                    reason: "Bright alternative with ♯4 for added color",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::Mixolydian,
                    rank: 4,
                    reason: "Major with ♭7 - common in blues and rock",
                },
            ],
            KnownChord::Minor => vec![
                ScaleCandidate::Mode {
                    kind: ModeKind::Aeolian,
                    rank: 1,
                    reason: "Natural minor - primary choice for minor triads",
                },
                ScaleCandidate::Scale {
                    kind: ScaleKind::MinorPentatonic,
                    rank: 2,
                    reason: "Five-note minor sound - blues and rock standard",
                },
                ScaleCandidate::Scale {
                    kind: ScaleKind::Blues,
                    rank: 3,
                    reason: "Minor pentatonic with ♭5 - essential blues scale",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::Dorian,
                    rank: 4,
                    reason: "Minor with ♮6 - jazzy, brighter minor sound",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::Phrygian,
                    rank: 5,
                    reason: "Minor with ♭2 - exotic, Spanish flavor",
                },
                ScaleCandidate::Scale {
                    kind: ScaleKind::HarmonicMinor,
                    rank: 6,
                    reason: "Classical minor with ♮7 for strong resolution",
                },
            ],
            KnownChord::Major7 => vec![
                ScaleCandidate::Mode {
                    kind: ModeKind::Ionian,
                    rank: 1,
                    reason: "Natural major 7th from major scale",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::Lydian,
                    rank: 2,
                    reason: "Bright maj7 sound with ♯4 for modern jazz",
                },
            ],
            KnownChord::Dominant(_) => vec![
                ScaleCandidate::Mode {
                    kind: ModeKind::Mixolydian,
                    rank: 1,
                    reason: "Primary dominant scale - major with ♭7",
                },
                ScaleCandidate::Scale {
                    kind: ScaleKind::Blues,
                    rank: 2,
                    reason: "Essential blues sound over dominant chords",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::LydianDominant,
                    rank: 3,
                    reason: "Dominant with ♯11 for sophisticated color",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::MixolydianFlat6,
                    rank: 4,
                    reason: "Dominant with ♭13 for darker, minor-leaning sound",
                },
                ScaleCandidate::Scale {
                    kind: ScaleKind::WholeTone,
                    rank: 5,
                    reason: "Symmetrical scale for augmented dominant color",
                },
            ],
            KnownChord::MinorMajor7 => vec![
                ScaleCandidate::Scale {
                    kind: ScaleKind::MelodicMinor,
                    rank: 1,
                    reason: "Source scale for minor-major 7 sound",
                },
                ScaleCandidate::Scale {
                    kind: ScaleKind::HarmonicMinor,
                    rank: 2,
                    reason: "Alternative with ♮7 and ♭6",
                },
            ],
            KnownChord::MinorDominant(_) => vec![
                ScaleCandidate::Mode {
                    kind: ModeKind::Dorian,
                    rank: 1,
                    reason: "Classic minor 7 scale - minor with ♮6",
                },
                ScaleCandidate::Scale {
                    kind: ScaleKind::MinorPentatonic,
                    rank: 2,
                    reason: "Simple, effective minor 7 choice",
                },
                ScaleCandidate::Scale {
                    kind: ScaleKind::Blues,
                    rank: 3,
                    reason: "Blues flavor over minor 7 chords",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::Aeolian,
                    rank: 4,
                    reason: "Natural minor alternative",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::Phrygian,
                    rank: 5,
                    reason: "Minor 7 with ♭2 for modal flavor",
                },
            ],
            KnownChord::DominantSharp11(_) => vec![
                ScaleCandidate::Mode {
                    kind: ModeKind::LydianDominant,
                    rank: 1,
                    reason: "Defining scale for dominant ♯11 sound",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::Mixolydian,
                    rank: 2,
                    reason: "Basic dominant scale alternative",
                },
            ],
            KnownChord::Augmented => vec![
                ScaleCandidate::Scale {
                    kind: ScaleKind::WholeTone,
                    rank: 1,
                    reason: "Symmetrical scale built from augmented triads",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::LydianAugmented,
                    rank: 2,
                    reason: "Major with ♯4 and ♯5",
                },
            ],
            KnownChord::AugmentedMajor7 => vec![
                ScaleCandidate::Mode {
                    kind: ModeKind::LydianAugmented,
                    rank: 1,
                    reason: "3rd mode of melodic minor - major 7 with ♯5",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::IonianSharp5,
                    rank: 2,
                    reason: "Major with ♯5 from harmonic minor",
                },
            ],
            KnownChord::AugmentedDominant(_) => vec![
                ScaleCandidate::Scale {
                    kind: ScaleKind::WholeTone,
                    rank: 1,
                    reason: "Primary scale for augmented dominant chords",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::LydianDominant,
                    rank: 2,
                    reason: "Can be used with ♯5 alterations",
                },
            ],
            KnownChord::HalfDiminished(_) => vec![
                ScaleCandidate::Mode {
                    kind: ModeKind::Locrian,
                    rank: 1,
                    reason: "Primary half-diminished scale - 7th mode of major",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::LocrianNatural2,
                    rank: 2,
                    reason: "Half-diminished with ♮2 - smoother melodic motion",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::LocrianNatural6,
                    rank: 3,
                    reason: "Half-diminished with ♮6 from harmonic minor",
                },
            ],
            KnownChord::Diminished => vec![
                ScaleCandidate::Scale {
                    kind: ScaleKind::DiminishedWholeHalf,
                    rank: 1,
                    reason: "Symmetrical scale for fully diminished 7th chords",
                },
                ScaleCandidate::Scale {
                    kind: ScaleKind::DiminishedHalfWhole,
                    rank: 2,
                    reason: "Alternative diminished scale pattern",
                },
            ],
            KnownChord::DominantFlat9(_) => vec![
                ScaleCandidate::Scale {
                    kind: ScaleKind::DiminishedHalfWhole,
                    rank: 1,
                    reason: "Primary scale for dominant ♭9 - half-whole pattern",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::PhrygianDominant,
                    rank: 2,
                    reason: "Spanish sound with ♭9 and major 3rd",
                },
            ],
            KnownChord::DominantSharp9(_) => vec![
                ScaleCandidate::Mode {
                    kind: ModeKind::Altered,
                    rank: 1,
                    reason: "Altered dominant scale - all alterations available",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::DorianFlat2,
                    rank: 2,
                    reason: "Minor with ♭2 providing ♯9 color",
                },
            ],
            KnownChord::MinorDominantFlat13(_) => vec![
                ScaleCandidate::Mode {
                    kind: ModeKind::Aeolian,
                    rank: 1,
                    reason: "Natural minor with ♭6/♭13",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::Phrygian,
                    rank: 2,
                    reason: "Minor with ♭2 and ♭6",
                },
            ],
            KnownChord::MinorDominantFlat9Flat13(_) => vec![
                ScaleCandidate::Mode {
                    kind: ModeKind::Phrygian,
                    rank: 1,
                    reason: "Minor with ♭2 (♭9) and ♭6 (♭13)",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::DorianFlat2,
                    rank: 2,
                    reason: "Minor with ♭2 and ♮6 for contrast",
                },
            ],
            KnownChord::Sharp11 => vec![
                ScaleCandidate::Mode {
                    kind: ModeKind::Lydian,
                    rank: 1,
                    reason: "Major with ♯11 for bright, modern sound",
                },
                ScaleCandidate::Mode {
                    kind: ModeKind::LydianDominant,
                    rank: 2,
                    reason: "Dominant with ♯11",
                },
            ],
        }
    }
}
