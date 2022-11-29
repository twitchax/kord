// Traits.

use std::ops::Add;

pub trait HasOctave {
    fn octave(&self) -> Octave;
}

// Enum.

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

// Blanket impls.