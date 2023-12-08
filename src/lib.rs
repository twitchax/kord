//! A library to easily explore music theory principles.
//!
//! # Examples
//!
//! ```
//! use klib::core::chord::*;
//! use klib::core::known_chord::KnownChord;
//! use klib::core::modifier::Degree;
//! use klib::core::note::*;
//!
//! // Check to see what _kind_ of chord this is.
//! assert_eq!(
//!     Chord::new(C).augmented().seven().known_chord(),
//!     KnownChord::AugmentedDominant(Degree::Seven)
//! );
//! ```
//!
//! ```
//! use klib::core::base::Parsable;
//! use klib::core::chord::*;
//! use klib::core::note::*;
//!
//! // Parse a chord from a string, and inspect the scale.
//! assert_eq!(
//!     Chord::parse("Cm7b5").unwrap().scale(),
//!     vec![C, D, EFlat, F, GFlat, AFlat, BFlat]
//! );
//! ```
//!
//! ```
//! use klib::core::chord::*;
//! use klib::core::note::*;
//!
//! // From a note, create a chord, and look at the chord tones.
//! assert_eq!(
//!     C.into_chord().augmented().major7().chord(),
//!     vec![C, E, GSharp, B]
//! );
//! ```

#![warn(rustdoc::broken_intra_doc_links, rust_2018_idioms, clippy::all, missing_docs)]
#![allow(incomplete_features)]
#![allow(clippy::needless_range_loop)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#![feature(concat_idents)]
#![feature(iter_advance_by)]
#![feature(int_roundings)]
#![feature(coverage_attribute)]

pub mod core;
pub mod helpers;

#[cfg(feature = "analyze_base")]
pub mod analyze;

#[cfg(feature = "ml_base")]
pub mod ml;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "analyze_base")]
pub use rodio;
