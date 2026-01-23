//! A module that contains the [`Chord`] struct and related traits.

use std::{cmp::Ordering, collections::HashSet, fmt::Display};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use pest::Parser;

use crate::core::{
    base::{HasDescription, HasName, HasPreciseName, HasStaticName, Parsable, Res},
    interval::Interval,
    known_chord::{HasRelativeChord, HasRelativeScale, HasScaleCandidates, IntervalCandidate, IntervalCollectionKind, KnownChord, ScaleCandidate},
    modifier::{known_modifier_sets, likely_extension_sets, one_off_modifier_sets, Degree, Extension, HasIsDominant, Modifier},
    named_pitch::HasNamedPitch,
    note::{CZero, Note, NoteRecreator},
    octave::{HasOctave, Octave},
    parser::{note_str_to_note, octave_str_to_octave, ChordParser, Rule},
    pitch::{HasFrequency, Pitch},
};

// Traits.

/// A trait that represents a type that has a root note.
pub trait HasRoot {
    /// Returns the root note of the implementor (most likely a [`Chord`]).
    fn root(&self) -> Note;
}

/// A trait that represents a type that has a slash note.
pub trait HasSlash {
    /// Returns the slash note of the implementor (most likely a [`Chord`]).
    fn slash(&self) -> Note;
}

/// A trait that represents a type that has modifiers.
pub trait HasModifiers {
    /// Returns the modifiers of the implementor (most likely a [`Chord`]).
    fn modifiers(&self) -> &HashSet<Modifier>;
}

/// A trait that represents a type that has extensions.
pub trait HasExtensions {
    /// Returns the extensions of the implementor (most likely a [`Chord`]).
    fn extensions(&self) -> &HashSet<Extension>;
}

/// A trait that represents a type that has an inversion.
pub trait HasInversion {
    /// Returns the inversion of the implementor (most likely a [`Chord`]).
    fn inversion(&self) -> u8;
}

/// A trait that represents a type that has "crunchiness".
pub trait HasIsCrunchy {
    /// Returns the "crunchiness" of the implementor (most likely a [`Chord`]).
    fn is_crunchy(&self) -> bool;
}

/// A trait that represents a type that has an octave.
pub trait HasKnownChord {
    /// Returns the known chord of the implementor (most likely a [`Chord`]).
    fn known_chord(&self) -> KnownChord;
}

/// A trait that represents a type that has a chord.
pub trait HasScale {
    /// Returns the scale of the implementor (most likely a [`Chord`]).
    fn scale(&self) -> Vec<Note>;
}

/// A trait that represents a type that has a chord.
pub trait HasChord {
    /// Returns the chord of the implementor (most likely a [`Chord`]).
    fn chord(&self) -> Vec<Note>;
}

/// A trait that represents a type that has a chord.
///
/// These methods all take ownership of the existing implementor (usually a [`Chord`]),
/// and then return a new chord.  This can be circumvented by using an explicit `clone()`.
/// E.g., `chord.clone().minor()`.
pub trait Chordable {
    /// Adds a modifier to the implementor (most likely a [`Chord`]), and returns a new chord.
    fn with_modifier(self, modifier: Modifier) -> Chord;
    /// Adds modifiers to the implementor (most likely a [`Chord`]), and returns a new chord.
    fn with_modifiers(self, modifiers: &[Modifier]) -> Chord;
    /// Adds an extension to the implementor (most likely a [`Chord`]), and returns a new chord.
    fn with_extension(self, extension: Extension) -> Chord;
    /// Adds extensions to the implementor (most likely a [`Chord`]), and returns a new chord.
    fn with_extensions(self, extensions: &[Extension]) -> Chord;
    /// Sets the inversion number of the implementor (most likely a [`Chord`]), and returns a new chord.
    fn with_inversion(self, inversion: u8) -> Chord;
    /// Sets the slash note of the implementor (most likely a [`Chord`]), and returns a new chord.
    fn with_slash(self, slash: Note) -> Chord;
    /// Sets the octave of the implementor (most likely the root note of a chord), and returns a new chord.
    fn with_octave(self, octave: Octave) -> Chord;
    /// Sets whether or not the implementor (most likely a [`Chord`]) is crunchy.
    fn with_crunchy(self, is_crunchy: bool) -> Chord;

    // Modifiers.

    /// Returns a new chord with a minor modifier on the implementor (most likely a [`Chord`]).
    fn minor(self) -> Chord;

    /// Returns a new chord with a flat 5 modifier on the implementor (most likely a [`Chord`]).
    fn flat5(self) -> Chord;
    /// Returns a new chord with a flat 5 modifier on the implementor (most likely a [`Chord`]).
    fn flat_five(self) -> Chord;

    /// Returns a new chord with a sharp 5 modifier on the implementor (most likely a [`Chord`]).
    fn augmented(self) -> Chord;
    /// Returns a new chord with a sharp 5 modifier on the implementor (most likely a [`Chord`]).
    fn aug(self) -> Chord;

    /// Returns a new chord with a major 7 modifier on the implementor (most likely a [`Chord`]).
    fn major7(self) -> Chord;
    /// Returns a new chord with a major 7 modifier on the implementor (most likely a [`Chord`]).
    fn major_seven(self) -> Chord;
    /// Returns a new chord with a major 7 modifier on the implementor (most likely a [`Chord`]).
    fn maj7(self) -> Chord;

    /// Returns a new chord with a dominant 7 modifier on the implementor (most likely a [`Chord`]).
    fn dominant7(self) -> Chord;
    /// Returns a new chord with a dominant 7 modifier on the implementor (most likely a [`Chord`]).
    fn seven(self) -> Chord;
    /// Returns a new chord with a dominant 9 modifier on the implementor (most likely a [`Chord`]).
    fn dominant9(self) -> Chord;
    /// Returns a new chord with a dominant 9 modifier on the implementor (most likely a [`Chord`]).
    fn nine(self) -> Chord;
    /// Returns a new chord with a dominant 11 modifier on the implementor (most likely a [`Chord`]).
    fn dominant11(self) -> Chord;
    /// Returns a new chord with a dominant 11 modifier on the implementor (most likely a [`Chord`]).
    fn eleven(self) -> Chord;
    /// Returns a new chord with a dominant 13 modifier on the implementor (most likely a [`Chord`]).
    fn dominant13(self) -> Chord;
    /// Returns a new chord with a dominant 13 modifier on the implementor (most likely a [`Chord`]).
    fn thirteen(self) -> Chord;
    /// Returns a new chord with a dominant modifier on the implementor (most likely a [`Chord`]).
    fn dominant(self, dominant: Degree) -> Chord;

    /// Returns a new chord with a flat 9 modifier on the implementor (most likely a [`Chord`]).
    fn flat9(self) -> Chord;
    /// Returns a new chord with a flat 9 modifier on the implementor (most likely a [`Chord`]).
    fn flat_nine(self) -> Chord;

    /// Returns a new chord with a sharp 9 modifier on the implementor (most likely a [`Chord`]).
    fn sharp9(self) -> Chord;
    /// Returns a new chord with a sharp 9 modifier on the implementor (most likely a [`Chord`]).
    fn sharp_nine(self) -> Chord;

    /// Returns a new chord with a sharp 11 modifier on the implementor (most likely a [`Chord`]).
    fn sharp11(self) -> Chord;
    /// Returns a new chord with a sharp 11 modifier on the implementor (most likely a [`Chord`]).
    fn sharp_eleven(self) -> Chord;

    /// Returns a new chord with a flat 13 modifier on the implementor (most likely a [`Chord`]).
    fn flat13(self) -> Chord;
    /// Returns a new chord with a flat 13 modifier on the implementor (most likely a [`Chord`]).
    fn flat_thirteen(self) -> Chord;

    // Special.

