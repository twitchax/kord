//! A module for working with the parser for chord symbols.

use pest_derive::Parser;

use crate::core::{
    base::Res,
    mode_kind::ModeKind,
    note::{self, Note},
    octave::Octave,
    scale_kind::ScaleKind,
};

/// A parser for chord symbols.
///
/// This is built from a PEG grammar defined in `chord.pest`.
#[derive(Parser)]
#[grammar = "../chord.pest"]
pub struct ChordParser;

// Helpers.

/// Parses a [`Note`] [`str`] into a [`Note`].
#[coverage(off)]
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
#[coverage(off)]
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

/// Parses a mode name string into a [`ModeKind`].
#[coverage(off)]
pub fn mode_name_str_to_mode_kind(mode_str: &str) -> Res<ModeKind> {
    let normalized = mode_str.to_lowercase()
        .replace("♮", "natural")
        .replace("♯", "sharp")
        .replace("#", "sharp")
        .replace("♭", "flat")
        .replace("b", "flat")
        .replace(" ", "");
    
    let mode = match normalized.as_str() {
        // Major scale modes
        "ionian" => ModeKind::Ionian,
        "dorian" => ModeKind::Dorian,
        "phrygian" => ModeKind::Phrygian,
        "lydian" => ModeKind::Lydian,
        "mixolydian" => ModeKind::Mixolydian,
        "aeolian" => ModeKind::Aeolian,
        "locrian" => ModeKind::Locrian,
        
        // Harmonic minor modes
        "locriannatural6" | "locriannat6" => ModeKind::LocrianNatural6,
        "ioniansharp5" | "ionianaugmented" | "majorsharp5" | "augmentedmajor" => ModeKind::IonianSharp5,
        "doriansharp4" => ModeKind::DorianSharp4,
        "phrygiandominant" | "spanishphrygian" | "phrygianmajor" => ModeKind::PhrygianDominant,
        "lydiansharp2" => ModeKind::LydianSharp2,
        "ultralocrian" => ModeKind::Ultralocrian,
        
        // Melodic minor modes
        "dorianflat2" | "phrygiannatural6" | "phrygiannat6" => ModeKind::DorianFlat2,
        "lydianaugmented" | "lydiansharp5" => ModeKind::LydianAugmented,
        "lydiandominant" | "lydianflat7" | "mixolydiansharp4" | "acoustic" | "acousticscale" => ModeKind::LydianDominant,
        "mixolydianflat6" | "aeoliandominant" => ModeKind::MixolydianFlat6,
        "locriannatural2" | "locriannat2" | "locriansharp2" | "half-diminished" | "half-diminishednatural2" | "half-diminishednat2" => ModeKind::LocrianNatural2,
        "altered" | "alteredscale" | "superlocrian" => ModeKind::Altered,
        
        _ => return Err(crate::core::base::Err::msg("Unknown mode name")),
    };

    Ok(mode)
}

/// Parses a scale name string into a [`ScaleKind`].
#[coverage(off)]
pub fn scale_name_str_to_scale_kind(scale_str: &str) -> Res<ScaleKind> {
    let normalized = scale_str.to_lowercase().replace(" ", "");
    let scale = match normalized.as_str() {
        "major" => ScaleKind::Major,
        "naturalminor" => ScaleKind::NaturalMinor,
        "harmonicminor" => ScaleKind::HarmonicMinor,
        "melodicminor" => ScaleKind::MelodicMinor,
        "wholetone" => ScaleKind::WholeTone,
        "chromatic" => ScaleKind::Chromatic,
        "diminished(whole-half)" => ScaleKind::DiminishedWholeHalf,
        "diminished(half-whole)" => ScaleKind::DiminishedHalfWhole,
        _ => return Err(crate::core::base::Err::msg("Unknown scale name")),
    };

    Ok(scale)
}
