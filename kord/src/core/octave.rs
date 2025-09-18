//! A module for the octave of a note.

use std::ops::{Add, AddAssign, Sub, SubAssign};

use std::sync::LazyLock;

use crate::core::base::HasStaticName;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Traits.

/// A trait for types that have an octave property.
pub trait HasOctave {
    /// Returns the octave of the type.
    fn octave(&self) -> Octave;
}

// Enum.

/// An enum representing the octave of a note.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default, Ord, PartialOrd)]
#[repr(u8)]
pub enum Octave {
    /// The octave 0.
    Zero,
    /// The octave 1.
    One,
    /// The octave 2.
    Two,
    /// The octave 3.
    Three,
    /// The octave 4.
    #[default]
    Four,
    /// The octave 5.
    Five,
    /// The octave 6.
    Six,
    /// The octave 7.
    Seven,
    /// The octave 8.
    Eight,
    /// The octave 9.
    Nine,
    /// The octave 10.
    Ten,
    /// The octave 11.
    Eleven,
    /// The octave 12.
    Twelve,
    /// The octave 13.
    Thirteen,
    /// The octave 14.
    Fourteen,
    /// The octave 15.
    Fifteen,
}

// Octave impls.

impl HasStaticName for Octave {
    #[inline]
    fn static_name(&self) -> &'static str {
        match self {
            Octave::Zero => "0",
            Octave::One => "1",
            Octave::Two => "2",
            Octave::Three => "3",
            Octave::Four => "4",
            Octave::Five => "5",
            Octave::Six => "6",
            Octave::Seven => "7",
            Octave::Eight => "8",
            Octave::Nine => "9",
            Octave::Ten => "10",
            Octave::Eleven => "11",
            Octave::Twelve => "12",
            Octave::Thirteen => "13",
            Octave::Fourteen => "14",
            Octave::Fifteen => "15",
        }
    }

    #[inline]
    fn static_name_ascii(&self) -> &'static str {
        self.static_name()
    }
}

impl Add for Octave {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let new_octave = self as u8 + rhs as u8;

        assert!(new_octave <= 15, "Octave overflow");

        // SAFETY: The new octave is guaranteed to be less than or equal to 15.
        unsafe { std::mem::transmute(new_octave) }
    }
}

impl Sub for Octave {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_octave = (self as u8).checked_sub(rhs as u8).expect("Octave underflow.");

        assert!(new_octave <= 15, "Octave overflow");

        // SAFETY: The new octave is guaranteed to be less than or equal to 15.
        unsafe { std::mem::transmute(new_octave) }
    }
}

impl TryFrom<u8> for Octave {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 15 {
            Err("Octave overflow.")
        } else {
            // SAFETY: The new octave is guaranteed to be less than or equal to 15.
            Ok(unsafe { std::mem::transmute::<u8, Octave>(value) })
        }
    }
}

impl Add<i8> for Octave {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        let new_octave = self as i8 + rhs;

        if new_octave > 15 {
            panic!("Octave overflow.");
        } else if new_octave < 0 {
            panic!("Octave underflow.");
        }

        // SAFETY: The new octave is guaranteed to be less than or equal to 15.
        unsafe { std::mem::transmute(new_octave) }
    }
}

impl Sub<i8> for Octave {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        self + (-rhs)
    }
}

impl AddAssign for Octave {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<i8> for Octave {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl SubAssign<i8> for Octave {
    fn sub_assign(&mut self, rhs: i8) {
        *self = *self - rhs;
    }
}

impl HasOctave for Octave {
    fn octave(&self) -> Octave {
        *self
    }
}

// Statics.

/// An array of all octaves.
pub static ALL_OCTAVES: LazyLock<[Octave; 16]> = LazyLock::new(|| {
    [
        Octave::Zero,
        Octave::One,
        Octave::Two,
        Octave::Three,
        Octave::Four,
        Octave::Five,
        Octave::Six,
        Octave::Seven,
        Octave::Eight,
        Octave::Nine,
        Octave::Ten,
        Octave::Eleven,
        Octave::Twelve,
        Octave::Thirteen,
        Octave::Fourteen,
        Octave::Fifteen,
    ]
});

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::octave::HasOctave;
    use pretty_assertions::assert_eq;

    #[test]
    #[should_panic]
    fn test_self_overflow() {
        let _ = Octave::Fifteen + Octave::One;
    }

    #[test]
    #[should_panic]
    fn test_i8_add_overflow() {
        let _ = Octave::Fifteen + 1;
    }

    #[test]
    #[should_panic]
    fn test_i8_add_underflow() {
        let _ = Octave::Zero + -1;
    }

    #[test]
    #[should_panic]
    fn test_i8_sub_overflow() {
        let _ = Octave::Fifteen - -1;
    }

    #[test]
    #[should_panic]
    fn test_i8_sub_underflow() {
        let _ = Octave::Zero - 1;
    }

    #[test]
    fn test_add_assign_self() {
        let mut a = Octave::Four;
        a += Octave::One;
        assert_eq!(a, Octave::Five);
    }

    #[test]
    fn test_add_assign_i8() {
        let mut a = Octave::Four;
        a += 1;
        assert_eq!(a, Octave::Five);
    }

    #[test]
    fn test_sub_assign_i8() {
        let mut a = Octave::Four;
        a -= 1;
        assert_eq!(a, Octave::Three);
    }

    #[test]
    fn test_properties() {
        assert_eq!(Octave::Four.octave(), Octave::Four);
        assert_eq!(Octave::default(), Octave::Four);
    }

    #[test]
    fn test_names() {
        assert_eq!(ALL_OCTAVES.map(|o| o.static_name()).join(" "), "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15");
    }
}
