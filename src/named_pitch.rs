// Traits.

use crate::{pitch::{HasPitch, Pitch}, base::HasStaticName};

pub trait HasNamedPitch {
    fn named_pitch(&self) -> NamedPitch;
}

pub trait HasLetter {
    fn letter(&self) -> &'static str;
}

// Enum.

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum NamedPitch {
    ATripleSharp
    BSharp,
    C,
    DDoubleFlat,
    
    BDoubleSharp
    CSharp,
    DFlat,
    ETripleFlat

    //BTripleSharp
    CDoubleSharp,
    D,
    EDoubleFlat,
    FTripleFlat

    CTripleSharp
    DSharp,
    EFlat,
    FDoubleFlat

    DDoubleSharp,
    E,
    FFlat,
    GTripleFlat

    DTripleSharp
    ESharp,
    F,
    GDoubleFlat,

    EDoubleSharp
    FSharp,
    GFlat,
    ATripleFlat

    ETripleSharp
    FDoubleSharp,
    G,
    ADoubleFlat,

    FTripleSharp,
    GSharp,
    AFlat,
    BTripleFlat

    GDoubleSharp,
    A,
    BDoubleFlat,
    CTripleFlat,

    GTripleSharp,
    ASharp,
    BFlat,
    CDoubleFlat,

    ADoubleSharp,
    B,
    CFlat,
    DTripleFlat,
}

// Impls.

impl HasNamedPitch for NamedPitch {
    fn named_pitch(&self) -> NamedPitch {
        *self
    }
}

impl HasLetter for NamedPitch {
    fn letter(&self) -> &'static str {
        match self {
            NamedPitch::BSharp => "B",
            NamedPitch::C => "C",
            NamedPitch::DDoubleFlat => "D",
            
            NamedPitch::CSharp => "C",
            NamedPitch::DFlat => "D",

            NamedPitch::CDoubleSharp => "C",
            NamedPitch::D => "D",
            NamedPitch::EDoubleFlat => "E",

            NamedPitch::DSharp => "D",
            NamedPitch::EFlat => "E",

            NamedPitch::DDoubleSharp => "D",
            NamedPitch::E => "E",
            NamedPitch::FFlat => "F",

            NamedPitch::ESharp => "E",
            NamedPitch::F => "F",
            NamedPitch::GDoubleFlat => "G",

            NamedPitch::FSharp => "F",
            NamedPitch::GFlat => "G",

            NamedPitch::FDoubleSharp => "F",
            NamedPitch::G => "G",
            NamedPitch::ADoubleFlat => "A",

            NamedPitch::GSharp => "G",
            NamedPitch::AFlat => "A",

            NamedPitch::GDoubleSharp => "G",
            NamedPitch::A => "A",
            NamedPitch::BDoubleFlat => "B",

            NamedPitch::ASharp => "A",
            NamedPitch::BFlat => "B",

            NamedPitch::ADoubleSharp => "A",
            NamedPitch::B => "B",
            NamedPitch::CFlat => "C",
        }
    }
}

impl HasStaticName for NamedPitch {
    fn static_name(&self) -> &'static str {
        match self {
            NamedPitch::BSharp => "B♯",
            NamedPitch::C => "C",
            NamedPitch::DDoubleFlat => "D𝄫",
            
            NamedPitch::CSharp => "C♯",
            NamedPitch::DFlat => "D♭",

            NamedPitch::CDoubleSharp => "C𝄪",
            NamedPitch::D => "D",
            NamedPitch::EDoubleFlat => "E𝄫",

            NamedPitch::DSharp => "D♯",
            NamedPitch::EFlat => "E♭",

            NamedPitch::DDoubleSharp => "D𝄪",
            NamedPitch::E => "E",
            NamedPitch::FFlat => "F♭",

            NamedPitch::ESharp => "E♯",
            NamedPitch::F => "F",
            NamedPitch::GDoubleFlat => "G𝄫",

            NamedPitch::FSharp => "F♯",
            NamedPitch::GFlat => "G♭",

            NamedPitch::FDoubleSharp => "F𝄪",
            NamedPitch::G => "G",
            NamedPitch::ADoubleFlat => "A𝄫",

            NamedPitch::GSharp => "G♯",
            NamedPitch::AFlat => "A♭",

            NamedPitch::GDoubleSharp => "G𝄪",
            NamedPitch::A => "A",
            NamedPitch::BDoubleFlat => "B𝄫",

            NamedPitch::ASharp => "A♯",
            NamedPitch::BFlat => "B♭",

            NamedPitch::ADoubleSharp => "A𝄪",
            NamedPitch::B => "B",
            NamedPitch::CFlat => "C♭",
        }
    }
}

