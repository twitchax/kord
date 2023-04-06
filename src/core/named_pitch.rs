//! A module for working with named pitches.

use std::ops::{Add, Sub};

use crate::core::{
    base::HasStaticName,
    pitch::{HasPitch, Pitch},
};

// Traits.

/// A trait for types that have a named pitch.
pub trait HasNamedPitch {
    /// Returns the named pitch of the type.
    fn named_pitch(&self) -> NamedPitch;
}

/// A trait for types that have a letter.
pub trait HasLetter {
    /// Returns the letter of the type.
    fn letter(&self) -> &'static str;
}

// Enum.

/// An enum representing named pitch.
///
/// A [`NamedPitch`] is a pitch that has a name, such as `C` or `F♯`.
/// While a [`Pitch`] is a pitch that has a frequency, a [`NamedPitch`] is a pitch that has an
/// enharmonic name (could share the same pitch with another).
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum NamedPitch {
    /// The pitch F triple flat.
    FTripleFlat,
    /// The pitch C triple flat.
    CTripleFlat,
    /// The pitch G triple flat.
    GTripleFlat,
    /// The pitch D triple flat.
    DTripleFlat,
    /// The pitch A triple flat.
    ATripleFlat,
    /// The pitch E triple flat.
    ETripleFlat,
    /// The pitch B triple flat.
    BTripleFlat,

    /// The pitch F double flat.
    FDoubleFlat,
    /// The pitch C double flat.
    CDoubleFlat,
    /// The pitch G double flat.
    GDoubleFlat,
    /// The pitch D double flat.
    DDoubleFlat,
    /// The pitch A double flat.
    ADoubleFlat,
    /// The pitch E double flat.
    EDoubleFlat,
    /// The pitch B double flat.
    BDoubleFlat,

    /// The pitch F flat.
    FFlat,
    /// The pitch C flat.
    CFlat,
    /// The pitch G flat.
    GFlat,
    /// The pitch D flat.
    DFlat,
    /// The pitch A flat.
    AFlat,
    /// The pitch E flat.
    EFlat,
    /// The pitch B flat.
    BFlat,

    /// The pitch F.
    F,
    /// The pitch C.
    C,
    /// The pitch G.
    G,
    /// The pitch D.
    D,
    /// The pitch A.
    A,
    /// The pitch E.
    E,
    /// The pitch B.
    B,

    /// The pitch F sharp.
    FSharp,
    /// The pitch C sharp.
    CSharp,
    /// The pitch G sharp.
    GSharp,
    /// The pitch D sharp.
    DSharp,
    /// The pitch A sharp.
    ASharp,
    /// The pitch E sharp.
    ESharp,
    /// The pitch B sharp.
    BSharp,

    /// The pitch F double sharp.
    FDoubleSharp,
    /// The pitch C double sharp.
    CDoubleSharp,
    /// The pitch G double sharp.
    GDoubleSharp,
    /// The pitch D double sharp.
    DDoubleSharp,
    /// The pitch A double sharp.
    ADoubleSharp,
    /// The pitch E double sharp.
    EDoubleSharp,
    /// The pitch B double sharp.
    BDoubleSharp,

    /// The pitch F triple sharp.
    FTripleSharp,
    /// The pitch C triple sharp.
    CTripleSharp,
    /// The pitch G triple sharp.
    GTripleSharp,
    /// The pitch D triple sharp.
    DTripleSharp,
    /// The pitch A triple sharp.
    ATripleSharp,
    /// The pitch E triple sharp.
    ETripleSharp,
    /// The pitch B triple sharp.
    BTripleSharp,
}

// Impls.

impl HasNamedPitch for NamedPitch {
    fn named_pitch(&self) -> NamedPitch {
        *self
    }
}

