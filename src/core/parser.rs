//! A module for working with the parser for chord symbols.

use pest_derive::Parser;

use crate::core::{
    base::Res,
    note::{self, Note},
    octave::Octave,
};

/// A parser for chord symbols.
///
/// This is built from a PEG grammar defined in `chord.pest`.
#[derive(Parser)]
#[grammar = "../chord.pest"]
pub struct ChordParser;

// Helpers.

/// Parses a [`Note`] [`str`] into a [`Note`].
#[no_coverage]
pub fn note_str_to_note(note_str: &str) -> Res<Note> {
    let chord = match note_str {
        "A" => note::A,
        "A#" | "A♯" => note::ASharp,
        "A##" | "A𝄪" => note::ADoubleSharp,
        "Ab" | "A♭" => note::AFlat,
        "Abb" | "A𝄫" => note::ADoubleFlat,
        "B" => note::B,
        "B#" | "B♯" => note::BSharp,
        "B##" | "B𝄪" => note::BDoubleSharp,
        "Bb" | "B♭" => note::BFlat,
        "Bbb" | "B𝄫" => note::BDoubleFlat,
        "C" => note::C,
        "C#" | "C♯" => note::CSharp,
        "C##" | "C𝄪" => note::CDoubleSharp,
        "Cb" | "C♭" => note::CFlat,
        "Cbb" | "C𝄫" => note::CDoubleFlat,
        "D" => note::D,
        "D#" | "D♯" => note::DSharp,
        "D##" | "D𝄪" => note::DDoubleSharp,
        "Db" | "D♭" => note::DFlat,
        "Dbb" | "D𝄫" => note::DDoubleFlat,
        "E" => note::E,
        "E#" | "E♯" => note::ESharp,
        "E##" | "E𝄪" => note::EDoubleSharp,
        "Eb" | "E♭" => note::EFlat,
        "Ebb" | "E𝄫" => note::EDoubleFlat,
        "F" => note::F,
        "F#" | "F♯" => note::FSharp,
        "F##" | "F𝄪" => note::FDoubleSharp,
        "Fb" | "F♭" => note::FFlat,
        "Fbb" | "F𝄫" => note::FDoubleFlat,
        "G" => note::G,
        "G#" | "G♯" => note::GSharp,
        "G##" | "G𝄪" => note::GDoubleSharp,
        "Gb" | "G♭" => note::GFlat,
        "Gbb" | "G𝄫" => note::GDoubleFlat,
        _ => return Err(crate::core::base::Err::msg("Please use fairly standard notes (e.g., don't use triple sharps / flats).")),
    };

    Ok(chord)
}

/// Parses an [`Octave`] [`str`] into an [`Octave`].
#[no_coverage]
pub fn octave_str_to_octave(note_str: &str) -> Res<Octave> {
    let octave = match note_str {
        "0" => Octave::Zero,
        "1" => Octave::One,
        "2" => Octave::Two,
        "3" => Octave::Three,
        "4" => Octave::Four,
        "5" => Octave::Five,
        "6" => Octave::Six,
        "7" => Octave::Seven,
        "8" => Octave::Eight,
        "9" => Octave::Nine,
        _ => return Err(crate::core::base::Err::msg("Please use a valid octave (0 - 9).")),
    };

    Ok(octave)
}
