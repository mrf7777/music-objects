#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/music-objects/0.1.0")]

//! Complete western music theory library.
//!
//! Provides several objects that make representing most western music theory concepts
//! possible. The objects that you will find here are able to represent vertical and horizontal concepts.
//! This lets you describe harmony and rhythm respectively.
//!
//! # Example
//!
//! ```rust,editable
//! use music_objects::{pitch, interval};
//!
//! fn main() {
//!     let middle_c = pitch::NotePitch::new(pitch::NotePitchClass::C, 4);
//!     let e4 = pitch::NotePitch::new(pitch::NotePitchClass::E, 4);
//!     let major_3rd_up = interval::DirectedSemitoneInterval::new(interval::SemitoneInterval::new(4), interval::Direction::Up);
//!
//!     assert_eq!(major_3rd_up.apply_to_note_pitch(&middle_c).unwrap(), e4);
//! }
//! ```
//!
//! # Cargo features
//!
//! This library provides an optional `serde` feature; when enabled, you can serialize and deserialize all
//! of the data structures in this crate.

/// Composition objects.
pub mod composition;

/// Harmony objects; contains constructs for chords.
pub mod harmony;

/// Intervals between pitches.
pub mod interval;

/// Objects for notes which are expressions of pitch in a composition.
pub mod note;

/// Pitch, tuning, and labeled pitches.
pub mod pitch;

/// Tempo, metre, and compound rhythms.
pub mod rhythm;