    /// Returns a new chord with a diminished modifier on the implementor (most likely a [`Chord`]).
    fn diminished(self) -> Chord;
    /// Returns a new chord with a diminished modifier on the implementor (most likely a [`Chord`]).
    fn dim(self) -> Chord;

    /// Returns a new chord with a half-diminished (m7♭5) modifier on the implementor (most likely a [`Chord`]).
    fn half_diminished(self) -> Chord;
    /// Returns a new chord with a half-diminished (m7♭5) modifier on the implementor (most likely a [`Chord`]).
    fn half_dim(self) -> Chord;

    // Extensions.

    /// Returns a new chord with a sus2 extension on the implementor (most likely a [`Chord`]).
    fn sus2(self) -> Chord;
    /// Returns a new chord with a sus2 extension on the implementor (most likely a [`Chord`]).
    fn sus_two(self) -> Chord;

    /// Returns a new chord with a sus4 extension on the implementor (most likely a [`Chord`]).
    fn sus4(self) -> Chord;
    /// Returns a new chord with a sus4 extension on the implementor (most likely a [`Chord`]).
    fn sus_four(self) -> Chord;
    /// Returns a new chord with a sus4 extension on the implementor (most likely a [`Chord`]).
    fn sustain(self) -> Chord;
    /// Returns a new chord with a sus4 extension on the implementor (most likely a [`Chord`]).
    fn sus(self) -> Chord;

    /// Returns a new chord with a flat 11 extension on the implementor (most likely a [`Chord`]).
    fn flat11(self) -> Chord;
    /// Returns a new chord with a flat 11 extension on the implementor (most likely a [`Chord`]).
    fn flat_eleven(self) -> Chord;

    /// Returns a new chord with a sharp 13 extension on the implementor (most likely a [`Chord`]).
    fn sharp13(self) -> Chord;
    /// Returns a new chord with a sharp 13 extension on the implementor (most likely a [`Chord`]).
    fn sharp_thirteen(self) -> Chord;

    /// Returns a new chord with an add2 extension on the implementor (most likely a [`Chord`]).
    fn add2(self) -> Chord;
    /// Returns a new chord with an add2 extension on the implementor (most likely a [`Chord`]).
    fn add_two(self) -> Chord;

    /// Returns a new chord with an add4 extension on the implementor (most likely a [`Chord`]).
    fn add4(self) -> Chord;
    /// Returns a new chord with an add4 extension on the implementor (most likely a [`Chord`]).
    fn add_four(self) -> Chord;

    /// Returns a new chord with an add6 extension on the implementor (most likely a [`Chord`]).
    fn add6(self) -> Chord;
    /// Returns a new chord with an add6 extension on the implementor (most likely a [`Chord`]).
    fn add_six(self) -> Chord;

    /// Returns a new chord with an add9 extension on the implementor (most likely a [`Chord`]).
    fn add9(self) -> Chord;
    /// Returns a new chord with an add9 extension on the implementor (most likely a [`Chord`]).
    fn add_nine(self) -> Chord;

    /// Returns a new chord with an add11 extension on the implementor (most likely a [`Chord`]).
    fn add11(self) -> Chord;
    /// Returns a new chord with an add11 extension on the implementor (most likely a [`Chord`]).
    fn add_eleven(self) -> Chord;

    /// Returns a new chord with an add13 extension on the implementor (most likely a [`Chord`]).
    fn add13(self) -> Chord;
    /// Returns a new chord with an add13 extension on the implementor (most likely a [`Chord`]).
    fn add_thirteen(self) -> Chord;
}

/// A trait for types that have a dominant degree; i.e., 7, 9, 11, 13.
pub trait HasDomninantDegree {
    /// Returns the dominant degree of the implementor (most likely a [`Chord`]).
    fn dominant_degree(&self) -> Option<Degree>;
}

// Struct.

/// The primary chord struct.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Chord {
    /// The root note of the chord.
    root: Note,
    /// The slash note of the chord.
    slash: Option<Note>,
    /// The modifiers of the chord.
    modifiers: HashSet<Modifier>,
    /// The extensions of the chord.
    extensions: HashSet<Extension>,
    /// The inversion of the chord.
    inversion: u8,
    /// Whether or not this chord is "crunchy".
    ///
    /// Crunchy chords take extensions down an octave, which gives the chord some "crunch".
    is_crunchy: bool,
}

// Impls.

impl Ord for Chord {
    fn cmp(&self, other: &Self) -> Ordering {
        let a_inversion = self.inversion;
        let b_inversion = other.inversion;
        let cmp_inversion = a_inversion.cmp(&b_inversion);

        let a_slashes = self.slash.is_some() as u8;
        let b_slashes = other.slash.is_some() as u8;
        let cmp_slashes = a_slashes.cmp(&b_slashes);

        let a_crunchy = self.is_crunchy as u8;
        let b_crunchy = other.is_crunchy as u8;
        let cmp_crunchy = a_crunchy.cmp(&b_crunchy);

        let a_extensions_len = self.extensions.len() as u8;
        let b_extensions_len = other.extensions.len() as u8;
        let cmp_extensions = {
            let result = a_extensions_len.cmp(&b_extensions_len);

            if result.is_eq() {
                let a_extensions = Vec::from_iter(&self.extensions);
                let b_extensions = Vec::from_iter(&other.extensions);

                a_extensions.cmp(&b_extensions)
            } else {
                result
            }
        };

        let a_modifiers_len = self.modifiers.len() as u8;
        let b_modifiers_len = other.modifiers.len() as u8;
        let cmp_modifiers = {
            let result = a_modifiers_len.cmp(&b_modifiers_len);

            if result.is_eq() {
                let a_modifiers = Vec::from_iter(&self.modifiers);
                let b_modifiers = Vec::from_iter(&other.modifiers);

                a_modifiers.cmp(&b_modifiers)
            } else {
                result
            }
        };

        // Give a slight preference to chords without slashes and inversions.
        let a_inversion_exists = u8::from(a_inversion != 0);
        let b_inversion_exists = u8::from(b_inversion != 0);

        let a_all_changes_len = a_extensions_len + a_modifiers_len + 2 * a_slashes + 2 * a_inversion_exists;
        let b_all_changes_len = b_extensions_len + b_modifiers_len + 2 * b_slashes + 2 * b_inversion_exists;

        let cmp_all_changes = a_all_changes_len.cmp(&b_all_changes_len);

        let a_root = self.root;
        let b_root = other.root;
        let cmp_root = a_root.cmp(&b_root);

        cmp_all_changes
            .then(cmp_inversion)
            .then(cmp_slashes)
            .then(cmp_extensions)
            .then(cmp_modifiers)
            .then(cmp_root)
            .then(cmp_crunchy)
    }
}

impl PartialOrd for Chord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Chord {
    /// Returns a new chord with the given root.
    pub fn new(root: Note) -> Self {
        Self {
            root,
            slash: None,
            modifiers: HashSet::new(),
            extensions: HashSet::new(),
            inversion: 0,
            is_crunchy: false,
        }
    }
}

impl Chord {
    /// Attempts to guess the chord from detected pitch classes.
    /// Tries intelligent permutations across octaves to find all plausible voicings.
    /// The `try_from_notes` method will handle slash chords and inversions automatically.
    pub fn try_from_pitches(all_pitches: &[Pitch]) -> Res<Vec<Self>> {
        use crate::core::named_pitch::NamedPitch;
        use crate::core::octave::Octave;

        if all_pitches.is_empty() {
            return Err(anyhow::Error::msg("Must have at least one pitch to guess a chord."));
        }

        let mut all_note_combinations = Vec::new();

        // Convert pitches to notes at octave 4 as base
        let mut base_notes: Vec<Note> = all_pitches.iter().map(|&p| Note::new(NamedPitch::from(p), Octave::Four)).collect();
        base_notes.sort();

        // Generate rotations - each rotation bumps the first note(s) to octave 5
        for rotation in 0..base_notes.len() {
            let mut rotated = base_notes.clone();

            // Move first 'rotation' notes up an octave
            for i in 0..rotation {
                rotated[i] = rotated[i].with_octave(Octave::Five);
            }

            // Re-sort after octave adjustments
            rotated.sort();

            // Add this voicing to try
            all_note_combinations.push(rotated);
        }

        // Try all combinations through the existing chord guesser
        let mut all_candidates = Vec::new();
        for notes in all_note_combinations {
            if notes.len() >= 3 {
                if let Ok(candidates) = Self::try_from_notes(&notes) {
                    all_candidates.extend(candidates);
                }
            }
        }

        // Sort by complexity (simplest first), then deduplicate by string representation
        all_candidates.sort();
        all_candidates.dedup_by_key(|c| c.to_string());

        if all_candidates.is_empty() {
            return Err(anyhow::Error::msg("Could not determine chord from pitches."));
        }

        Ok(all_candidates)
    }

