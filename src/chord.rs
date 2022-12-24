use std::{collections::HashSet, fmt::Display};

use pest::Parser;

use crate::{note::{Note, CZero, NoteRecreator}, modifier::{Modifier, Extension, Degree, HasIsDominant, known_modifier_sets, likely_extension_sets, one_off_modifier_sets}, known_chord::{KnownChord, HasRelativeChord, HasRelativeScale}, interval::Interval, base::{HasDescription, HasName, HasStaticName, Res, Parsable}, parser::{ChordParser, Rule, note_str_to_note}, octave::{Octave, HasOctave}, named_pitch::HasNamedPitch, pitch::{HasFrequency}};

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

    // Modifiers.

    /// Returns a new chord with a minor modifier on the implementor (most likely a [`Chord`]).
    fn minor(self) -> Chord;
    /// Returns a new chord with a minor modifier on the implementor (most likely a [`Chord`]).
    fn min(self) -> Chord;

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

    /// Returns a new chord with a flat 13 extension on the implementor (most likely a [`Chord`]).
    fn flat13(self) -> Chord;
    /// Returns a new chord with a flat 13 extension on the implementor (most likely a [`Chord`]).
    fn flat_thirteen(self) -> Chord;

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
}

// Impls.

impl Chord {
    /// Returns a new chord with the given root.
    pub fn new(root: Note) -> Self {
        Self { 
            root, 
            slash: None, 
            modifiers: HashSet::new(), 
            extensions: HashSet::new(), 
            inversion: 0 
        }
    }

    /// Attempts to guess the chord from the notes.
    pub fn from_notes(notes: &[Note]) -> Res<Vec<Self>> {
        if notes.len() < 3 {
            return Err(anyhow::Error::msg("Must have at least three notes to guess a chord."));
        }

        let mut notes = notes.to_vec();
        notes.sort();

        let mut result = Vec::new();

        // Iterate through all known chords (and some likely extensions) and find the longest match.
        for mod_set in known_modifier_sets() {
            for mod_set2 in one_off_modifier_sets() {
                for ext_set in likely_extension_sets() {
                    // Check using the first note as the root.
                    let candidate_chord_root = Chord::new(notes[0]).with_modifiers(mod_set).with_modifiers(mod_set2).with_extensions(ext_set);
                    let candidate_chord_root_notes = candidate_chord_root.chord();
    
                    if notes.len() == candidate_chord_root_notes.len() && notes.iter().zip(&candidate_chord_root.chord()).all(|(a, b)| a.frequency() == b.frequency()) {
                        result.push(candidate_chord_root);
                    }
    
                    // Check using the first note as a slash.
                    let candidate_chord_slash = Chord::new(notes[1]).with_slash(notes[0]).with_modifiers(mod_set).with_extensions(ext_set);
                    let candidate_chord_slash_notes = candidate_chord_slash.chord();
    
                    if notes.len() == candidate_chord_slash_notes.len() && notes.iter().zip(&candidate_chord_slash.chord()).all(|(a, b)| a.frequency() == b.frequency()) {
                        result.push(candidate_chord_slash);
                    }
                }
            }
        }

        // Remove extensions and modifiers that are expressed elsewhere in the chord.
        result.iter_mut().for_each(|c| {
            let dominant_degree = c.dominant_degree();

            if let Some(degree) = dominant_degree {
                match degree {
                    Degree::Nine => {
                        c.extensions.remove(&Extension::Add9);
                    },
                    Degree::Eleven => {
                        c.extensions.remove(&Extension::Add9);
                        c.extensions.remove(&Extension::Add11);
                    },
                    Degree::Thirteen => {
                        c.extensions.remove(&Extension::Add9);
                        c.extensions.remove(&Extension::Add11);
                        c.extensions.remove(&Extension::Add13);
                    },
                    _ => {}
                }
            }

            if c.modifiers.contains(&Modifier::Diminished) {
                c.modifiers.remove(&Modifier::Minor);
                c.modifiers.remove(&Modifier::Flat5);
                c.modifiers.remove(&Modifier::Augmented5);
            }
        });

        // Order the candidates by "simplicity" (i.e., least slashes, least extensions, and least modifiers).
        result.sort_by(|a, b| {
            let a_slashes = a.slash.is_some() as u8;
            let b_slashes = b.slash.is_some() as u8;

            let a_extensions = a.extensions.len() as u8;
            let b_extensions = b.extensions.len() as u8;

            let a_modifiers = a.modifiers.len() as u8;
            let b_modifiers = b.modifiers.len() as u8;

            a_slashes.cmp(&b_slashes).then(a_extensions.cmp(&b_extensions)).then(a_modifiers.cmp(&b_modifiers))
        });

        // Remove duplicates.
        result.dedup_by(|a, b| a.modifiers == b.modifiers && a.extensions == b.extensions);

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

        if self.modifiers.contains(&Modifier::Flat9) && !known_name.contains("(♭9)") {
            name.push_str("(♭9)");
        }

        if self.modifiers.contains(&Modifier::Sharp9) && !known_name.contains("(♯9)") {
            name.push_str("(♯9)");
        }

        if self.modifiers.contains(&Modifier::Sharp11) && !known_name.contains("(♯11)") {
            name.push_str("(♯11)");
        }

        // Add extensions.
        if !self.extensions.is_empty() {
            for e in self.extensions.iter() {
                name.push_str(&format!("({})", e.static_name()));
            }
        }

        // Ad slash note.
        if let Some(slash) = self.slash {
            name.push_str(&format!("/{}", slash.static_name()));
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

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scale = self.scale().iter().map(|n| n.static_name()).collect::<Vec<_>>().join(", ");
        let chord = self.chord().iter().map(|n| n.static_name()).collect::<Vec<_>>().join(", ");

        write!(f, "{}\n   {}\n   {}\n   {}", self.name(), self.description(), scale, chord)
    }
}

impl Chordable for Chord {
    fn with_modifier(mut self, modifier: Modifier) -> Chord {
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

        Chord {
            root,
            ..self
        }
    }