impl HasLetter for NamedPitch {
    #[no_coverage]
    fn letter(&self) -> &'static str {
        match self {
            NamedPitch::FTripleFlat => "F",
            NamedPitch::CTripleFlat => "C",
            NamedPitch::GTripleFlat => "G",
            NamedPitch::DTripleFlat => "D",
            NamedPitch::ATripleFlat => "A",
            NamedPitch::ETripleFlat => "E",
            NamedPitch::BTripleFlat => "B",

            NamedPitch::FDoubleFlat => "F",
            NamedPitch::CDoubleFlat => "C",
            NamedPitch::GDoubleFlat => "G",
            NamedPitch::DDoubleFlat => "D",
            NamedPitch::ADoubleFlat => "A",
            NamedPitch::EDoubleFlat => "E",
            NamedPitch::BDoubleFlat => "B",

            NamedPitch::FFlat => "F",
            NamedPitch::CFlat => "C",
            NamedPitch::GFlat => "G",
            NamedPitch::DFlat => "D",
            NamedPitch::AFlat => "A",
            NamedPitch::EFlat => "E",
            NamedPitch::BFlat => "B",

            NamedPitch::F => "F",
            NamedPitch::C => "C",
            NamedPitch::G => "G",
            NamedPitch::D => "D",
            NamedPitch::A => "A",
            NamedPitch::E => "E",
            NamedPitch::B => "B",

            NamedPitch::FSharp => "F",
            NamedPitch::CSharp => "C",
            NamedPitch::GSharp => "G",
            NamedPitch::DSharp => "D",
            NamedPitch::ASharp => "A",
            NamedPitch::ESharp => "E",
            NamedPitch::BSharp => "B",

            NamedPitch::FDoubleSharp => "F",
            NamedPitch::CDoubleSharp => "C",
            NamedPitch::GDoubleSharp => "G",
            NamedPitch::DDoubleSharp => "D",
            NamedPitch::ADoubleSharp => "A",
            NamedPitch::EDoubleSharp => "E",
            NamedPitch::BDoubleSharp => "B",

            NamedPitch::FTripleSharp => "F",
            NamedPitch::CTripleSharp => "C",
            NamedPitch::GTripleSharp => "G",
            NamedPitch::DTripleSharp => "D",
            NamedPitch::ATripleSharp => "A",
            NamedPitch::ETripleSharp => "E",
            NamedPitch::BTripleSharp => "B",
        }
    }
}

impl HasStaticName for NamedPitch {
    #[no_coverage]
    fn static_name(&self) -> &'static str {
        match self {
            NamedPitch::FTripleFlat => "F♭𝄫",
            NamedPitch::CTripleFlat => "C♭𝄫",
            NamedPitch::GTripleFlat => "G♭𝄫",
            NamedPitch::DTripleFlat => "D♭𝄫",
            NamedPitch::ATripleFlat => "A♭𝄫",
            NamedPitch::ETripleFlat => "E♭𝄫",
            NamedPitch::BTripleFlat => "B♭𝄫",

            NamedPitch::FDoubleFlat => "F𝄫",
            NamedPitch::CDoubleFlat => "C𝄫",
            NamedPitch::GDoubleFlat => "G𝄫",
            NamedPitch::DDoubleFlat => "D𝄫",
            NamedPitch::ADoubleFlat => "A𝄫",
            NamedPitch::EDoubleFlat => "E𝄫",
            NamedPitch::BDoubleFlat => "B𝄫",

            NamedPitch::FFlat => "F♭",
            NamedPitch::CFlat => "C♭",
            NamedPitch::GFlat => "G♭",
            NamedPitch::DFlat => "D♭",
            NamedPitch::AFlat => "A♭",
            NamedPitch::EFlat => "E♭",
            NamedPitch::BFlat => "B♭",

            NamedPitch::F => "F",
            NamedPitch::C => "C",
            NamedPitch::G => "G",
            NamedPitch::D => "D",
            NamedPitch::A => "A",
            NamedPitch::E => "E",
            NamedPitch::B => "B",

            NamedPitch::FSharp => "F♯",
            NamedPitch::CSharp => "C♯",
            NamedPitch::GSharp => "G♯",
            NamedPitch::DSharp => "D♯",
            NamedPitch::ASharp => "A♯",
            NamedPitch::ESharp => "E♯",
            NamedPitch::BSharp => "B♯",

            NamedPitch::FDoubleSharp => "F𝄪",
            NamedPitch::CDoubleSharp => "C𝄪",
            NamedPitch::GDoubleSharp => "G𝄪",
            NamedPitch::DDoubleSharp => "D𝄪",
            NamedPitch::ADoubleSharp => "A𝄪",
            NamedPitch::EDoubleSharp => "E𝄪",
            NamedPitch::BDoubleSharp => "B𝄪",

            NamedPitch::FTripleSharp => "F♯𝄪",
            NamedPitch::CTripleSharp => "C♯𝄪",
            NamedPitch::GTripleSharp => "G♯𝄪",
            NamedPitch::DTripleSharp => "D♯𝄪",
            NamedPitch::ATripleSharp => "A♯𝄪",
            NamedPitch::ETripleSharp => "E♯𝄪",
            NamedPitch::BTripleSharp => "B♯𝄪",
        }
    }
}

