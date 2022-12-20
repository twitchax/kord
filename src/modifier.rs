use crate::base::HasStaticName;

// Traits.

/// A trait for types that have a static name.
pub trait HasIsDominant {
    /// Returns whether the type (usually the modifier enum) is dominant.
    fn is_dominant(&self) -> bool;
}

// Enum.

/// An enum representing the degree of a dominant chord.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum Degree {
    /// Seventh degree.
    Seven,
    /// Ninth degree.
    Nine,
    /// Eleventh degree.
    Eleven,
    /// Thirteenth degree.
    Thirteen,
}

/// An enum representing the modifier of a chord.
/// 
/// Modifiers are "special extensions" that essentially have the capacity to _change_ 
/// how the chord is interpreted by the system.  E.g., a dominant flat 9 chord is not
/// _just_ a dominant chord with a flat 9 extension, but rather a chord that is
/// represented by an entirely specific scale (half/whole/half diminished).
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum Modifier {
    /// Minor modifier.
    Minor,

    /// Flat 5 modifier.
    Flat5,
    /// Sharp 5 modifier.
    Augmented5,

    /// Major 7 modifier.
    Major7,
    /// Dominant modifier with degree.
    Dominant(Degree),

    // Flat 9 modifier.
    Flat9,
    // Sharp 9 modifier.
    Sharp9,

    // Sharp 11 modifier.
    Sharp11,

    // Diminished modifier.
    Diminished,
}

/// An enum representing the extension of a chord.
/// 
/// Extensions are not really "special" in the sense that they do not change how the
/// chord is interpreted by the system.  E.g., an `add2` just adds a 2 to the chord,
/// and the chord is still interpreted as a major chord.
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

impl HasIsDominant for Modifier {
    fn is_dominant(&self) -> bool {
        matches!(self, Modifier::Dominant(_))
    }
}

impl HasStaticName for Degree {
    fn static_name(&self) -> &'static str {
        match self {
            Degree::Seven => "7",
            Degree::Nine => "9",
            Degree::Eleven => "11",
            Degree::Thirteen => "13",
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