    /// Attempts to guess the chord from the notes.
    pub fn try_from_notes(notes: &[Note]) -> Res<Vec<Self>> {
        if notes.len() < 3 {
            return Err(anyhow::Error::msg("Must have at least three notes to guess a chord."));
        }

        let mut notes = notes.to_vec();
        notes.sort();

        let mut result = Vec::new();

        // Iterate through all known chords (and some likely extensions) and find the longest match.
        for inversion in 0..3 {
            let proper_root = if inversion == 0 {
                notes[0]
            } else {
                let note = notes[notes.len() - inversion];

                note.with_octave(note.octave() - 1)
            };

            let proper_root_slash = if inversion == 0 {
                notes[1]
            } else {
                let note = notes[notes.len() - inversion];

                note.with_octave(note.octave() - 1)
            };

            for mod_set in known_modifier_sets() {
                for mod_set2 in one_off_modifier_sets() {
                    for ext_set in likely_extension_sets() {
                        for is_crunchy in [false, true] {
                            // Check using the first note as the root.
                            let candidate_chord_root = Chord::new(proper_root)
                                .with_modifiers(mod_set)
                                .with_modifiers(mod_set2)
                                .with_extensions(ext_set)
                                .with_inversion(inversion as u8)
                                .with_crunchy(is_crunchy);
                            let candidate_chord_root_notes = candidate_chord_root.chord();

                            if notes.len() == candidate_chord_root_notes.len() && notes.iter().zip(&candidate_chord_root.chord()).all(|(a, b)| a.frequency() == b.frequency()) {
                                result.push(candidate_chord_root);
                            }

                            // Check using the first note as a slash.
                            let candidate_chord_slash = Chord::new(proper_root_slash)
                                .with_slash(notes[0])
                                .with_modifiers(mod_set)
                                .with_modifiers(mod_set2)
                                .with_extensions(ext_set)
                                .with_inversion(inversion as u8)
                                .with_crunchy(is_crunchy);
                            let candidate_chord_slash_notes = candidate_chord_slash.chord();

                            if notes.len() == candidate_chord_slash_notes.len() && notes.iter().zip(&candidate_chord_slash.chord()).all(|(a, b)| a.frequency() == b.frequency()) {
                                result.push(candidate_chord_slash);
                            }
                        }
                    }
                }
            }
        }

        // Remove extensions and modifiers that are expressed elsewhere in the chord.
        for c in &mut result {
            let dominant_degree = c.dominant_degree();

            if let Some(degree) = dominant_degree {
                match degree {
                    Degree::Nine => {
                        c.extensions.remove(&Extension::Add9);
                    }
                    Degree::Eleven => {
                        c.extensions.remove(&Extension::Add9);
                        c.extensions.remove(&Extension::Add11);
                    }
                    Degree::Thirteen => {
                        c.extensions.remove(&Extension::Add9);
                        c.extensions.remove(&Extension::Add11);
                        c.extensions.remove(&Extension::Add13);
                    }
                    Degree::Seven => {}
                }
            }

            if c.modifiers.contains(&Modifier::Diminished) {
                c.modifiers.remove(&Modifier::Minor);
                c.modifiers.remove(&Modifier::Flat5);
                c.modifiers.remove(&Modifier::Augmented5);
            }
        }

        // Order the candidates by "simplicity" (i.e., least slashes, least extensions, least modifiers, and least inversion).
        result.sort();

        // Remove duplicates (and ignore crunchy; i.e., `C7` and `C7!` should be treated as "the same").
        result.dedup_by(|a, b| a.modifiers == b.modifiers && a.extensions == b.extensions && a.slash == b.slash && a.inversion == b.inversion);

        Ok(result)
    }
}

impl HasName for Chord {
    fn name(&self) -> String {
        let known_name = self.known_chord().name();
        let known_name = known_name.as_str();
        let mut name = String::new();

        name.push_str(self.root.static_name());

        name.push_str(known_name);

        // Add special modifiers that are true modifiers when not part of their "special case".

        if self.modifiers.contains(&Modifier::Flat5) && !known_name.contains("(♭5)") {
            name.push_str("(♭5)");
        }

        if self.modifiers.contains(&Modifier::Augmented5) && !known_name.contains('+') && !known_name.contains("(♯5)") {
            name.push_str("(♯5)");
        }

        if self.modifiers.contains(&Modifier::Flat9) && !known_name.contains("(♭9)") {
            name.push_str("(♭9)");
        }

        if self.modifiers.contains(&Modifier::Sharp9) && !known_name.contains("(♯9)") {
            name.push_str("(♯9)");
        }

        if self.modifiers.contains(&Modifier::Sharp11) && !known_name.contains("(♯11)") {
            name.push_str("(♯11)");
        }

        if self.modifiers.contains(&Modifier::Flat13) && !known_name.contains("(♭13)") {
            name.push_str("(♭13)");
        }

        // Add extensions.
        if !self.extensions.is_empty() {
            for e in &self.extensions {
                name.push_str(&format!("({})", e.static_name()));
            }
        }

        // Add slash note.
        if let Some(slash) = self.slash {
            name.push_str(&format!("/{}", slash.static_name()));
        }

        // Add special information about the chord.

        name
    }
}

impl HasPreciseName for Chord {
    fn precise_name(&self) -> String {
        let mut name = String::new();

        name.push_str(&self.name());

        // Add octave modifier.
        if self.root.octave() != Octave::Four {
            name.push_str(&format!("@{}", self.root.octave().static_name()));
        }

        // Add inversion modifier.
        if self.inversion != 0 {
            name.push_str(&format!("^{}", self.inversion));
        }

        // Add crunchy modifier.
        if self.is_crunchy {
            name.push('!');
        }

        name
    }
}

impl HasRoot for Chord {
    fn root(&self) -> Note {
        self.root
    }
}

impl HasSlash for Chord {
    fn slash(&self) -> Note {
        self.slash.unwrap_or(self.root)
    }
}

impl HasModifiers for Chord {
    fn modifiers(&self) -> &HashSet<Modifier> {
        &self.modifiers
    }
}

impl HasExtensions for Chord {
    fn extensions(&self) -> &HashSet<Extension> {
        &self.extensions
    }
}

impl HasInversion for Chord {
    fn inversion(&self) -> u8 {
        self.inversion
    }
}

impl HasIsCrunchy for Chord {
    fn is_crunchy(&self) -> bool {
        self.is_crunchy
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scale = self.scale().iter().map(HasStaticName::static_name).collect::<Vec<_>>().join(", ");
        let chord = self.chord().iter().map(HasStaticName::static_name).collect::<Vec<_>>().join(", ");

        writeln!(f, "{}", self.precise_name())?;
        writeln!(f, "   {}", self.description())?;
        writeln!(f, "   {}", scale)?;
        write!(f, "   {}", chord)?;

        Ok(())
    }
}

