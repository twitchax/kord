// Traits.

pub trait HasDistance {
    fn distance(&self) -> u8;
}

// Enum.

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum Interval {
    PerfectUnison,
    DiminishedSecond,

    AugmentedUnison,
    MinorSecond,

    MajorSecond,
    DiminishedThird,

    AugmentedSecond,
    MinorThird,

    MajorThird,
    DiminishedFourth,

    AugmentedThird,
    PerfectFourth,

    AugmentedFourth,
    DiminishedFifth,

    PerfectFifth,
    DiminishedSixth,

    AugmentedFifth,
    MinorSixth,

    MajorSixth,
    DiminishedSeventh,

    AugmentedSixth,
    MinorSeventh,

    MajorSeventh,
    DiminishedOctave,

    AugmentedSeventh,
    PerfectOctave,

    MinorNinth,
    MajorNinth,
    AugmentedNinth,

    DiminishedEleventh,
    PerfectEleventh,
    AugmentedEleventh,

    MinorThirteenth,
    MajorThirteenth,
    AugmentedThirteenth,
}

// Impls.

impl HasDistance for Interval {
    fn distance(&self) -> u8 {
        match self {
            Interval::PerfectUnison => 0,
            Interval::DiminishedSecond => 1,

            Interval::AugmentedUnison => 4,
            Interval::MinorSecond => 5,

            Interval::MajorSecond => 8,
            Interval::DiminishedThird => 9,

            Interval::AugmentedSecond => 12,
            Interval::MinorThird => 13,

            Interval::MajorThird => 16,
            Interval::DiminishedFourth => 17,

            Interval::AugmentedThird => 19,
            Interval::PerfectFourth => 20,

            Interval::AugmentedFourth => 24,
            Interval::DiminishedFifth => 25,

            Interval::PerfectFifth => 28,
            Interval::DiminishedSixth => 29,

            Interval::AugmentedFifth => 32,
            Interval::MinorSixth => 33,
            
            Interval::MajorSixth => 36,
            Interval::DiminishedSeventh => 37,

            Interval::AugmentedSixth => 40,
            Interval::MinorSeventh => 41,

            Interval::MajorSeventh => 44,
            Interval::DiminishedOctave => 45,

            Interval::AugmentedSeventh => 47,
            Interval::PerfectOctave => 48,

            
            Interval::MinorNinth => 53,
            Interval::MajorNinth => 56,
            Interval::AugmentedNinth => 60,

            Interval::DiminishedEleventh => 65,
            Interval::PerfectEleventh => 69,
            Interval::AugmentedEleventh => 72,

            Interval::MinorThirteenth => 81,
            Interval::MajorThirteenth => 84,
            Interval::AugmentedThirteenth => 88,
        }
    }
}