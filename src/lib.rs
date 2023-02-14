//! A library to easily explore music theory principles.
//!
//! # Examples
//!
//! ```
//! use klib::core::known_chord::KnownChord;
//! use klib::core::modifier::Degree;
//! use klib::core::note::*;
//! use klib::core::chord::*;
//!
//! // Check to see what _kind_ of chord this is.
//! assert_eq!(Chord::new(C).augmented().seven().known_chord(), KnownChord::AugmentedDominant(Degree::Seven));
//!
//! ```
//!
//! ```
//! use klib::core::base::Parsable;
//! use klib::core::note::*;
//! use klib::core::chord::*;
//!
//! // Parse a chord from a string, and inspect the scale.
//! assert_eq!(Chord::parse("Cm7b5").unwrap().scale(), vec![C, D, EFlat, F, GFlat, AFlat, BFlat]);
//! ```
//!
//! ```
//! use klib::core::note::*;
//! use klib::core::chord::*;
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
#![feature(no_coverage)]
#![feature(int_roundings)]

pub mod core;

#[cfg(any(feature = "analyze", feature = "analyze_base", feature = "analyze_mic", feature = "analyze_file"))]
pub mod analyze;

#[cfg(any(feature = "ml", feature = "ml_base", feature = "ml_gather", feature = "ml_train", feature = "ml_infer"))]
pub mod ml;

#[cfg(any(feature = "analyze", feature = "analyze_base", feature = "analyze_mic", feature = "analyze_file"))]
pub use rodio;