impl Chordable for Chord {
    fn with_modifier(mut self, modifier: Modifier) -> Chord {
        // Augmented modifiers trump b5 and dim modifiers.
        if modifier == Modifier::Augmented5 {
            self.modifiers.remove(&Modifier::Flat5);
            self.modifiers.remove(&Modifier::Diminished);
        }

        if (modifier == Modifier::Diminished || modifier == Modifier::Flat5) && self.modifiers.contains(&Modifier::Augmented5) {
            return self;
        }

        self.modifiers.insert(modifier);

        self
    }

    fn with_modifiers(self, modifiers: &[Modifier]) -> Chord {
        let mut chord = self;

        for m in modifiers {
            chord = chord.with_modifier(*m);
        }

        chord
    }

    fn with_extension(mut self, extension: Extension) -> Chord {
        self.extensions.insert(extension);

        self
    }

    fn with_extensions(self, extensions: &[Extension]) -> Chord {
        let mut chord = self;

        for e in extensions {
            chord = chord.with_extension(*e);
        }

        chord
    }

    fn with_inversion(mut self, inversion: u8) -> Chord {
        self.inversion = inversion;

        self
    }

    fn with_slash(mut self, slash: Note) -> Chord {
        self.slash = Some(slash);

        self
    }

    fn with_octave(self, octave: Octave) -> Self {
        let root = Note::new(self.root.named_pitch(), octave);

        Chord { root, ..self }
    }

    fn with_crunchy(self, is_crunchy: bool) -> Chord {
        Chord { is_crunchy, ..self }
    }

    // Modifiers.

    fn minor(self) -> Chord {
        self.with_modifier(Modifier::Minor)
    }

    fn flat5(self) -> Chord {
        self.with_modifier(Modifier::Flat5)
    }

    fn flat_five(self) -> Chord {
        self.flat5()
    }

    fn augmented(self) -> Chord {
        self.with_modifier(Modifier::Augmented5)
    }

    fn aug(self) -> Chord {
        self.augmented()
    }

    fn major7(self) -> Chord {
        self.with_modifier(Modifier::Major7)
    }

    fn major_seven(self) -> Chord {
        self.major7()
    }

    fn maj7(self) -> Chord {
        self.major7()
    }

    fn dominant7(self) -> Chord {
        self.with_modifier(Modifier::Dominant(Degree::Seven))
    }

    fn seven(self) -> Chord {
        self.dominant7()
    }

    fn dominant9(self) -> Chord {
        self.with_modifier(Modifier::Dominant(Degree::Nine))
    }

    fn nine(self) -> Chord {
        self.dominant9()
    }

    fn dominant11(self) -> Chord {
        self.with_modifier(Modifier::Dominant(Degree::Eleven))
    }

    fn eleven(self) -> Chord {
        self.dominant11()
    }

    fn dominant13(self) -> Chord {
        self.with_modifier(Modifier::Dominant(Degree::Thirteen))
    }

    fn thirteen(self) -> Chord {
        self.dominant13()
    }

    fn dominant(self, dominant: Degree) -> Chord {
        self.with_modifier(Modifier::Dominant(dominant))
    }

    fn flat9(self) -> Chord {
        self.with_modifier(Modifier::Flat9)
    }

    fn flat_nine(self) -> Chord {
        self.flat9()
    }

    fn sharp9(self) -> Chord {
        self.with_modifier(Modifier::Sharp9)
    }

    fn sharp_nine(self) -> Chord {
        self.sharp9()
    }

    fn sharp11(self) -> Chord {
        self.with_modifier(Modifier::Sharp11)
    }

    fn sharp_eleven(self) -> Chord {
        self.sharp11()
    }

    // Special.

    fn diminished(self) -> Chord {
        self.with_modifier(Modifier::Diminished)
    }

    fn dim(self) -> Chord {
        self.diminished()
    }

    fn half_diminished(self) -> Chord {
        self.minor().seven().flat5()
    }

    fn half_dim(self) -> Chord {
        self.half_diminished()
    }

    // Extensions.

    fn sus2(self) -> Chord {
        self.with_extension(Extension::Sus2)
    }

    fn sus_two(self) -> Chord {
        self.sus2()
    }

    fn sus4(self) -> Chord {
        self.with_extension(Extension::Sus4)
    }

    fn sus_four(self) -> Chord {
        self.sus4()
    }

    fn sustain(self) -> Chord {
        self.sus4()
    }

    fn sus(self) -> Chord {
        self.sustain()
    }

    fn flat11(self) -> Chord {
        self.with_extension(Extension::Flat11)
    }

    fn flat_eleven(self) -> Chord {
        self.flat11()
    }

    fn flat13(self) -> Chord {
        self.with_modifier(Modifier::Flat13)
    }

    fn flat_thirteen(self) -> Chord {
        self.flat13()
    }

    fn sharp13(self) -> Chord {
        self.with_extension(Extension::Sharp13)
    }

    fn sharp_thirteen(self) -> Chord {
        self.sharp13()
    }

    fn add2(self) -> Chord {
        self.with_extension(Extension::Add2)
    }

    fn add_two(self) -> Chord {
        self.add2()
    }

    fn add4(self) -> Chord {
        self.with_extension(Extension::Add4)
    }

    fn add_four(self) -> Chord {
        self.add4()
    }

    fn add6(self) -> Chord {
        self.with_extension(Extension::Add6)
    }

    fn add_six(self) -> Chord {
        self.add6()
    }

    fn add9(self) -> Chord {
        self.with_extension(Extension::Add9)
    }

    fn add_nine(self) -> Chord {
        self.add9()
    }

    fn add11(self) -> Chord {
        self.with_extension(Extension::Add11)
    }

    fn add_eleven(self) -> Chord {
        self.add11()
    }

    fn add13(self) -> Chord {
        self.with_extension(Extension::Add13)
    }

    fn add_thirteen(self) -> Chord {
        self.add13()
    }
}

impl HasKnownChord for Chord {
    fn known_chord(&self) -> KnownChord {
        let modifiers = &self.modifiers;
        let degree = self.dominant_degree();

        let contains_dominant = degree.is_some();
        let degree = degree.unwrap_or(Degree::Seven);

        if modifiers.contains(&Modifier::Diminished) {
            KnownChord::Diminished
        } else if modifiers.contains(&Modifier::Minor) {
            if modifiers.contains(&Modifier::Major7) {
                return KnownChord::MinorMajor7;
            }

            if contains_dominant {
                if modifiers.contains(&Modifier::Flat5) {
                    return KnownChord::HalfDiminished(degree);
                }

                if modifiers.contains(&Modifier::Flat13) {
                    if modifiers.contains(&Modifier::Flat9) {
                        return KnownChord::MinorDominantFlat9Flat13(degree);
                    }

                    return KnownChord::MinorDominantFlat13(degree);
                }

                return KnownChord::MinorDominant(degree);
            }

            KnownChord::Minor
        } else {
            if modifiers.contains(&Modifier::Augmented5) {
                if modifiers.contains(&Modifier::Major7) {
                    return KnownChord::AugmentedMajor7;
                }

                if contains_dominant {
                    if modifiers.contains(&Modifier::Flat9) {
                        return KnownChord::AugmentedDominantFlat9(degree);
                    }
                    
                    return KnownChord::AugmentedDominant(degree);
                }

                return KnownChord::Augmented;
            }

            if self.modifiers.contains(&Modifier::Major7) {
                return KnownChord::Major7;
            }

            if contains_dominant {
                if modifiers.contains(&Modifier::Flat9) {
                    return KnownChord::DominantFlat9(degree);
                }

                if modifiers.contains(&Modifier::Sharp9) {
                    return KnownChord::DominantSharp9(degree);
                }

                if modifiers.contains(&Modifier::Sharp11) {
                    return KnownChord::DominantSharp11(degree);
                }

                return KnownChord::Dominant(degree);
            }

            // This is a special case where the sharp 11 has to be "alone".
            if self.modifiers.contains(&Modifier::Sharp11) && modifiers.len() == 1 {
                return KnownChord::Sharp11;
            }

            KnownChord::Major
        }
    }
}