    // Modifiers.

    fn minor(self) -> Chord {
        self.with_modifier(Modifier::Minor)
    }

    fn min(self) -> Chord {
        self.minor()
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
        self.with_extension(Extension::Flat13)
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

                return KnownChord::MinorDominant(degree);
            }

            return KnownChord::Minor;
        } else {
            if modifiers.contains(&Modifier::Augmented5) {
                if modifiers.contains(&Modifier::Major7) {
                    return KnownChord::AugmentedMajor7;
                }

                if contains_dominant {
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
            
            return KnownChord::Major;
        }
    }
}

impl HasDescription for Chord {
    fn description(&self) -> &'static str {
        self.known_chord().description()
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

        if modifiers.contains(&Modifier::Flat9) {
            result.push(Interval::MinorNinth);
        }

        if modifiers.contains(&Modifier::Sharp9) {
            result.push(Interval::AugmentedNinth);
        }

        if modifiers.contains(&Modifier::Sharp11) {
            result.push(Interval::AugmentedEleventh);
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

        if extensions.contains(&Extension::Flat13) {
            result.push(Interval::MinorThirteenth);
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
        
        result.sort();
        result.dedup();

        result
    }
}

impl HasScale for Chord {
    fn scale(&self) -> Vec<Note> {
        self.relative_scale().into_iter().map(|i| self.root + i).collect()
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

        result
    }
}

impl HasDomninantDegree for Chord {
    fn dominant_degree(&self) -> Option<Degree> {
        let modifiers = &self.modifiers;
        if !modifiers.contains(&Modifier::Dominant(Degree::Seven)) && !modifiers.contains(&Modifier::Dominant(Degree::Nine)) && !modifiers.contains(&Modifier::Dominant(Degree::Eleven)) && !modifiers.contains(&Modifier::Dominant(Degree::Thirteen)) {
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
        
        let mut result = Chord::new(note_str_to_note(note.as_str())?);

        while let Some(component) = components.next() {
            match component.as_rule() {
                Rule::maj7_modifier => {
                    result = result.major7();
                },
                Rule::minor => {
                    result = result.minor();
                },
                Rule::augmented => {
                    result = result.augmented();
                },
                Rule::diminished => {
                    result = result.diminished();
                },
                Rule::half_diminished => {
                    result = result.half_diminished();
                },
                Rule::dominant_modifier => {
                    match component.as_str() {
                        "7" => {
                            result = result.seven();
                        },
                        "9" => {
                            result = result.nine();
                        },
                        "11" => {
                            result = result.eleven();
                        },
                        "13" => {
                            result = result.thirteen();
                        },
                        _ => {
                            unreachable!();
                        }
                    }
                },
                Rule::modifier => {
                    match component.as_str() {
                        "sus2" => {
                            result = result.sus2();
                        },
                        "sus4" => {
                            result = result.sus4();
                        },
                        "add2" => {
                            result = result.add2();
                        },
                        "add4" => {
                            result = result.add4();
                        },
                        "add6" | "6" => {
                            result = result.add6();
                        },
                        "b5" | "♭5" => {
                            result = result.flat5();
                        },
                        "add9" => {
                            result = result.add9();
                        },
                        "b9" | "♭9" => {
                            result = result.flat9();
                        },
                        "#9" | "♯9" => {
                            result = result.sharp9();
                        },
                        "add11" => {
                            result = result.add11();
                        },
                        "b11" | "♭11" => {
                            result = result.flat11();
                        },
                        "#11" | "♯11" => {
                            result = result.sharp11();
                        },
                        "add13" => {
                            result = result.add13();
                        },
                        "b13" | "♭13" => {
                            result = result.flat13();
                        },
                        "#13" | "♯13" => {
                            result = result.sharp13();
                        },
                        _ => {
                            unreachable!();
                        }
                    }
                },
                Rule::slash => {
                    let note = note_str_to_note(components.next().unwrap().as_str())?;

                    result = result.with_slash(note);
                },
                Rule::EOI => {},
                _ => {
                    unreachable!();
                }
            }
        }

        Ok(result)
    }
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};
    use crate::{note::*, octave::HasOctave};

