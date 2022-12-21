use pest_derive::Parser;

use crate::{base::Res, note::{self, Note}};

/// A parser for chord symbols.
/// 
/// This is built from a PEG grammar defined in `chord.pest`.
#[derive(Parser)]
#[grammar = "../chord.pest"]
pub struct ChordParser;

/// Parses a [`Note`] [`str`] into a [`Note`].
#[no_coverage]
pub fn note_str_to_note(note_str: &str) -> Res<Note> {
    let chord = match note_str {
        "A" => note::A,
        "A#" => note::ASharp,
        "A♯" => note::ASharp,
        "Ab" => note::AFlat,
        "A♭" => note::AFlat,
        "B" => note::B,
        "Bb" => note::BFlat,
        "B♭" => note::BFlat,
        "C" => note::C,
        "C#" => note::CSharp,
        "C♯" => note::CSharp,
        "D" => note::D,
        "D#" => note::DSharp,
        "D♯" => note::DSharp,
        "Db" => note::DFlat,
        "D♭" => note::DFlat,
        "E" => note::E,
        "Eb" => note::EFlat,
        "E♭" => note::EFlat,
        "F" => note::F,
        "F#" => note::FSharp,
        "F♯" => note::FSharp,
        "G" => note::G,
        "G#" => note::GSharp,
        "G♯" => note::GSharp,
        "Gb" => note::GFlat,
        "G♭" => note::GFlat,
        _ => return Err(crate::base::Err::msg("Please use fairly standard notes (e.g., don't use `C♭`).")),
    };

    Ok(chord)
}