impl HasPitch for NamedPitch {
    #[no_coverage]
    fn pitch(&self) -> Pitch {
        match self {
            NamedPitch::FTripleFlat => Pitch::D,
            NamedPitch::CTripleFlat => Pitch::A,
            NamedPitch::GTripleFlat => Pitch::E,
            NamedPitch::DTripleFlat => Pitch::B,
            NamedPitch::ATripleFlat => Pitch::GFlat,
            NamedPitch::ETripleFlat => Pitch::DFlat,
            NamedPitch::BTripleFlat => Pitch::AFlat,

            NamedPitch::FDoubleFlat => Pitch::EFlat,
            NamedPitch::CDoubleFlat => Pitch::BFlat,
            NamedPitch::GDoubleFlat => Pitch::F,
            NamedPitch::DDoubleFlat => Pitch::C,
            NamedPitch::ADoubleFlat => Pitch::G,
            NamedPitch::EDoubleFlat => Pitch::D,
            NamedPitch::BDoubleFlat => Pitch::A,

            NamedPitch::FFlat => Pitch::E,
            NamedPitch::CFlat => Pitch::B,
            NamedPitch::GFlat => Pitch::GFlat,
            NamedPitch::DFlat => Pitch::DFlat,
            NamedPitch::AFlat => Pitch::AFlat,
            NamedPitch::EFlat => Pitch::EFlat,
            NamedPitch::BFlat => Pitch::BFlat,

            NamedPitch::F => Pitch::F,
            NamedPitch::C => Pitch::C,
            NamedPitch::G => Pitch::G,
            NamedPitch::D => Pitch::D,
            NamedPitch::A => Pitch::A,
            NamedPitch::E => Pitch::E,
            NamedPitch::B => Pitch::B,

            NamedPitch::FSharp => Pitch::GFlat,
            NamedPitch::CSharp => Pitch::DFlat,
            NamedPitch::GSharp => Pitch::AFlat,
            NamedPitch::DSharp => Pitch::EFlat,
            NamedPitch::ASharp => Pitch::BFlat,
            NamedPitch::ESharp => Pitch::F,
            NamedPitch::BSharp => Pitch::C,

            NamedPitch::FDoubleSharp => Pitch::G,
            NamedPitch::CDoubleSharp => Pitch::D,
            NamedPitch::GDoubleSharp => Pitch::A,
            NamedPitch::DDoubleSharp => Pitch::E,
            NamedPitch::ADoubleSharp => Pitch::B,
            NamedPitch::EDoubleSharp => Pitch::GFlat,
            NamedPitch::BDoubleSharp => Pitch::DFlat,

            NamedPitch::FTripleSharp => Pitch::AFlat,
            NamedPitch::CTripleSharp => Pitch::EFlat,
            NamedPitch::GTripleSharp => Pitch::BFlat,
            NamedPitch::DTripleSharp => Pitch::F,
            NamedPitch::ATripleSharp => Pitch::C,
            NamedPitch::ETripleSharp => Pitch::G,
            NamedPitch::BTripleSharp => Pitch::D,
        }
    }
}

impl Add<i8> for NamedPitch {
    type Output = Self;

    fn add(self, rhs: i8) -> Self {
        let index = ALL_PITCHES.iter().position(|&p| p == self).unwrap();

        let new_index = index as i8 + rhs;

        if !(0..=49).contains(&new_index) {
            panic!("NamedPitch out of range.");
        }

        ALL_PITCHES[new_index as usize]
    }
}

impl Sub<i8> for NamedPitch {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self {
        self + (-rhs)
    }
}

impl From<Pitch> for NamedPitch {
    fn from(pitch: Pitch) -> Self {
        NamedPitch::from(&pitch)
    }
}

impl From<&Pitch> for NamedPitch {
    fn from(pitch: &Pitch) -> Self {
        match pitch {
            Pitch::C => NamedPitch::C,
            Pitch::DFlat => NamedPitch::DFlat,
            Pitch::D => NamedPitch::D,
            Pitch::EFlat => NamedPitch::EFlat,
            Pitch::E => NamedPitch::E,
            Pitch::F => NamedPitch::F,
            Pitch::GFlat => NamedPitch::GFlat,
            Pitch::G => NamedPitch::G,
            Pitch::AFlat => NamedPitch::AFlat,
            Pitch::A => NamedPitch::A,
            Pitch::BFlat => NamedPitch::BFlat,
            Pitch::B => NamedPitch::B,
        }
    }
}

// Statics.

