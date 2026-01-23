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
//!     vec![C, DFlat, EFlat, F, GFlat, AFlat, BFlat]
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
//!
//! # Scales and Modes
//!
//! ```
//! use klib::core::base::HasName;
//! use klib::core::mode::*;
//! use klib::core::mode_kind::*;
//! use klib::core::note::*;
//! use klib::core::scale::*;
//! use klib::core::scale_kind::*;
//!
//! // Create a D Dorian mode
//! let mode = Mode::new(D, ModeKind::Dorian);
//! assert_eq!(mode.name(), "D dorian");
//!
//! // Create an A harmonic minor scale
//! let scale = Scale::new(A, ScaleKind::HarmonicMinor);
//! assert_eq!(scale.name(), "A harmonic minor");
//! ```
//!
//! # Notation (Unified Parsing)
//!
//! ```
//! use klib::core::base::Parsable;
//! use klib::core::notation::Notation;
//!
//! // Automatically detects and parses scales, modes, or chords
//! let scale = Notation::parse("C major pentatonic").unwrap();
//! assert!(scale.is_scale());
//!
//! let mode = Notation::parse("D dorian").unwrap();
//! assert!(mode.is_mode());
//!
//! let chord = Notation::parse("Cmaj7").unwrap();
//! assert!(chord.is_chord());
//! ```

#![warn(rustdoc::broken_intra_doc_links, rust_2018_idioms, clippy::all, missing_docs)]
#![allow(incomplete_features)]
#![allow(clippy::needless_range_loop)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
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

#[cfg(feature = "audio")]
pub use rodio;
