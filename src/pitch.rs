// Traits.

use crate::octave::{HasOctave};

pub trait HasPitch {
    fn pitch(&self) -> Pitch;
}

pub trait HasBaseFrequency {
    fn base_frequency(&self) -> f32;
}

pub trait HasFrequency: HasBaseFrequency + HasOctave {
    fn frequency(&self) -> f32;
}

// Enum.

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum Pitch {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

// Pitch impls.

impl HasBaseFrequency for Pitch {
    fn base_frequency(&self) -> f32 {
        match self {
            Pitch::C => 16.35,
            Pitch::CSharp => 17.32,
            Pitch::D => 18.35,
            Pitch::DSharp => 19.45,
            Pitch::E => 20.60,
            Pitch::F => 21.83,
            Pitch::FSharp => 23.12,
            Pitch::G => 24.50,
            Pitch::GSharp => 25.96,
            Pitch::A => 27.50,
            Pitch::ASharp => 29.14,
            Pitch::B => 30.87,
        }
    }
}

impl HasPitch for Pitch {
    fn pitch(&self) -> Pitch {
        *self
    }
}

// Blanket impls.

default impl<T> HasBaseFrequency for T
where
    T: HasPitch,
{
    fn base_frequency(&self) -> f32 {
        self.pitch().base_frequency()
    }
}

default impl<T> HasFrequency for T
where
    T: HasBaseFrequency + HasOctave
{
    fn frequency(&self) -> f32 {
        let octave = self.octave();
        let base_frequency = self.base_frequency();

        base_frequency * 2.0_f32.powf(octave as u8 as f32)
    }
}