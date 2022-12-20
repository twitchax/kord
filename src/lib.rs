//! A library to easily explore music theory principles.
//! 
//! # Examples
//! 
//! ```
//! use klib::known_chord::KnownChord;
//! use klib::modifier::Degree;
//! use klib::note::*;
//! use klib::chord::*;
//! 
//! // Check to see what _kind_ of chord this is.
//! assert_eq!(Chord::new(C).augmented().seven().known_chord(), KnownChord::AugmentedDominant(Degree::Seven));
//! 
//! // Parse a chord from a string, and inspect the scale.
//! assert_eq!(Chord::parse("Cm7b5").unwrap().scale(), vec![C, D, EFlat, F, GFlat, AFlat, BFlat]);
//! 
//! // From a note, create a chord, and look at the chord tones.
//! assert_eq!(C.into_chord().augmented().major7().chord(), vec![C, E, GSharp, B]);
//! ```

#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#![feature(concat_idents)]
#![feature(iter_advance_by)]

pub mod base;
pub mod octave;
pub mod pitch;
pub mod named_pitch;
pub mod note;
pub mod interval;
pub mod modifier;
pub mod known_chord;
pub mod chord;
pub mod parser;