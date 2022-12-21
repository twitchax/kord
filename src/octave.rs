use std::ops::{Add, AddAssign, Sub, SubAssign};

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