static ALL_PITCHES: [NamedPitch; 49] = [
    NamedPitch::FTripleFlat,
    NamedPitch::CTripleFlat,
    NamedPitch::GTripleFlat,
    NamedPitch::DTripleFlat,
    NamedPitch::ATripleFlat,
    NamedPitch::ETripleFlat,
    NamedPitch::BTripleFlat,
    NamedPitch::FDoubleFlat,
    NamedPitch::CDoubleFlat,
    NamedPitch::GDoubleFlat,
    NamedPitch::DDoubleFlat,
    NamedPitch::ADoubleFlat,
    NamedPitch::EDoubleFlat,
    NamedPitch::BDoubleFlat,
    NamedPitch::FFlat,
    NamedPitch::CFlat,
    NamedPitch::GFlat,
    NamedPitch::DFlat,
    NamedPitch::AFlat,
    NamedPitch::EFlat,
    NamedPitch::BFlat,
    NamedPitch::F,
    NamedPitch::C,
    NamedPitch::G,
    NamedPitch::D,
    NamedPitch::A,
    NamedPitch::E,
    NamedPitch::B,
    NamedPitch::FSharp,
    NamedPitch::CSharp,
    NamedPitch::GSharp,
    NamedPitch::DSharp,
    NamedPitch::ASharp,
    NamedPitch::ESharp,
    NamedPitch::BSharp,
    NamedPitch::FDoubleSharp,
    NamedPitch::CDoubleSharp,
    NamedPitch::GDoubleSharp,
    NamedPitch::DDoubleSharp,
    NamedPitch::ADoubleSharp,
    NamedPitch::EDoubleSharp,
    NamedPitch::BDoubleSharp,
    NamedPitch::FTripleSharp,
    NamedPitch::CTripleSharp,
    NamedPitch::GTripleSharp,
    NamedPitch::DTripleSharp,
    NamedPitch::ATripleSharp,
    NamedPitch::ETripleSharp,
    NamedPitch::BTripleSharp,
];

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::note::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[should_panic]
    fn test_improper_add() {
        let _ = C.named_pitch() + 50;
    }

    #[test]
    fn test_properties() {
        assert_eq!(NamedPitch::A.named_pitch(), NamedPitch::A);
    }

    #[test]
    fn test_pitch_conversion() {
        assert_eq!(NamedPitch::from(Pitch::C), NamedPitch::C);
        assert_eq!(NamedPitch::from(&Pitch::C), NamedPitch::C);
        assert_eq!(NamedPitch::from(Pitch::DFlat), NamedPitch::DFlat);
        assert_eq!(NamedPitch::from(&Pitch::DFlat), NamedPitch::DFlat);
        assert_eq!(NamedPitch::from(Pitch::D), NamedPitch::D);
        assert_eq!(NamedPitch::from(&Pitch::D), NamedPitch::D);
        assert_eq!(NamedPitch::from(Pitch::EFlat), NamedPitch::EFlat);
        assert_eq!(NamedPitch::from(&Pitch::EFlat), NamedPitch::EFlat);
        assert_eq!(NamedPitch::from(Pitch::E), NamedPitch::E);
        assert_eq!(NamedPitch::from(&Pitch::E), NamedPitch::E);
        assert_eq!(NamedPitch::from(Pitch::F), NamedPitch::F);
        assert_eq!(NamedPitch::from(&Pitch::F), NamedPitch::F);
        assert_eq!(NamedPitch::from(Pitch::GFlat), NamedPitch::GFlat);
        assert_eq!(NamedPitch::from(&Pitch::GFlat), NamedPitch::GFlat);
        assert_eq!(NamedPitch::from(Pitch::G), NamedPitch::G);
        assert_eq!(NamedPitch::from(&Pitch::G), NamedPitch::G);
        assert_eq!(NamedPitch::from(Pitch::AFlat), NamedPitch::AFlat);
        assert_eq!(NamedPitch::from(&Pitch::AFlat), NamedPitch::AFlat);
        assert_eq!(NamedPitch::from(Pitch::A), NamedPitch::A);
        assert_eq!(NamedPitch::from(&Pitch::A), NamedPitch::A);
        assert_eq!(NamedPitch::from(Pitch::BFlat), NamedPitch::BFlat);
        assert_eq!(NamedPitch::from(&Pitch::BFlat), NamedPitch::BFlat);
        assert_eq!(NamedPitch::from(Pitch::B), NamedPitch::B);
        assert_eq!(NamedPitch::from(&Pitch::B), NamedPitch::B);
    }
}
