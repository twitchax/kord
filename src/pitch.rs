// Traits.

use once_cell::sync::Lazy;

/// A trait for types that have a pitch property.
pub trait HasPitch {
    /// Returns the pitch of the type (usually a [`Note`]).
    fn pitch(&self) -> Pitch;
}

/// A trait for types that have a base frequency property.
pub trait HasBaseFrequency {
    /// Returns the base frequency of the type (usually a [`Pitch`]).
    fn base_frequency(&self) -> f32;
}

/// A trait for types that have a frequency property.
pub trait HasFrequency {
    /// Returns the frequency of the type (usually a [`Note`]).
    fn frequency(&self) -> f32;
}

// Enum.

/// An enum representing the pitch of a note.
///
/// The frequencies of the pitches are based on the [A4 frequency](https://en.wikipedia.org/wiki/A4_(pitch_standard)).
/// There is no enharmonic representation here, so all of the sharps are represented.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum Pitch {
    /// The pitch C.
    C,
    /// The pitch C♯.
    CSharp,
    /// The pitch D.
    D,
    /// The pitch D♯.
    DSharp,
    /// The pitch E.
    E,
    /// The pitch F.
    F,
    /// The pitch F♯.
    FSharp,
    /// The pitch G.
    G,
    /// The pitch G♯.
    GSharp,
    /// The pitch A.
    A,
    /// The pitch A♯.
    ASharp,
    /// The pitch B.
    B,
}

// Pitch impls.

impl HasBaseFrequency for Pitch {
    #[no_coverage]
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

// Statics.

pub(crate) static ALL_PITCHES: Lazy<[Pitch; 12]> = Lazy::new(|| {
    [
        Pitch::C,
        Pitch::CSharp,
        Pitch::D,
        Pitch::DSharp,
        Pitch::E,
        Pitch::F,
        Pitch::FSharp,
        Pitch::G,
        Pitch::GSharp,
        Pitch::A,
        Pitch::ASharp,
        Pitch::B,
    ]
});

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_properties() {
        assert_eq!(Pitch::G.pitch(), Pitch::G);
        assert_eq!(Pitch::G.base_frequency(), 24.50);
    }
}