impl HasDescription for Chord {
    fn description(&self) -> &'static str {
        self.known_chord().description()
    }
}

impl Chord {
    /// Returns the static interval candidates for this chord
    pub fn scale_interval_candidates(&self) -> &'static [IntervalCandidate] {
        self.known_chord().scale_interval_candidates()
    }
}

use crate::core::interval::HasIntervals;

impl HasIntervals for Chord {
    fn intervals(&self) -> &'static [Interval] {
        self.known_chord().intervals()
    }
}

impl HasScaleCandidates for Chord {
    fn scale_candidates(&self) -> Vec<ScaleCandidate> {
        self.scale_interval_candidates().iter().map(IntervalCandidate::to_scale_candidate).collect()
    }
}

impl HasRelativeScale for Chord {
    fn relative_scale(&self) -> Vec<Interval> {
        self.known_chord().relative_scale()
    }
}

impl HasRelativeChord for Chord {
    fn relative_chord(&self) -> Vec<Interval> {
        let mut result = self.known_chord().relative_chord();
        let modifiers = &self.modifiers;
        let extensions = &self.extensions;

        // Dominant extensions.

        if modifiers.contains(&Modifier::Dominant(Degree::Nine)) {
            result.push(Interval::MajorNinth);
        } else if modifiers.contains(&Modifier::Dominant(Degree::Eleven)) {
            result.push(Interval::MajorNinth);
            result.push(Interval::PerfectEleventh);
        } else if modifiers.contains(&Modifier::Dominant(Degree::Thirteen)) {
            result.push(Interval::MajorNinth);
            result.push(Interval::PerfectEleventh);
            result.push(Interval::MajorThirteenth);
        }

        // Special modifiers that can also be extensions.

        if modifiers.contains(&Modifier::Flat5) {
            result.remove(2);
            result.push(Interval::DiminishedFifth);
        }

        if modifiers.contains(&Modifier::Augmented5) {
            result.remove(2);
            result.push(Interval::AugmentedFifth);
        }

        if modifiers.contains(&Modifier::Flat9) {
            result.push(Interval::MinorNinth);
        }

        if modifiers.contains(&Modifier::Sharp9) {
            result.push(Interval::AugmentedNinth);
        }

        if modifiers.contains(&Modifier::Sharp11) {
            result.push(Interval::AugmentedEleventh);
        }

        if modifiers.contains(&Modifier::Flat13) {
            result.push(Interval::MinorThirteenth);
        }

        // Extensions.

        if extensions.contains(&Extension::Sus2) {
            result.remove(1);
            result.push(Interval::MajorSecond);
        }

        if extensions.contains(&Extension::Sus4) {
            result.remove(1);
            result.push(Interval::PerfectFourth);
        }

        if extensions.contains(&Extension::Flat11) {
            result.push(Interval::DiminishedEleventh);
        }

        if extensions.contains(&Extension::Sharp13) {
            result.push(Interval::AugmentedThirteenth);
        }

        if extensions.contains(&Extension::Add2) {
            result.push(Interval::MajorSecond);
        }

        if extensions.contains(&Extension::Add4) {
            result.push(Interval::PerfectFourth);
        }

        if extensions.contains(&Extension::Add6) {
            result.push(Interval::MajorSixth);
        }

        if extensions.contains(&Extension::Add9) {
            result.push(Interval::MajorNinth);
        }

        if extensions.contains(&Extension::Add11) {
            result.push(Interval::PerfectEleventh);
        }

        if extensions.contains(&Extension::Add13) {
            result.push(Interval::MajorThirteenth);
        }

        // Keep everything in order.
        result.sort();
        result.dedup();

        result
    }
}

impl HasScale for Chord {
    fn scale(&self) -> Vec<Note> {
        // Get the first (primary) interval candidate and root it at self.root()
        let candidates = self.scale_interval_candidates();
        if let Some(candidate) = candidates.first() {
            match candidate.kind {
                IntervalCollectionKind::Mode(kind) => crate::core::mode::Mode::new(self.root, kind).notes(),
                IntervalCollectionKind::Scale(kind) => crate::core::scale::Scale::new(self.root, kind).notes(),
            }
        } else {
            // Fallback to relative_scale if no candidates (shouldn't happen except for Unknown)
            self.relative_scale().into_iter().map(|i| self.root + i).collect()
        }
    }
}

impl HasChord for Chord {
    fn chord(&self) -> Vec<Note> {
        let mut result: Vec<_> = self.relative_chord().into_iter().map(|i| self.root + i).collect();

        // Perform inversions.
        for _ in 0..self.inversion {
            let mut note = result.remove(0);

            while note < *result.last().unwrap_or(&CZero) {
                note += Interval::PerfectOctave;
            }

            result.push(note);
        }

        // If this chord is crunchy, bring all "octave" intervals down to the first octave frame.
        if self.is_crunchy {
            let bottom = *result.first().unwrap_or(&CZero);
            let top = bottom.with_octave(bottom.octave() + 1);

            for note in &mut result {
                while *note > top {
                    *note = note.with_octave(note.octave() - 1);
                }
            }
        }

        // Add slash note.
        if let Some(mut slash) = self.slash {
            // Fix slash note (it should be less than, or equal to, one octave away from the bottom tone).
            let bottom = *result.first().unwrap_or(&CZero);
            let floor = Note::new(bottom.named_pitch(), bottom.octave() - 1);

            slash = slash.with_octave(Octave::Zero);
            while slash < floor {
                slash += Interval::PerfectOctave;
            }

            result.insert(0, slash);
        }

        // Crunchiness, etc. can introduce changes, so resort, and dedup.
        result.sort();
        result.dedup();

        result
    }
}

impl HasDomninantDegree for Chord {
    fn dominant_degree(&self) -> Option<Degree> {
        let modifiers = &self.modifiers;
        if !modifiers.contains(&Modifier::Dominant(Degree::Seven))
            && !modifiers.contains(&Modifier::Dominant(Degree::Nine))
            && !modifiers.contains(&Modifier::Dominant(Degree::Eleven))
            && !modifiers.contains(&Modifier::Dominant(Degree::Thirteen))
        {
            return None;
        }

        Some(match modifiers.iter().find(|m| m.is_dominant()) {
            Some(Modifier::Dominant(d)) => *d,
            _ => Degree::Seven,
        })
    }
}

