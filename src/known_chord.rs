use crate::{interval::Interval, base::{HasStaticName, HasDescription}};

// Traits.

pub trait HasRelativeScale {
    fn relative_scale(&self) -> Vec<Interval>;
}

pub trait HasRelativeChord {
    fn relative_chord(&self) -> Vec<Interval>;
}

// Enum.

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum KnownChord {
    Unknown,
    Major,
    Minor,
    Major7,
    Dominant,
    MinorMajor7,
    MinorDominant,
    DominantSharp11,
    Augmented,
    AugmentedMajor7,
    AugmentedDominant,
    HalfDiminished,
    Diminished,
    DominantFlat9,
    DominantSharp9
}

// Impls.

impl HasDescription for KnownChord {
    fn description(&self) -> &'static str {
        match self {
            KnownChord::Unknown => "Unknown",
            KnownChord::Major => "major",
            KnownChord::Minor => "minor",
            KnownChord::Major7 => "major 7, ionian, first mode of major scale",
            KnownChord::Dominant => "dominant 7, mixolydian, fifth mode of major scale, major with flat seven",
            KnownChord::MinorMajor7 => "minor major 7, melodic minor, major with flat third",
            KnownChord::MinorDominant => "minor 7, dorian, second mode of major scale, major with flat third and flat seven",
            KnownChord::DominantSharp11 => "dominant sharp 11, lydian dominant, lyxian, major with sharp four and flat seven",
            KnownChord::Augmented => "augmented, major with sharp five",
            KnownChord::AugmentedMajor7 => "augmented major 7, lyxian, major with sharp four and five, third mode of melodic minor",
            KnownChord::AugmentedDominant => "augmented 7, whole tone",
            KnownChord::HalfDiminished => "half diminished, locrian, minor seven flat five, seventh mode of major scale, major scale starting one half step up",
            KnownChord::Diminished => "fully diminished (whole first), diminished seventh, whole/half/whole diminished",
            KnownChord::DominantFlat9 => "dominant flat 9, fully diminished (half first), half/whole/half diminished",
            KnownChord::DominantSharp9 => "dominant sharp 9, altered, altered dominant, super locrian, diminished whole tone, seventh mode of a melodic minor scale, melodic minor up a half step",
        }
    }
}

impl HasRelativeScale for KnownChord {
    fn relative_scale(&self) -> Vec<Interval> {
        match self {
            KnownChord::Unknown => vec![

            ],
            KnownChord::Major => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MajorThird, 
                Interval::PerfectFourth, 
                Interval::PerfectFifth, 
                Interval::MajorSixth, 
                Interval::MajorSeventh
            ],
            KnownChord::Minor => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MinorThird, 
                Interval::PerfectFourth, 
                Interval::PerfectFifth, 
                Interval::MinorSixth, 
                Interval::MinorSeventh
            ],
            KnownChord::Major7 => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MajorThird, 
                Interval::PerfectFourth, 
                Interval::PerfectFifth, 
                Interval::MajorSixth, 
                Interval::MajorSeventh
            ],
            KnownChord::Dominant => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MajorThird, 
                Interval::PerfectFourth, 
                Interval::PerfectFifth, 
                Interval::MajorSixth, 
                Interval::MinorSeventh
            ],
            KnownChord::MinorMajor7 => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MinorThird, 
                Interval::PerfectFourth, 
                Interval::PerfectFifth, 
                Interval::MajorSixth, 
                Interval::MajorSeventh
            ],
            KnownChord::MinorDominant => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MinorThird, 
                Interval::PerfectFourth, 
                Interval::PerfectFifth, 
                Interval::MajorSixth, 
                Interval::MinorSeventh
            ],
            KnownChord::DominantSharp11 => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MajorThird, 
                Interval::AugmentedFourth, 
                Interval::PerfectFifth, 
                Interval::MajorSixth, 
                Interval::MinorSeventh
            ],
            KnownChord::Augmented => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MajorThird, 
                Interval::PerfectFourth, 
                Interval::AugmentedFifth, 
                Interval::MajorSixth,
                Interval::MajorSeventh
            ],
            KnownChord::AugmentedMajor7 => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MajorThird, 
                Interval::AugmentedFourth, 
                Interval::AugmentedFifth, 
                Interval::MajorSixth,
                Interval::MajorSeventh
            ],
            KnownChord::AugmentedDominant => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MajorThird, 
                Interval::AugmentedFourth, 
                Interval::AugmentedFifth,
                Interval::AugmentedSixth
            ],
            KnownChord::HalfDiminished => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MinorThird, 
                Interval::PerfectFourth, 
                Interval::DiminishedFifth, 
                Interval::MinorSixth, 
                Interval::MinorSeventh
            ],
            KnownChord::Diminished => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MinorThird, 
                Interval::PerfectFourth, 
                Interval::DiminishedFifth, 
                Interval::MinorSixth, 
                Interval::DiminishedSeventh,
                Interval::MajorSeventh
            ],
            KnownChord::DominantFlat9 => vec![
                Interval::PerfectUnison, 
                Interval::MinorSecond, 
                Interval::MinorThird,
                Interval::MajorThird,
                Interval::AugmentedFourth, 
                Interval::PerfectFifth, 
                Interval::MajorSixth, 
                Interval::MinorSeventh
            ],
            KnownChord::DominantSharp9 => vec![
                Interval::PerfectUnison, 
                Interval::MinorSecond,
                Interval::MinorThird,
                Interval::DiminishedFourth, 
                Interval::DiminishedFifth, 
                Interval::MinorSixth,
                Interval::MinorSeventh
            ],
        }
    }
}

