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
        }
    }

    /// Returns the degree of the parent scale that this mode starts on (for documentation only).
    ///
    /// This is metadata - the intervals are NOT derived from the parent.
    pub fn parent_degree(&self) -> u8 {
        match self {
            ModeKind::Ionian => 1,
            ModeKind::Dorian => 2,
            ModeKind::Phrygian => 3,
            ModeKind::Lydian => 4,
            ModeKind::Mixolydian => 5,
            ModeKind::Aeolian => 6,
            ModeKind::Locrian => 7,
        }
    }
}

impl HasIntervals for ModeKind {
    fn intervals(&self) -> &'static [Interval] {
        match self {
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
        }
    }
}

impl HasDescription for ModeKind {
    fn description(&self) -> &'static str {
        match self {
            ModeKind::Ionian => "ionian, 1st mode of major scale, major scale",
            ModeKind::Dorian => "dorian, 2nd mode of major scale, minor with raised 6th",
            ModeKind::Phrygian => "phrygian, 3rd mode of major scale, minor with lowered 2nd",
            ModeKind::Lydian => "lydian, 4th mode of major scale, major with raised 4th",
            ModeKind::Mixolydian => "mixolydian, 5th mode of major scale, major with lowered 7th",
            ModeKind::Aeolian => "aeolian, 6th mode of major scale, natural minor",
            ModeKind::Locrian => "locrian, 7th mode of major scale, diminished, half-diminished chord scale",
        }
    }
}

impl HasStaticName for ModeKind {
    fn static_name(&self) -> &'static str {
        match self {
            ModeKind::Ionian => "ionian",
            ModeKind::Dorian => "dorian",
            ModeKind::Phrygian => "phrygian",
            ModeKind::Lydian => "lydian",
            ModeKind::Mixolydian => "mixolydian",
            ModeKind::Aeolian => "aeolian",
            ModeKind::Locrian => "locrian",
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