impl Parsable for Chord {
    fn parse(input: &str) -> Res<Self>
    where
        Self: Sized,
    {
        let root = ChordParser::parse(Rule::chord, input)?.next().unwrap();

        assert_eq!(Rule::chord, root.as_rule());

        let mut components = root.into_inner();

        let note = components.next().unwrap();

        assert_eq!(Rule::note, note.as_rule());

        let mut result = Chord::new(note_str_to_note(note.into_inner().as_str())?);

        while let Some(component) = components.next() {
            match component.as_rule() {
                Rule::maj7_modifier => {
                    result = result.major7();
                }
                Rule::minor => {
                    result = result.minor();
                }
                Rule::augmented => {
                    result = result.augmented();
                }
                Rule::diminished => {
                    result = result.diminished();
                }
                Rule::half_diminished => {
                    result = result.half_diminished();
                }
                Rule::dominant_modifier => match component.as_str() {
                    "7" => {
                        result = result.seven();
                    }
                    "9" => {
                        result = result.nine();
                    }
                    "11" => {
                        result = result.eleven();
                    }
                    "13" => {
                        result = result.thirteen();
                    }
                    _ => {
                        return Err(anyhow::Error::msg(format!("Unknown dominant modifier: {}", component.as_str())));
                    }
                },
                Rule::modifier => match component.as_str() {
                    "sus2" => {
                        result = result.sus2();
                    }
                    "sus4" => {
                        result = result.sus4();
                    }
                    "add2" => {
                        result = result.add2();
                    }
                    "add4" => {
                        result = result.add4();
                    }
                    "add6" | "6" => {
                        result = result.add6();
                    }
                    "b5" | "♭5" => {
                        result = result.flat5();
                    }
                    "#5" | "♯5" => {
                        result = result.augmented();
                    }
                    "add9" => {
                        result = result.add9();
                    }
                    "b9" | "♭9" => {
                        result = result.flat9();
                    }
                    "#9" | "♯9" => {
                        result = result.sharp9();
                    }
                    "add11" => {
                        result = result.add11();
                    }
                    "b11" | "♭11" => {
                        result = result.flat11();
                    }
                    "#11" | "♯11" => {
                        result = result.sharp11();
                    }
                    "add13" => {
                        result = result.add13();
                    }
                    "b13" | "♭13" => {
                        result = result.flat13();
                    }
                    "#13" | "♯13" => {
                        result = result.sharp13();
                    }
                    _ => {
                        return Err(anyhow::Error::msg(format!("Unknown modifier: {}", component.as_str())));
                    }
                },
                Rule::slash => {
                    let note = note_str_to_note(components.next().unwrap().as_str())?;

                    result = result.with_slash(note);
                }
                Rule::at => {
                    let octave = octave_str_to_octave(components.next().unwrap().as_str())?;

                    result = result.with_octave(octave);
                }
                Rule::hat => {
                    let inversion = components.next().unwrap().as_str().parse::<u8>()?;

                    result = result.with_inversion(inversion);
                }
                Rule::bang => {
                    result = result.with_crunchy(true);
                }
                Rule::EOI => {}
                _ => {
                    return Err(anyhow::Error::msg(format!("Unknown rule in chord parser: {:?}", component.as_rule())));
                }
            }
        }

        Ok(result)
    }
}

#[cfg(feature = "audio")]
use super::base::{Playable, PlaybackHandle};

#[cfg(feature = "audio")]
impl Playable for Chord {
    #[coverage(off)]
    fn play(&self, delay: std::time::Duration, length: std::time::Duration, fade_in: std::time::Duration) -> Res<PlaybackHandle> {
        use rodio::{source::SineWave, OutputStreamBuilder, Sink, Source};

        let chord_tones = self.chord();

        if length.as_secs_f32() <= chord_tones.len() as f32 * delay.as_secs_f32() {
            return Err(anyhow::Error::msg(
                "The delay is too long for the length of play (i.e., the number of chord tones times the delay is longer than the length).",
            ));
        }

        let stream = OutputStreamBuilder::open_default_stream()?;

        let mut sinks = vec![];

        for (k, n) in chord_tones.into_iter().enumerate() {
            let sink = Sink::connect_new(stream.mixer());

            let d = delay * k as u32;

            let source = SineWave::new(n.frequency()).take_duration(length - d).buffered().delay(d).fade_in(fade_in).amplify(0.20);

            sink.append(source);

            sinks.push(sink);
        }

        Ok(PlaybackHandle::new(stream, sinks))
    }
}

impl Default for Chord {
    fn default() -> Self {
        Chord::new(super::note::C)
    }
}

impl Chord {
    /// Formats the chord with full scale/mode candidate recommendations.
    ///
    /// This returns a verbose string representation that includes:
    /// - Chord name, description, scale notes, and chord tones
    /// - Complete list of recommended scales/modes with rankings, reasons, notes, and descriptions
    ///
    /// Use this when you want comprehensive improvisation guidance.
    /// For minimal output, use `Display` instead (via `to_string()` or `format!("{}", chord)`).
    pub fn format_with_scale_candidates(&self) -> String {
        use std::fmt::Write;

        let mut result = String::new();

        let scale = self.scale().iter().map(HasStaticName::static_name).collect::<Vec<_>>().join(", ");
        let chord = self.chord().iter().map(HasStaticName::static_name).collect::<Vec<_>>().join(", ");

        writeln!(&mut result, "{}", self.precise_name()).unwrap();
        writeln!(&mut result, "   {}", self.description()).unwrap();
        writeln!(&mut result, "   {}", scale).unwrap();
        writeln!(&mut result, "   {}", chord).unwrap();

        // Add scale/mode candidates
        let candidates = self.scale_candidates();
        if !candidates.is_empty() {
            writeln!(&mut result).unwrap();
            writeln!(&mut result, "   Recommended scales/modes:").unwrap();
            for candidate in candidates {
                let notes = candidate.notes(self.root());
                let notes_str = notes.iter().map(HasStaticName::static_name).collect::<Vec<_>>().join(", ");
                writeln!(&mut result, "     {}. {} - {} ({})", candidate.rank(), candidate.name(), candidate.reason(), notes_str).unwrap();
                writeln!(&mut result, "        {}", candidate.description()).unwrap();
            }
        }

        result
    }
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{note::*, octave::HasOctave};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_text() {
        assert_eq!(Chord::new(C).flat9().sharp9().sharp11().add13().with_slash(E).name(), "C(♭9)(♯9)(♯11)(add13)/E");
        assert_eq!(Chord::new(C).flat5().name(), "C(♭5)");
        assert_eq!(Chord::new(C).minor().augmented().name(), "Cm(♯5)");
        assert_eq!(Chord::new(C).with_octave(Octave::Six).precise_name(), "C@6");

        // Test Display is minimal (no scale candidates)
        let display_output = format!("{}", Chord::new(C).minor().seven().flat_five());
        assert!(display_output.contains("Cm7(♭5)"));
        assert!(display_output.contains("half diminished"));
        assert!(display_output.contains("C, D♭, E♭, F, G♭, A♭, B♭"));
        assert!(display_output.contains("C, E♭, G♭, B♭"));
        assert!(!display_output.contains("Recommended scales/modes:"));

        // Test format_with_scale_candidates includes recommendations
        let verbose_output = Chord::new(C).minor().seven().flat_five().format_with_scale_candidates();
        assert!(verbose_output.contains("Recommended scales/modes:"));
        assert!(verbose_output.contains("locrian"));
    }

    #[test]
    fn test_display_format() {
        // Test that Display output is minimal and stable (no scale candidates)
        let chord = Chord::new(C);
        let output = format!("{}", chord);
        let expected = "C\n   major\n   C, D, E, F, G, A, B\n   C, E, G";
        assert_eq!(output, expected);

        // Test that format_with_scale_candidates includes recommendations
        let verbose_output = chord.format_with_scale_candidates();
        assert!(verbose_output.contains("Recommended scales/modes:"));
        assert!(verbose_output.contains("ionian"));
        assert!(verbose_output.contains("major pentatonic"));
    }

    #[test]
    fn test_properties() {
        assert_eq!(Chord::new(C).seven().flat9().root(), C);
        assert_eq!(Chord::new(C).with_slash(E).slash(), E);
        assert_eq!(Chord::new(C).slash(), C);
        assert_eq!(Chord::new(C).flat9().add13().with_slash(E).modifiers(), &vec![Modifier::Flat9].into_iter().collect::<HashSet<_>>());
        assert_eq!(Chord::new(C).flat9().add13().with_slash(E).extensions(), &vec![Extension::Add13].into_iter().collect::<HashSet<_>>());
        assert_eq!(Chord::new(C).flat9().add13().with_slash(E).seven().dominant_degree(), Some(Degree::Seven));
        assert_eq!(Chord::new(C).flat9().add13().with_slash(E).nine().dominant_degree(), Some(Degree::Nine));
        assert_eq!(Chord::new(C).flat9().with_inversion(1).inversion(), 1);
        assert_eq!(Chord::new(C).flat9().with_octave(Octave::Three).root().octave(), Octave::Three);
    }