impl HasRelativeChord for KnownChord {
    fn relative_chord(&self) -> Vec<Interval> {
        match self {
            KnownChord::Unknown => vec![

            ],
            KnownChord::Major => vec![
                Interval::PerfectUnison, 
                Interval::MajorThird, 
                Interval::PerfectFifth
            ],
            KnownChord::Minor => vec![
                Interval::PerfectUnison, 
                Interval::MinorThird, 
                Interval::PerfectFifth
            ],
            KnownChord::Major7 => vec![
                Interval::PerfectUnison, 
                Interval::MajorThird, 
                Interval::PerfectFifth, 
                Interval::MajorSeventh
            ],
            KnownChord::Dominant => vec![
                Interval::PerfectUnison, 
                Interval::MajorThird, 
                Interval::PerfectFifth, 
                Interval::MinorSeventh
            ],
            KnownChord::MinorMajor7 => vec![
                Interval::PerfectUnison, 
                Interval::MinorThird, 
                Interval::PerfectFifth, 
                Interval::MajorSeventh
            ],
            KnownChord::MinorDominant => vec![
                Interval::PerfectUnison, 
                Interval::MinorThird, 
                Interval::PerfectFifth, 
                Interval::MinorSeventh
            ],
            KnownChord::DominantSharp11 => vec![
                Interval::PerfectUnison, 
                Interval::MajorThird, 
                Interval::PerfectFifth, 
                Interval::MinorSeventh,
                Interval::AugmentedEleventh
            ],
            KnownChord::Augmented => vec![
                Interval::PerfectUnison, 
                Interval::MajorThird, 
                Interval::AugmentedFifth
            ],
            KnownChord::AugmentedMajor7 => vec![
                Interval::PerfectUnison, 
                Interval::MajorThird, 
                Interval::AugmentedFifth, 
                Interval::MajorSeventh
            ],
            KnownChord::AugmentedDominant => vec![
                Interval::PerfectUnison, 
                Interval::MajorThird, 
                Interval::AugmentedFifth, 
                Interval::MinorSeventh
            ],
            KnownChord::HalfDiminished => vec![
                Interval::PerfectUnison, 
                Interval::MinorThird, 
                Interval::DiminishedFifth, 
                Interval::MinorSeventh
            ],
            KnownChord::Diminished => vec![
                Interval::PerfectUnison, 
                Interval::MinorThird, 
                Interval::DiminishedFifth, 
                Interval::DiminishedSeventh
            ],
            KnownChord::DominantFlat9 => vec![
                Interval::PerfectUnison, 
                Interval::MajorThird, 
                Interval::PerfectFifth, 
                Interval::MinorSeventh,
                Interval::MinorNinth
            ],
            KnownChord::DominantSharp9 => vec![
                Interval::PerfectUnison, 
                Interval::MajorThird, 
                Interval::PerfectFifth, 
                Interval::MinorSeventh,
                Interval::AugmentedNinth
            ],
        }
    }
}

impl HasStaticName for KnownChord {
    fn static_name(&self) -> &'static str {
        match self {
            KnownChord::Unknown => "",
            KnownChord::Major => "",
            KnownChord::Minor => "m",
            KnownChord::Major7 => "maj7",
            KnownChord::Dominant => "7",
            KnownChord::MinorMajor7 => "m(maj7)",
            KnownChord::MinorDominant => "m7",
            KnownChord::DominantSharp11 => "7(♯11)",
            KnownChord::Augmented => "+",
            KnownChord::AugmentedMajor7 => "+(maj7)",
            KnownChord::AugmentedDominant => "+7",
            KnownChord::HalfDiminished => "m7(♭5)",
            KnownChord::Diminished => "dim",
            KnownChord::DominantFlat9 => "7(♭9)",
            KnownChord::DominantSharp9 => "7(♯9)",
        }
    }
}