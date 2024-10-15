//! A module for working with chord modifiers.

use std::sync::LazyLock;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::base::HasStaticName;

// Traits.

/// A trait for types that have a static name.
pub trait HasIsDominant {
    /// Returns whether the type (usually the modifier enum) is dominant.
    fn is_dominant(&self) -> bool;
}

// Enum.

/// An enum representing the degree of a dominant chord.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    /// Flat 9 modifier.
    Flat9,
    /// Sharp 9 modifier.
    Sharp9,

    /// Sharp 11 modifier.
    Sharp11,

    /// Diminished modifier.
    Diminished,
}

/// An enum representing the extension of a chord.
///
/// Extensions are not really "special" in the sense that they do not change how the
/// chord is interpreted by the system.  E.g., an `add2` just adds a 2 to the chord,
/// and the chord is still interpreted as a major chord.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = KordExtension))]
pub enum Extension {
    /// Sus2 extension.
    Sus2,
    /// Sus4 extension.
    Sus4,

    /// Flat 11 extension.
    Flat11,

    /// Flat 13 extension.
    Flat13,
    /// Sharp 13 extension.
    Sharp13,

    /// Add2 extension.
    Add2,
    /// Add4 extension.
    Add4,
    /// Add6 extension.
    Add6,

    /// Add9 extension.
    Add9,
    /// Add11 extension.
    Add11,
    /// Add13 extension.
    Add13,
}

// Impls.

impl HasIsDominant for Modifier {
    fn is_dominant(&self) -> bool {
        matches!(self, Modifier::Dominant(_))
    }
}

impl HasStaticName for Degree {
    #[coverage(off)]
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
    #[coverage(off)]
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
    #[coverage(off)]
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

// Helpers.

/// Returns the sets of modifiers that have associated known chords.
pub fn known_modifier_sets() -> &'static [Vec<Modifier>] {
    KNOWN_MODIFIER_SETS.as_ref()
}

/// Returns the sets of modifiers that can be used as one off extensions.
pub fn one_off_modifier_sets() -> &'static [Vec<Modifier>] {
    ONE_OFF_MODIFIER_SETS.as_ref()
}

/// Returns the sets of extensions that are useful to test when guessing chords.
pub fn likely_extension_sets() -> &'static [Vec<Extension>] {
    LIKELY_EXTENSION_SETS.as_ref()
}

// Statics.

static KNOWN_MODIFIER_SETS: LazyLock<[Vec<Modifier>; 35]> = LazyLock::new(|| {
    [
        vec![],
        vec![Modifier::Minor],
        vec![Modifier::Major7],
        vec![Modifier::Dominant(Degree::Seven)],
        vec![Modifier::Dominant(Degree::Nine)],
        vec![Modifier::Dominant(Degree::Eleven)],
        vec![Modifier::Dominant(Degree::Thirteen)],
        vec![Modifier::Minor, Modifier::Major7],
        vec![Modifier::Minor, Modifier::Dominant(Degree::Seven)],
        vec![Modifier::Minor, Modifier::Dominant(Degree::Nine)],
        vec![Modifier::Minor, Modifier::Dominant(Degree::Eleven)],
        vec![Modifier::Minor, Modifier::Dominant(Degree::Thirteen)],
        vec![Modifier::Sharp11, Modifier::Dominant(Degree::Seven)],
        vec![Modifier::Sharp11, Modifier::Dominant(Degree::Nine)],
        vec![Modifier::Sharp11, Modifier::Dominant(Degree::Eleven)],
        vec![Modifier::Sharp11, Modifier::Dominant(Degree::Thirteen)],
        vec![Modifier::Augmented5],
        vec![Modifier::Augmented5, Modifier::Major7],
        vec![Modifier::Augmented5, Modifier::Dominant(Degree::Seven)],
        vec![Modifier::Augmented5, Modifier::Dominant(Degree::Nine)],
        vec![Modifier::Augmented5, Modifier::Dominant(Degree::Eleven)],
        vec![Modifier::Augmented5, Modifier::Dominant(Degree::Thirteen)],
        vec![Modifier::Minor, Modifier::Flat5, Modifier::Dominant(Degree::Seven)],
        vec![Modifier::Minor, Modifier::Flat5, Modifier::Dominant(Degree::Nine)],
        vec![Modifier::Minor, Modifier::Flat5, Modifier::Dominant(Degree::Eleven)],
        vec![Modifier::Minor, Modifier::Flat5, Modifier::Dominant(Degree::Thirteen)],
        vec![Modifier::Diminished],
        vec![Modifier::Flat9, Modifier::Dominant(Degree::Seven)],
        vec![Modifier::Flat9, Modifier::Dominant(Degree::Nine)],
        vec![Modifier::Flat9, Modifier::Dominant(Degree::Eleven)],
        vec![Modifier::Flat9, Modifier::Dominant(Degree::Thirteen)],
        vec![Modifier::Sharp9, Modifier::Dominant(Degree::Seven)],
        vec![Modifier::Sharp9, Modifier::Dominant(Degree::Nine)],
        vec![Modifier::Sharp9, Modifier::Dominant(Degree::Eleven)],
        vec![Modifier::Sharp9, Modifier::Dominant(Degree::Thirteen)],
    ]
});

static ONE_OFF_MODIFIER_SETS: LazyLock<[Vec<Modifier>; 6]> = LazyLock::new(|| {
    [
        vec![],
        vec![Modifier::Sharp11],
        vec![Modifier::Augmented5],
        vec![Modifier::Flat5],
        vec![Modifier::Flat9],
        vec![Modifier::Sharp9],
    ]
});

static LIKELY_EXTENSION_SETS: LazyLock<[Vec<Extension>; 12]> = LazyLock::new(|| {
    [
        vec![],
        vec![Extension::Sus2],
        vec![Extension::Sus4],
        vec![Extension::Add2],
        vec![Extension::Add4],
        vec![Extension::Add6],
        vec![Extension::Add9],
        vec![Extension::Add11],
        vec![Extension::Add13],
        vec![Extension::Flat11],
        vec![Extension::Flat13],
        vec![Extension::Sharp13],
    ]
});