    #[test]
    fn test_known_chords() {
        assert_eq!(Chord::new(C).known_chord(), KnownChord::Major);
        assert_eq!(Chord::new(C).minor().known_chord(), KnownChord::Minor);
        assert_eq!(Chord::new(C).major7().known_chord(), KnownChord::Major7);
        assert_eq!(Chord::new(C).minor().major7().known_chord(), KnownChord::MinorMajor7);
        assert_eq!(Chord::new(C).minor().dominant(Degree::Seven).known_chord(), KnownChord::MinorDominant(Degree::Seven));
        assert_eq!(Chord::new(C).minor().eleven().known_chord(), KnownChord::MinorDominant(Degree::Eleven));
        assert_eq!(Chord::new(C).seven().known_chord(), KnownChord::Dominant(Degree::Seven));
        assert_eq!(Chord::new(C).eleven().known_chord(), KnownChord::Dominant(Degree::Eleven));
        assert_eq!(Chord::new(C).thirteen().known_chord(), KnownChord::Dominant(Degree::Thirteen));
        assert_eq!(Chord::new(C).diminished().known_chord(), KnownChord::Diminished);
        assert_eq!(Chord::new(C).dim().known_chord(), KnownChord::Diminished);
        assert_eq!(Chord::new(C).minor().seven().flat5().known_chord(), KnownChord::HalfDiminished(Degree::Seven));
        assert_eq!(Chord::new(C).augmented().known_chord(), KnownChord::Augmented);
        assert_eq!(Chord::new(C).aug().major7().known_chord(), KnownChord::AugmentedMajor7);
        assert_eq!(Chord::new(C).augmented().seven().known_chord(), KnownChord::AugmentedDominant(Degree::Seven));
        assert_eq!(Chord::new(C).seven().sharp11().known_chord(), KnownChord::DominantSharp11(Degree::Seven));
        assert_eq!(Chord::new(C).seven().flat9().known_chord(), KnownChord::DominantFlat9(Degree::Seven));
        assert_eq!(Chord::new(C).seven().sharp9().known_chord(), KnownChord::DominantSharp9(Degree::Seven));
        assert_eq!(Chord::new(C).seven().flat9().augmented().known_chord(), KnownChord::AugmentedDominantFlat9(Degree::Seven));

        assert_eq!(Chord::new(C).sus2().known_chord(), KnownChord::Major);
        assert_eq!(Chord::new(C).sus4().known_chord(), KnownChord::Major);
        assert_eq!(Chord::new(C).sustain().known_chord(), KnownChord::Major);
        assert_eq!(Chord::new(C).seven().sus().known_chord(), KnownChord::Dominant(Degree::Seven));
    }

    #[test]
    fn test_scales() {
        // Basic.

        assert_eq!(Chord::new(C).scale(), vec![C, D, E, F, G, A, B]);
        assert_eq!(Chord::new(C).minor().scale(), vec![C, D, EFlat, F, G, AFlat, BFlat]);
        assert_eq!(Chord::new(C).major_seven().scale(), vec![C, D, E, F, G, A, B]);
        assert_eq!(Chord::new(C).minor().maj7().scale(), vec![C, D, EFlat, F, G, A, B]);
        assert_eq!(Chord::new(C).minor().seven().scale(), vec![C, D, EFlat, F, G, A, BFlat]);
        assert_eq!(Chord::new(C).minor().eleven().scale(), vec![C, D, EFlat, F, G, A, BFlat]);
        assert_eq!(Chord::new(C).seven().scale(), vec![C, D, E, F, G, A, BFlat]);
        assert_eq!(Chord::new(C).eleven().scale(), vec![C, D, E, F, G, A, BFlat]);
        assert_eq!(Chord::new(C).thirteen().scale(), vec![C, D, E, F, G, A, BFlat]);
        assert_eq!(Chord::new(C).diminished().scale(), vec![C, D, EFlat, F, GFlat, AFlat, BDoubleFlat, B]);
        assert_eq!(Chord::new(C).dim().scale(), vec![C, D, EFlat, F, GFlat, AFlat, BDoubleFlat, B]);
        assert_eq!(Chord::new(C).minor().seven().flat5().scale(), vec![C, DFlat, EFlat, F, GFlat, AFlat, BFlat]);
        assert_eq!(Chord::new(C).augmented().scale(), vec![C, D, E, F, GSharp, A, B]);
        assert_eq!(Chord::new(C).augmented().major7().scale(), vec![C, D, E, FSharp, GSharp, A, B]);
        assert_eq!(Chord::new(C).augmented().seven().scale(), vec![C, D, E, FSharp, GSharp, ASharp]);
        assert_eq!(Chord::new(C).seven().sharp_eleven().scale(), vec![C, D, E, FSharp, G, A, BFlat]);
        assert_eq!(Chord::new(C).seven().flat_nine().scale(), vec![C, DFlat, EFlat, E, FSharp, G, A, BFlat]);
        assert_eq!(Chord::new(C).seven().sharp_nine().scale(), vec![C, DFlat, EFlat, FFlat, GFlat, AFlat, BFlat]);

        // Others.

        assert_eq!(Chord::new(DFlat).scale(), vec![DFlat, EFlat, F, GFlat, AFlat, BFlat, CFive]);
        assert_eq!(Chord::new(DFlat).seven().scale(), vec![DFlat, EFlat, F, GFlat, AFlat, BFlat, CFlatFive]);
        assert_eq!(Chord::new(DFlat).dim().scale(), vec![DFlat, EFlat, FFlat, GFlat, ADoubleFlat, BDoubleFlat, CDoubleFlatFive, CFive]);
    }