impl HasPitch for NamedPitch {
    fn pitch(&self) -> Pitch {
        match self {
            NamedPitch::BSharp => Pitch::C,
            NamedPitch::C => Pitch::C,
            NamedPitch::DDoubleFlat => Pitch::C,

            NamedPitch::CSharp => Pitch::CSharp,
            NamedPitch::DFlat => Pitch::CSharp,

            NamedPitch::CDoubleSharp => Pitch::D,
            NamedPitch::D => Pitch::D,
            NamedPitch::EDoubleFlat => Pitch::D,

            NamedPitch::DSharp => Pitch::DSharp,
            NamedPitch::EFlat => Pitch::DSharp,

            NamedPitch::DDoubleSharp => Pitch::E,
            NamedPitch::E => Pitch::E,
            NamedPitch::FFlat => Pitch::E,

            NamedPitch::ESharp => Pitch::F,
            NamedPitch::F => Pitch::F,
            NamedPitch::GDoubleFlat => Pitch::F,

            NamedPitch::FSharp => Pitch::FSharp,
            NamedPitch::GFlat => Pitch::FSharp,

            NamedPitch::FDoubleSharp => Pitch::G,
            NamedPitch::G => Pitch::G,
            NamedPitch::ADoubleFlat => Pitch::G,

            NamedPitch::GSharp => Pitch::GSharp,
            NamedPitch::AFlat => Pitch::GSharp,

            NamedPitch::GDoubleSharp => Pitch::A,
            NamedPitch::A => Pitch::A,
            NamedPitch::BDoubleFlat => Pitch::A,

            NamedPitch::ASharp => Pitch::ASharp,
            NamedPitch::BFlat => Pitch::ASharp,

            NamedPitch::ADoubleSharp => Pitch::B,
            NamedPitch::B => Pitch::B,
            NamedPitch::CFlat => Pitch::B,
        }
    }
}

impl NamedPitch {
    pub fn iter(&self) -> NamedPitchIter {
        NamedPitchIter {
            current: *self,
            octaves: 0,
        }
    }
}

// Iterators.

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
pub struct NamedPitchIter {
    pub current: NamedPitch,
    pub octaves: i8,
}

impl Iterator for NamedPitchIter {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: This is safe because every value of `NamedPitch` is present in `ALL_PITCHES`.
        let current = ALL_PITCHES.iter().position(|&p| p == self.current).unwrap();
        
        if current == ALL_PITCHES.len() - 1 {
            self.octaves += 1;
            self.current = ALL_PITCHES[0];
        } else {
            self.current = ALL_PITCHES[current + 1];
        }

        Some(*self)
    }
}

// Statics.

static ALL_PITCHES: [NamedPitch; 31] = [
    NamedPitch::BSharp,
    NamedPitch::C,
    NamedPitch::DDoubleFlat,
    NamedPitch::CSharp,
    NamedPitch::DFlat,
    NamedPitch::CDoubleSharp,
    NamedPitch::D,
    NamedPitch::EDoubleFlat,
    NamedPitch::DSharp,
    NamedPitch::EFlat,
    NamedPitch::DDoubleSharp,
    NamedPitch::E,
    NamedPitch::FFlat,
    NamedPitch::ESharp,
    NamedPitch::F,
    NamedPitch::GDoubleFlat,
    NamedPitch::FSharp,
    NamedPitch::GFlat,
    NamedPitch::FDoubleSharp,
    NamedPitch::G,
    NamedPitch::ADoubleFlat,
    NamedPitch::GSharp,
    NamedPitch::AFlat,
    NamedPitch::GDoubleSharp,
    NamedPitch::A,
    NamedPitch::BDoubleFlat,
    NamedPitch::ASharp,
    NamedPitch::BFlat,
    NamedPitch::ADoubleSharp,
    NamedPitch::B,
    NamedPitch::CFlat,
];