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

            Interval::AugmentedUnison => 2,
            Interval::MinorSecond => 3,

            Interval::MajorSecond => 5,
            Interval::DiminishedThird => 6,

            Interval::AugmentedSecond => 7,
            Interval::MinorThird => 8,

            Interval::MajorThird => 10,
            Interval::DiminishedFourth => 11,

            Interval::AugmentedThird => 12,
            Interval::PerfectFourth => 13,

            Interval::AugmentedFourth => 15,
            Interval::DiminishedFifth => 16,

            Interval::PerfectFifth => 18,
            Interval::DiminishedSixth => 19,

            Interval::AugmentedFifth => 20,
            Interval::MinorSixth => 21,
            
            Interval::MajorSixth => 23,
            Interval::DiminishedSeventh => 24,

            Interval::AugmentedSixth => 25,
            Interval::MinorSeventh => 26,

            Interval::MajorSeventh => 28,
            Interval::DiminishedOctave => 29,

            Interval::AugmentedSeventh => 30,
            Interval::PerfectOctave => 31,

            
            Interval::MinorNinth => 34,
            Interval::MajorNinth => 36,
            Interval::AugmentedNinth => 38,

            Interval::DiminishedEleventh => 42,
            Interval::PerfectEleventh => 44,
            Interval::AugmentedEleventh => 46,

            Interval::MinorThirteenth => 52,
            Interval::MajorThirteenth => 54,
            Interval::AugmentedThirteenth => 56,
        }
    }
}