    #[test]
    fn test_chords() {
        // Basic.

        assert_eq!(Chord::new(C).chord(), vec![C, E, G]);
        assert_eq!(Chord::new(C).minor().chord(), vec![C, EFlat, G]);
        assert_eq!(Chord::new(C).major7().chord(), vec![C, E, G, B]);
        assert_eq!(Chord::new(C).minor().major7().chord(), vec![C, EFlat, G, B]);
        assert_eq!(Chord::new(C).minor().seven().chord(), vec![C, EFlat, G, BFlat]);
        assert_eq!(Chord::new(C).minor().eleven().chord(), vec![C, EFlat, G, BFlat, DFive, FFive]);
        assert_eq!(Chord::new(C).seven().chord(), vec![C, E, G, BFlat]);
        assert_eq!(Chord::new(C).eleven().chord(), vec![C, E, G, BFlat, DFive, FFive]);
        assert_eq!(Chord::new(C).thirteen().chord(), vec![C, E, G, BFlat, DFive, FFive, AFive]);
        assert_eq!(Chord::new(C).diminished().chord(), vec![C, EFlat, GFlat, BDoubleFlat]);
        assert_eq!(Chord::new(C).dim().chord(), vec![C, EFlat, GFlat, BDoubleFlat]);
        assert_eq!(Chord::new(C).minor().seven().flat5().chord(), vec![C, EFlat, GFlat, BFlat]);
        assert_eq!(Chord::new(C).half_diminished().chord(), vec![C, EFlat, GFlat, BFlat]);
        assert_eq!(Chord::new(C).half_dim().chord(), vec![C, EFlat, GFlat, BFlat]);
        assert_eq!(Chord::new(C).augmented().chord(), vec![C, E, GSharp]);
        assert_eq!(Chord::new(C).augmented().major7().chord(), vec![C, E, GSharp, B]);
        assert_eq!(Chord::new(C).augmented().seven().chord(), vec![C, E, GSharp, BFlat]);
        assert_eq!(Chord::new(C).seven().sharp11().chord(), vec![C, E, G, BFlat, FSharpFive]);
        assert_eq!(Chord::new(C).seven().flat_nine().chord(), vec![C, E, G, BFlat, DFlatFive]);
        assert_eq!(Chord::new(C).seven().sharp_nine().chord(), vec![C, E, G, BFlat, DSharpFive]);

        // Extensions.

        assert_eq!(Chord::new(C).nine().sus2().chord(), vec![C, D, G, BFlat, DFive]);
        assert_eq!(Chord::new(C).nine().sus_two().chord(), vec![C, D, G, BFlat, DFive]);
        assert_eq!(Chord::new(C).nine().sus4().chord(), vec![C, F, G, BFlat, DFive]);
        assert_eq!(Chord::new(C).nine().sus_four().chord(), vec![C, F, G, BFlat, DFive]);
        assert_eq!(Chord::new(C).nine().sustain().chord(), vec![C, F, G, BFlat, DFive]);
        assert_eq!(Chord::new(C).seven().sus().chord(), vec![C, F, G, BFlat]);
        assert_eq!(Chord::new(C).seven().add2().chord(), vec![C, D, E, G, BFlat]);
        assert_eq!(Chord::new(C).seven().add_two().chord(), vec![C, D, E, G, BFlat]);
        assert_eq!(Chord::new(C).seven().add4().chord(), vec![C, E, F, G, BFlat]);
        assert_eq!(Chord::new(C).seven().add_four().chord(), vec![C, E, F, G, BFlat]);
        assert_eq!(Chord::new(C).add6().chord(), vec![C, E, G, A]);
        assert_eq!(Chord::new(C).seven().add9().chord(), vec![C, E, G, BFlat, DFive]);
        assert_eq!(Chord::new(C).seven().add_nine().chord(), vec![C, E, G, BFlat, DFive]);
        assert_eq!(Chord::new(C).seven().add11().chord(), vec![C, E, G, BFlat, FFive]);
        assert_eq!(Chord::new(C).seven().add_eleven().chord(), vec![C, E, G, BFlat, FFive]);
        assert_eq!(Chord::new(C).seven().add13().chord(), vec![C, E, G, BFlat, AFive]);
        assert_eq!(Chord::new(C).seven().add_thirteen().chord(), vec![C, E, G, BFlat, AFive]);
        assert_eq!(Chord::new(C).seven().add2().add4().chord(), vec![C, D, E, F, G, BFlat]);
        assert_eq!(Chord::new(C).seven().add6().chord(), vec![C, E, G, A, BFlat]);
        assert_eq!(Chord::new(C).seven().add_six().chord(), vec![C, E, G, A, BFlat]);
        assert_eq!(Chord::new(C).seven().flat11().chord(), vec![C, E, G, BFlat, FFlatFive]);
        assert_eq!(Chord::new(C).seven().flat_eleven().chord(), vec![C, E, G, BFlat, FFlatFive]);
        assert_eq!(Chord::new(C).seven().flat13().chord(), vec![C, E, G, BFlat, AFlatFive]);
        assert_eq!(Chord::new(C).seven().flat_thirteen().chord(), vec![C, E, G, BFlat, AFlatFive]);
        assert_eq!(Chord::new(C).seven().sharp13().chord(), vec![C, E, G, BFlat, ASharpFive]);
        assert_eq!(Chord::new(C).seven().sharp_thirteen().chord(), vec![C, E, G, BFlat, ASharpFive]);

        // Crunchy.

        assert_eq!(Chord::new(C).seven().sharp9().with_crunchy(true).chord(), vec![C, DSharp, E, G, BFlat]);

        // Slashes.

        assert_eq!(Chord::new(C).with_slash(D).chord(), vec![DThree, C, E, G]);
        assert_eq!(Chord::new(CFive).with_slash(D).chord(), vec![DFour, CFive, EFive, GFive]);

        // Inversions.

        assert_eq!(C.into_chord().with_inversion(1).chord(), vec![E, G, CFive]);
        assert_eq!(C.into_chord().with_inversion(2).chord(), vec![G, CFive, EFive]);
        assert_eq!(C.into_chord().maj7().with_inversion(3).chord(), vec![B, CFive, EFive, GFive]);
        assert_eq!(BFlatThree.into_chord().seven().flat9().with_inversion(1).chord(), vec![D, F, AFlat, CFlatFive, BFlatFive]);

        // Weird.
        assert_eq!(C.into_chord().flat5().aug().chord(), vec![C, E, GSharp]);
    }

    #[test]
    fn test_parse() {
        assert_eq!(Chord::parse("C").unwrap().chord(), vec![C, E, G]);
        assert_eq!(Chord::parse("Cm").unwrap().chord(), vec![C, EFlat, G]);
        assert_eq!(Chord::parse("Cm7").unwrap().chord(), vec![C, EFlat, G, BFlat]);
        assert_eq!(Chord::parse("Cm7b5").unwrap().chord(), vec![C, EFlat, GFlat, BFlat]);
        assert_eq!(Chord::parse("C7").unwrap().chord(), vec![C, E, G, BFlat]);
        assert_eq!(Chord::parse("C7b9").unwrap().chord(), vec![C, E, G, BFlat, DFlatFive]);
        assert_eq!(Chord::parse("C7b9#11").unwrap().chord(), vec![C, E, G, BFlat, DFlatFive, FSharpFive]);
        assert_eq!(Chord::parse("C(add6)").unwrap().chord(), vec![C, E, G, A]);
        assert_eq!(Chord::parse("Em(#5)").unwrap().chord(), vec![E, G, BSharp]);
        assert_eq!(Chord::parse("D+11").unwrap().chord(), vec![D, FSharp, ASharp, CFive, EFive, GFive]);
        assert_eq!(Chord::parse("Dm13b5").unwrap().chord(), vec![D, F, AFlat, CFive, EFive, GFive, BFive]);
        assert_eq!(Chord::parse("Dsus2").unwrap().chord(), vec![D, E, A]);
        assert_eq!(Chord::parse("Dsus4").unwrap().chord(), vec![D, G, A]);
        assert_eq!(Chord::parse("Dadd2").unwrap().chord(), vec![D, E, FSharp, A]);
        assert_eq!(Chord::parse("Dadd4").unwrap().chord(), vec![D, FSharp, G, A]);
        assert_eq!(Chord::parse("Dadd9").unwrap().chord(), vec![D, FSharp, A, EFive]);
        assert_eq!(Chord::parse("Dadd11").unwrap().chord(), vec![D, FSharp, A, GFive]);
        assert_eq!(Chord::parse("Dadd13").unwrap().chord(), vec![D, FSharp, A, BFive]);
        assert_eq!(Chord::parse("Dm#9").unwrap().chord(), vec![D, F, A, ESharpFive]);
        assert_eq!(Chord::parse("Dmb11").unwrap().chord(), vec![D, F, A, GFlatFive]);
        assert_eq!(Chord::parse("D(b13)").unwrap().chord(), vec![D, FSharp, A, BFlatFive]);
        assert_eq!(Chord::parse("D(#13)").unwrap().chord(), vec![D, FSharp, A, BSharpFive]);
    }

    #[test]
    fn test_guess() {
        assert_eq!(
            Chord::try_from_notes(&[EThree, C, EFlat, FSharp, ASharp, DFive]).unwrap().first().unwrap().chord(),
            Chord::parse("Cm9b5/E").unwrap().chord()
        );
        assert_eq!(Chord::try_from_notes(&[C, E, G]).unwrap().first().unwrap().chord(), Chord::parse("C").unwrap().chord());
        assert_eq!(
            Chord::try_from_notes(&[C, E, G, BFlat, DFive, FFive]).unwrap().first().unwrap().chord(),
            Chord::parse("C11").unwrap().chord()
        );
        assert_eq!(
            Chord::try_from_notes(&[C, E, G, BFlat, DFive, FFive, AFive]).unwrap().first().unwrap().chord(),
            Chord::parse("C13").unwrap().chord()
        );
        assert_eq!(Chord::try_from_notes(&[C, EFlat, GFlat, A]).unwrap().first().unwrap().chord(), Chord::parse("Cdim").unwrap().chord());
    }

    #[test]
    #[should_panic(expected = "Must have at least three notes to guess a chord.")]
    fn test_chord_from_notes_failure() {
        Chord::try_from_notes(&[C, E]).unwrap();
    }
}
