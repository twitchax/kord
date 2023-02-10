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
        "A#" => note::ASharp,
        "A♯" => note::ASharp,
        "A##" => note::ADoubleSharp,
        "A𝄪" => note::ADoubleSharp,
        "Ab" => note::AFlat,
        "A♭" => note::AFlat,
        "Abb" => note::ADoubleFlat,
        "A𝄫" => note::ADoubleFlat,
        "B" => note::B,
        "B#" => note::BSharp,
        "B♯" => note::BSharp,
        "B##" => note::BDoubleSharp,
        "B𝄪" => note::BDoubleSharp,
        "Bb" => note::BFlat,
        "B♭" => note::BFlat,
        "Bbb" => note::BDoubleFlat,
        "B𝄫" => note::BDoubleFlat,
        "C" => note::C,
        "C#" => note::CSharp,
        "C♯" => note::CSharp,
        "C##" => note::CDoubleSharp,
        "C𝄪" => note::CDoubleSharp,
        "Cb" => note::CFlat,
        "C♭" => note::CFlat,
        "Cbb" => note::CDoubleFlat,
        "C𝄫" => note::CDoubleFlat,
        "D" => note::D,
        "D#" => note::DSharp,
        "D♯" => note::DSharp,
        "D##" => note::DDoubleSharp,
        "D𝄪" => note::DDoubleSharp,
        "Db" => note::DFlat,
        "D♭" => note::DFlat,
        "Dbb" => note::DDoubleFlat,
        "D𝄫" => note::DDoubleFlat,
        "E" => note::E,
        "E#" => note::ESharp,
        "E♯" => note::ESharp,
        "E##" => note::EDoubleSharp,
        "E𝄪" => note::EDoubleSharp,
        "Eb" => note::EFlat,
        "E♭" => note::EFlat,
        "Ebb" => note::EDoubleFlat,
        "E𝄫" => note::EDoubleFlat,
        "F" => note::F,
        "F#" => note::FSharp,
        "F♯" => note::FSharp,
        "F##" => note::FDoubleSharp,
        "F𝄪" => note::FDoubleSharp,
        "Fb" => note::FFlat,
        "F♭" => note::FFlat,
        "Fbb" => note::FDoubleFlat,
        "F𝄫" => note::FDoubleFlat,
        "G" => note::G,
        "G#" => note::GSharp,
        "G♯" => note::GSharp,
        "G##" => note::GDoubleSharp,
        "G𝄪" => note::GDoubleSharp,
        "Gb" => note::GFlat,
        "G♭" => note::GFlat,
        "Gbb" => note::GDoubleFlat,
        "G𝄫" => note::GDoubleFlat,
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
