

// Enum.

use crate::base::HasStaticName;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum Dominant {
    Seven,
    Nine,
    Eleven,
    Thirteen,
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum Modifier {
    Minor,

    Flat5,
    Augmented5,

    Major7,
    Dominant(Dominant),

    Flat9,
    Sharp9,

    Sharp11,

    Diminished,
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum Extension {
    Sus2,
    Sus4,

    Flat11,

    Flat13,
    Sharp13,

    Add2,
    Add4,
    Add6,

    Add9,
    Add11,
    Add13,
}

// Impls.

impl HasStaticName for Dominant {
    fn static_name(&self) -> &'static str {
        match self {
            Dominant::Seven => "7",
            Dominant::Nine => "9",
            Dominant::Eleven => "11",
            Dominant::Thirteen => "13",
        }
    }
}

impl HasStaticName for Modifier {
    fn static_name(&self) -> &'static str {
        match self {
            Modifier::Minor => "m",

            Modifier::Flat5 => "♭5",
            Modifier::Augmented5 => "+",

            Modifier::Major7 => "maj7",
            Modifier::Dominant(dominant) => dominant.static_name(),

            Modifier::Flat9 => "♭9",
            Modifier::Sharp9 => "♯9",

            Modifier::Sharp11 => "♯11",

            Modifier::Diminished => "°",
        }
    }
}

impl HasStaticName for Extension {
    fn static_name(&self) -> &'static str {
        match self {
            Extension::Sus2 => "sus2",
            Extension::Sus4 => "sus4",

            Extension::Flat11 => "♭11",

            Extension::Flat13 => "♭13",
            Extension::Sharp13 => "♯13",

            Extension::Add2 => "add2",
            Extension::Add4 => "add4",
            Extension::Add6 => "add6",

            Extension::Add9 => "add9",
            Extension::Add11 => "add11",
            Extension::Add13 => "add13",
        }
    }
}