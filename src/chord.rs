
// Traits.

use std::collections::HashSet;

use crate::{note::{Note, CZero}, modifier::{Modifier, Extension, Dominant}, known_chord::{KnownChord, HasRelativeChord, HasRelativeScale}, interval::Interval, base::{HasDescription, HasName, HasStaticName}};

pub trait HasRoot {
    fn root(&self) -> Note;
}

pub trait HasBase {
    fn base(&self) -> Note;
}

pub trait HasShapeModifiers {
    fn shape_modifiers(&self) -> &HashSet<Modifier>;
}

pub trait HasExtensions {
    fn extensions(&self) -> &HashSet<Extension>;
}

pub trait HasInversion {
    fn inversion(&self) -> u8;
}

pub trait HasKnownChord {
    fn known_chord(&self) -> KnownChord;
}

pub trait HasScale {
    fn scale(&self) -> Vec<Note>;
}

pub trait HasChord {
    fn chord(&self) -> Vec<Note>;
}

pub trait Chordable {
    fn with_modifier(self, modifier: Modifier) -> Chord;
    fn with_extension(self, extension: Extension) -> Chord;
    fn with_inversion(self, inversion: u8) -> Chord;

    // Modifiers.

    fn minor(self) -> Chord;
    fn min(self) -> Chord;

    fn flat5(self) -> Chord;

    fn augmented(self) -> Chord;
    fn aug(self) -> Chord;

    fn major7(self) -> Chord;
    fn maj7(self) -> Chord;

    fn dominant7(self) -> Chord;
    fn seven(self) -> Chord;
    fn dominant9(self) -> Chord;
    fn nine(self) -> Chord;
    fn dominant11(self) -> Chord;
    fn eleven(self) -> Chord;
    fn dominant13(self) -> Chord;
    fn thirteen(self) -> Chord;
    fn dominant(self, dominant: Dominant) -> Chord;

    fn flat9(self) -> Chord;
    fn sharp9(self) -> Chord;

    fn sharp11(self) -> Chord;

    // Special.

    fn diminished(self) -> Chord;
    fn dim(self) -> Chord;

    fn half_diminished(self) -> Chord;
    fn half_dim(self) -> Chord;

    // Extensions.

    fn sus2(self) -> Chord;
    fn sus4(self) -> Chord;
    fn sustain(self) -> Chord;
    fn sus(self) -> Chord;

    fn flat11(self) -> Chord;

    fn flat13(self) -> Chord;
    fn sharp13(self) -> Chord;

    fn add2(self) -> Chord;
    fn add4(self) -> Chord;
    fn add6(self) -> Chord;

    fn add9(self) -> Chord;
    fn add11(self) -> Chord;
    fn add13(self) -> Chord;
}

// Struct.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Chord {
    root: Note,
    slash: Option<Note>,
    modifiers: HashSet<Modifier>,
    extensions: HashSet<Extension>,
    inversion: u8,
}

// Impls.


impl Chord {
    pub fn new(root: Note) -> Self {
        Self { 
            root, 
            slash: None, 
            modifiers: HashSet::new(), 
            extensions: HashSet::new(), 
            inversion: 0 
        }
    }
}