    #[test]
    fn test_text() {
        assert_eq!(Chord::new(C).flat9().sharp9().sharp11().add13().with_slash(E).name(), "C(♭9)(♯9)(♯11)(add13)/E");
        assert_eq!(format!("{}", Chord::new(C).min().seven().flat_five()), "Cm7(♭5)\n   half diminished, locrian, minor seven flat five, seventh mode of major scale, major scale starting one half step up\n   C, D, E♭, F, G♭, A♭, B♭\n   C, E♭, G♭, B♭");
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
        assert_eq!(Chord::new(C).minor().seven().flat5().scale(), vec![C, D, EFlat, F, GFlat, AFlat, BFlat]);
        assert_eq!(Chord::new(C).augmented().scale(), vec![C, D, E, F, GSharp, A, B]);
        assert_eq!(Chord::new(C).augmented().major7().scale(), vec![C, D, E, FSharp, GSharp, A, B]);
        assert_eq!(Chord::new(C).augmented().seven().scale(), vec![C, D, E, FSharp, GSharp, ASharp]);
        assert_eq!(Chord::new(C).seven().sharp_eleven().scale(), vec![C, D, E, FSharp, G, A, BFlat]);
        assert_eq!(Chord::new(C).seven().flat_nine().scale(), vec![C, DFlat, EFlat, E, FSharp, G, A, BFlat]);
        assert_eq!(Chord::new(C).seven().sharp_nine().scale(), vec![C, DFlat, EFlat, FFlat, GFlat, AFlat, BFlat]);

        // Others.

        assert_eq!(Chord::new(DFlat).scale(), vec![DFlat, EFlat, F, GFlat, AFlat, BFlat, CFive]);
        assert_eq!(Chord::new(DFlat).seven().scale(), vec![DFlat, EFlat, F, GFlat, AFlat, BFlat, CFlatFive]);
        assert_eq!(Chord::new(DFlat).dim().scale(), vec![DFlat, EFlat, FFlat, GFlat, ADoubleFlat, BDoubleFlat, CDoubleFlat, CFive]);
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

        // Slashes.

        assert_eq!(Chord::new(C).with_slash(D).chord(), vec![DThree, C, E, G]);
        assert_eq!(Chord::new(CFive).with_slash(D).chord(), vec![DFour, CFive, EFive, GFive]);

        // Inversions.

        assert_eq!(C.into_chord().with_inversion(1).chord(), vec![E, G, CFive]);
        assert_eq!(C.into_chord().with_inversion(2).chord(), vec![G, CFive, EFive]);
        assert_eq!(C.into_chord().maj7().with_inversion(3).chord(), vec![B, CFive, EFive, GFive]);
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
    }

    #[test]
    fn test_guess() {
        assert_eq!(Chord::from_notes(&[EThree, C, EFlat, FSharp, ASharp, DFive]).unwrap().first().unwrap().chord(), Chord::parse("Cm9b5/E").unwrap().chord());
    }
}