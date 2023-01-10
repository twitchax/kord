use std::ops::{Add, AddAssign, Sub, SubAssign};

use once_cell::sync::Lazy;

use crate::base::HasStaticName;

// Traits.

/// A trait for types that have an octave property.
pub trait HasOctave {
    /// Returns the octave of the type.
    fn octave(&self) -> Octave;
}

// Enum.

/// An enum representing the octave of a note.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum Octave {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

// Octave impls.

impl HasStaticName for Octave {
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
        }
    }
}

impl Add for Octave {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let new_octave = self as u8 + rhs as u8;

        if new_octave > 10 {
            panic!("Octave overflow");
        }

        // SAFETY: The new octave is guaranteed to be less than or equal to 10.
        unsafe { std::mem::transmute(new_octave) }
    }
}

impl Add<i8> for Octave {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        let new_octave = self as i8 + rhs;

        if new_octave > 10 {
            panic!("Octave overflow.");
        } else if new_octave < 0 {
            panic!("Octave underflow.");
        }

        // SAFETY: The new octave is guaranteed to be less than or equal to 10.
        unsafe { std::mem::transmute(new_octave) }
    }
}

impl Sub<i8> for Octave {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        let new_octave = self as i8 - rhs;

        if new_octave > 10 {
            panic!("Octave overflow.");
        } else if new_octave < 0 {
            panic!("Octave underflow.");
        }

        // SAFETY: The new octave is guaranteed to be less than or equal to 10.
        unsafe { std::mem::transmute(new_octave) }
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

impl Default for Octave {
    fn default() -> Self {
        Octave::Four
    }
}

// Statics.

pub(crate) static ALL_OCTAVES: Lazy<[Octave; 11]> = Lazy::new(|| [
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
    Octave::Ten  
]);

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};
    use crate::{octave::HasOctave};

    #[test]
    #[should_panic]
    fn test_self_overflow() {
        let _ = Octave::Ten + Octave::One;
    }

    #[test]
    #[should_panic]
    fn test_i8_add_overflow() {
        let _ = Octave::Ten + 1;
    }

    #[test]
    #[should_panic]
    fn test_i8_add_underflow() {
        let _ = Octave::Zero + -1;
    }

    #[test]
    #[should_panic]
    fn test_i8_sub_overflow() {
        let _ = Octave::Ten - -1;
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
}