impl HasName for Chord {
    fn name(&self) -> String {
        let mut name = String::new();

        name.push_str(self.root.static_name());

        name.push_str(self.known_chord().static_name());

        if !self.extensions.is_empty() {
            for e in self.extensions.iter() {
                name.push_str(&format!("({})", e.static_name()));
            }
        }

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

impl HasBase for Chord {
    fn base(&self) -> Note {
        self.slash.unwrap_or(self.root)
    }
}

impl HasShapeModifiers for Chord {
    fn shape_modifiers(&self) -> &HashSet<Modifier> {
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

impl Chordable for Chord {
    fn with_modifier(mut self, modifier: Modifier) -> Chord {
        self.modifiers.insert(modifier);

        self
    }

    fn with_extension(mut self, extension: Extension) -> Chord {
        self.extensions.insert(extension);

        self
    }

    fn with_inversion(mut self, inversion: u8) -> Chord {
        self.inversion = inversion;

        self
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

    fn augmented(self) -> Chord {
        self.with_modifier(Modifier::Augmented5)
    }

    fn aug(self) -> Chord {
        self.augmented()
    }

    fn major7(self) -> Chord {
        self.with_modifier(Modifier::Major7)
    }

    fn maj7(self) -> Chord {
        self.major7()
    }

    fn dominant7(self) -> Chord {
        self.with_modifier(Modifier::Dominant(Dominant::Seven))
    }

    fn seven(self) -> Chord {
        self.dominant7()
    }

    fn dominant9(self) -> Chord {
        self.with_modifier(Modifier::Dominant(Dominant::Nine))
    }

    fn nine(self) -> Chord {
        self.dominant9()
    }

    fn dominant11(self) -> Chord {
        self.with_modifier(Modifier::Dominant(Dominant::Eleven))
    }

    fn eleven(self) -> Chord {
        self.dominant11()
    }

    fn dominant13(self) -> Chord {
        self.with_modifier(Modifier::Dominant(Dominant::Thirteen))
    }

    fn thirteen(self) -> Chord {
        self.dominant13()
    }

    fn dominant(self, dominant: Dominant) -> Chord {
        self.with_modifier(Modifier::Dominant(dominant))
    }

    fn flat9(self) -> Chord {
        self.with_modifier(Modifier::Flat9)
    }

    fn sharp9(self) -> Chord {
        self.with_modifier(Modifier::Sharp9)
    }

    fn sharp11(self) -> Chord {
        self.with_modifier(Modifier::Sharp11)
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

    fn sus4(self) -> Chord {
        self.with_extension(Extension::Sus4)
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

    fn flat13(self) -> Chord {
        self.with_extension(Extension::Flat13)
    }

    fn sharp13(self) -> Chord {
        self.with_extension(Extension::Sharp13)
    }

    fn add2(self) -> Chord {
        self.with_extension(Extension::Add2)
    }

    fn add4(self) -> Chord {
        self.with_extension(Extension::Add4)
    }

    fn add6(self) -> Chord {
        self.with_extension(Extension::Add6)
    }

    fn add9(self) -> Chord {
        self.with_extension(Extension::Add9)
    }

    fn add11(self) -> Chord {
        self.with_extension(Extension::Add11)
    }

    fn add13(self) -> Chord {
        self.with_extension(Extension::Add13)
    }
}

impl HasKnownChord for Chord {
    fn known_chord(&self) -> KnownChord {
        let modifiers = &self.modifiers;
        let contains_dominant = modifiers.contains(&Modifier::Dominant(Dominant::Seven)) || modifiers.contains(&Modifier::Dominant(Dominant::Nine)) || modifiers.contains(&Modifier::Dominant(Dominant::Eleven)) || modifiers.contains(&Modifier::Dominant(Dominant::Thirteen));

        
        if modifiers.contains(&Modifier::Diminished) {
            KnownChord::Diminished
        } else if modifiers.contains(&Modifier::Minor) {
            if modifiers.contains(&Modifier::Major7) {
                return KnownChord::MinorMajor7;
            }

            if contains_dominant {
                if modifiers.contains(&Modifier::Flat5) {
                    return KnownChord::HalfDiminished;
                }

                return KnownChord::MinorDominant;
            }

            return KnownChord::Minor;
        } else {
            if modifiers.contains(&Modifier::Augmented5) {
                if modifiers.contains(&Modifier::Major7) {
                    return KnownChord::AugmentedMajor7;
                }

                if contains_dominant {
                    return KnownChord::AugmentedDominant;
                }

                return KnownChord::Augmented;
            }

            if self.modifiers.contains(&Modifier::Major7) {
                return KnownChord::Major7;
            }

            if contains_dominant {
                if modifiers.contains(&Modifier::Sharp11) {
                    return KnownChord::DominantSharp11;
                }
                
                if modifiers.contains(&Modifier::Flat9) {
                    return KnownChord::DominantFlat9;
                }
    
                if modifiers.contains(&Modifier::Sharp9) {
                    return KnownChord::DominantSharp9;
                }
                
                return KnownChord::Dominant;
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

        if modifiers.contains(&Modifier::Dominant(Dominant::Nine)) {
            result.push(Interval::MajorNinth);
        } else if modifiers.contains(&Modifier::Dominant(Dominant::Eleven)) {
            result.push(Interval::MajorNinth);
            result.push(Interval::PerfectEleventh);
        } else if modifiers.contains(&Modifier::Dominant(Dominant::Thirteen)) {
            result.push(Interval::MajorNinth);
            result.push(Interval::PerfectEleventh);
            result.push(Interval::MajorThirteenth);
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
        if let Some(slash) = self.slash {
            result.insert(0, slash);
        }

        result
    }
}

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};
    use crate::note::*;

    #[test]
    fn test_known_chords() {
        assert_eq!(Chord::new(C).known_chord(), KnownChord::Major);
        assert_eq!(Chord::new(C).minor().known_chord(), KnownChord::Minor);
        assert_eq!(Chord::new(C).major7().known_chord(), KnownChord::Major7);
        assert_eq!(Chord::new(C).minor().major7().known_chord(), KnownChord::MinorMajor7);
        assert_eq!(Chord::new(C).minor().seven().known_chord(), KnownChord::MinorDominant);
        assert_eq!(Chord::new(C).minor().eleven().known_chord(), KnownChord::MinorDominant);
        assert_eq!(Chord::new(C).seven().known_chord(), KnownChord::Dominant);
        assert_eq!(Chord::new(C).eleven().known_chord(), KnownChord::Dominant);
        assert_eq!(Chord::new(C).thirteen().known_chord(), KnownChord::Dominant);
        assert_eq!(Chord::new(C).diminished().known_chord(), KnownChord::Diminished);
        assert_eq!(Chord::new(C).dim().known_chord(), KnownChord::Diminished);
        assert_eq!(Chord::new(C).minor().seven().flat5().known_chord(), KnownChord::HalfDiminished);
        assert_eq!(Chord::new(C).augmented().known_chord(), KnownChord::Augmented);
        assert_eq!(Chord::new(C).augmented().major7().known_chord(), KnownChord::AugmentedMajor7);
        assert_eq!(Chord::new(C).augmented().seven().known_chord(), KnownChord::AugmentedDominant);
        assert_eq!(Chord::new(C).seven().sharp11().known_chord(), KnownChord::DominantSharp11);
        assert_eq!(Chord::new(C).seven().flat9().known_chord(), KnownChord::DominantFlat9);
        assert_eq!(Chord::new(C).seven().sharp9().known_chord(), KnownChord::DominantSharp9);

        assert_eq!(Chord::new(C).sus2().known_chord(), KnownChord::Major);
        assert_eq!(Chord::new(C).sus4().known_chord(), KnownChord::Major);
        assert_eq!(Chord::new(C).sustain().known_chord(), KnownChord::Major);
        assert_eq!(Chord::new(C).seven().sus().known_chord(), KnownChord::Dominant);
    }

    #[test]
    fn test_scales() {
        // Basic.

        assert_eq!(Chord::new(C).scale(), vec![C, D, E, F, G, A, B]);
        assert_eq!(Chord::new(C).minor().scale(), vec![C, D, EFlat, F, G, AFlat, BFlat]);
        assert_eq!(Chord::new(C).major7().scale(), vec![C, D, E, F, G, A, B]);
        assert_eq!(Chord::new(C).minor().major7().scale(), vec![C, D, EFlat, F, G, A, B]);
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
        assert_eq!(Chord::new(C).seven().sharp11().scale(), vec![C, D, E, FSharp, G, A, BFlat]);

        // Others.

        assert_eq!(Chord::new(DFlat).scale(), vec![DFlat, EFlat, F, GFlat, AFlat, BFlat, CFive]);
        assert_eq!(Chord::new(DFlat).seven().scale(), vec![DFlat, EFlat, F, GFlat, AFlat, BFlat, CFlat]);
        assert_eq!(Chord::new(DFlat).dim().scale(), vec![DFlat, EFlat, FFlat, GFlat, ADoubleFlat, BDoubleFlat, ADoubleSharp, CFive]);
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
        assert_eq!(Chord::new(C).augmented().chord(), vec![C, E, GSharp]);
        assert_eq!(Chord::new(C).augmented().major7().chord(), vec![C, E, GSharp, B]);
        assert_eq!(Chord::new(C).augmented().seven().chord(), vec![C, E, GSharp, BFlat]);
        assert_eq!(Chord::new(C).seven().sharp11().chord(), vec![C, E, G, BFlat, FSharpFive]);

        // Extensions.

        assert_eq!(Chord::new(C).nine().sus2().chord(), vec![C, D, G, BFlat, DFive]);
        assert_eq!(Chord::new(C).nine().sus4().chord(), vec![C, F, G, BFlat, DFive]);
        assert_eq!(Chord::new(C).nine().sustain().chord(), vec![C, F, G, BFlat, DFive]);
        assert_eq!(Chord::new(C).seven().sus().chord(), vec![C, F, G, BFlat]);
        assert_eq!(Chord::new(C).seven().add2().chord(), vec![C, D, E, G, BFlat]);
        assert_eq!(Chord::new(C).seven().add4().chord(), vec![C, E, F, G, BFlat]);
        assert_eq!(Chord::new(C).add6().chord(), vec![C, E, G, A]);
        assert_eq!(Chord::new(C).seven().add9().chord(), vec![C, E, G, BFlat, DFive]);
        assert_eq!(Chord::new(C).seven().add11().chord(), vec![C, E, G, BFlat, FFive]);
        assert_eq!(Chord::new(C).seven().add13().chord(), vec![C, E, G, BFlat, AFive]);
        assert_eq!(Chord::new(C).seven().add2().add4().chord(), vec![C, D, E, F, G, BFlat]);
        assert_eq!(Chord::new(C).seven().flat11().chord(), vec![C, E, G, BFlat, FFlatFive]);
        assert_eq!(Chord::new(C).seven().flat13().chord(), vec![C, E, G, BFlat, AFlatFive]);
        assert_eq!(Chord::new(C).seven().sharp13().chord(), vec![C, E, G, BFlat, ASharpFive]);

        // Inversions.

        assert_eq!(Chord::new(C).with_inversion(1).chord(), vec![E, G, CFive]);
        assert_eq!(Chord::new(C).with_inversion(2).chord(), vec![G, CFive, EFive]);

    }
}