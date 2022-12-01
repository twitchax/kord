use crate::{interval::Interval, base::{HasStaticName, HasDescription, HasName}, modifier::Degree};

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
    Dominant(Degree),
    MinorMajor7,
    MinorDominant(Degree),
    DominantSharp11(Degree),
    Augmented,
    AugmentedMajor7,
    AugmentedDominant(Degree),
    HalfDiminished(Degree),
    Diminished,
    DominantFlat9(Degree),
    DominantSharp9(Degree)
}

// Impls.

impl HasDescription for KnownChord {
    fn description(&self) -> &'static str {
        match self {
            KnownChord::Unknown => "Unknown",
            KnownChord::Major => "major",
            KnownChord::Minor => "minor",
            KnownChord::Major7 => "major 7, ionian, first mode of major scale",
            KnownChord::Dominant(_) => "dominant, mixolydian, fifth mode of major scale, major with flat seven",
            KnownChord::MinorMajor7 => "minor major 7, melodic minor, major with flat third",
            KnownChord::MinorDominant(_) => "minor 7, dorian, second mode of major scale, major with flat third and flat seven",
            KnownChord::DominantSharp11(_) => "dominant sharp 11, lydian dominant, lyxian, major with sharp four and flat seven",
            KnownChord::Augmented => "augmented, major with sharp five",
            KnownChord::AugmentedMajor7 => "augmented major 7, lyxian, major with sharp four and five, third mode of melodic minor",
            KnownChord::AugmentedDominant(_) => "augmented dominant, whole tone",
            KnownChord::HalfDiminished(_) => "half diminished, locrian, minor seven flat five, seventh mode of major scale, major scale starting one half step up",
            KnownChord::Diminished => "fully diminished (whole first), diminished seventh, whole/half/whole diminished",
            KnownChord::DominantFlat9(_) => "dominant flat 9, fully diminished (half first), half/whole/half diminished",
            KnownChord::DominantSharp9(_) => "dominant sharp 9, altered, altered dominant, super locrian, diminished whole tone, seventh mode of a melodic minor scale, melodic minor up a half step",
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
            KnownChord::Dominant(_) => vec![
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
            KnownChord::MinorDominant(_) => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MinorThird, 
                Interval::PerfectFourth, 
                Interval::PerfectFifth, 
                Interval::MajorSixth, 
                Interval::MinorSeventh
            ],
            KnownChord::DominantSharp11(_) => vec![
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
            KnownChord::AugmentedDominant(_) => vec![
                Interval::PerfectUnison, 
                Interval::MajorSecond, 
                Interval::MajorThird, 
                Interval::AugmentedFourth, 
                Interval::AugmentedFifth,
                Interval::AugmentedSixth
            ],
            KnownChord::HalfDiminished(_ ) => vec![
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
            KnownChord::DominantFlat9(_) => vec![
                Interval::PerfectUnison, 
                Interval::MinorSecond, 
                Interval::MinorThird,
                Interval::MajorThird,
                Interval::AugmentedFourth, 
                Interval::PerfectFifth, 
                Interval::MajorSixth, 
                Interval::MinorSeventh
            ],
            KnownChord::DominantSharp9(_) => vec![
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
            KnownChord::Dominant(_) => vec![
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
            KnownChord::MinorDominant(_) => vec![
                Interval::PerfectUnison, 
                Interval::MinorThird, 
                Interval::PerfectFifth, 
                Interval::MinorSeventh
            ],
            KnownChord::DominantSharp11(_) => vec![
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
            KnownChord::AugmentedDominant(_) => vec![
                Interval::PerfectUnison, 
                Interval::MajorThird, 
                Interval::AugmentedFifth, 
                Interval::MinorSeventh
            ],
            KnownChord::HalfDiminished(_) => vec![
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
            KnownChord::DominantFlat9(_) => vec![
                Interval::PerfectUnison, 
                Interval::MajorThird, 
                Interval::PerfectFifth, 
                Interval::MinorSeventh,
                Interval::MinorNinth
            ],
            KnownChord::DominantSharp9(_) => vec![
                Interval::PerfectUnison, 
                Interval::MajorThird, 
                Interval::PerfectFifth, 
                Interval::MinorSeventh,
                Interval::AugmentedNinth
            ],
        }
    }
}

impl HasName for KnownChord {
    fn name(&self) -> String {
        match self {
            KnownChord::Unknown => "".to_owned(),
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
        }
    }
}