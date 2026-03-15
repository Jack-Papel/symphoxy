//! # Symphoxy - Music as Code
//!
//! Symphoxy is a flexible, powerful music composition library that treats music as code.
//! Create complex musical compositions using an intuitive, simple API.
//!
//! ## Core Concepts
//!
//! - **Notes**: Individual musical sounds with pitch, duration, timbre, and volume
//! - **Lines**: Sequences of notes played one after another (melodies/rhythms)  
//! - **Pieces**: Multiple lines played simultaneously (harmony/polyphony)
//! - **Chords**: Groups of pitches played together
//!
//! ## Quick Start
//!
//! ```rust
//! use symphoxy::prelude::*;
//!
//! // Create individual notes
//! let c4 = C4;
//! let d4 = NotePitch::new(293.66); // Either manually specify frequency...
//! let e4 = d4.semitone(1);         // ...or use semitone offsets
//!
//! // Build melodies by adding notes together
//! let melody = piano(quarter(c4) + quarter(d4) + half(e4)); // C-D-E
//!
//! // Create harmony by stacking lines with *
//! let bass_line = bass(whole(C4.octave(-1))); // C3
//! let piece = melody * bass_line;
//!
//! // Create chords
//! let c_major = Chord::from_degrees(&MajorScale(C4), &[1, 3, 5]);
//! let chord_piece = piano(quarter(c_major));
//! ```
//!
//! ## Key Features
//!
//! ### Fluent API
//! Chain operations naturally:
//! ```rust
//! use symphoxy::prelude::*;
//!
//! // Create a melody with two instruments
//! let complex_line = piano(dotted(quarter)(C4))
//!     .volume(0.8) +
//!     electric_guitar(tie(eighth, sixteenth)(A4))
//!     .volume(1.2);
//! ```
//!
//! ### Rich Timbres
//! Built-in instrument sounds:
//! ```rust
//! use symphoxy::prelude::*;
//!
//! let piano_note = piano(quarter(C4));
//! let guitar_note = electric_guitar(quarter(C4));
//! let bass_note = bass(quarter(C4));
//! // Different notes have different drum kit sounds
//! let drum_hit = drums(quarter(C4.octave(1))); // Kick drum at C5
//! ```
//!
//! ### Flexible Note Lengths
//! Standard and custom durations:
//! ```rust
//! use symphoxy::prelude::*;
//!
//! let notes = piano(whole(C4)) + piano(half(C4)) + piano(quarter(C4)) + piano(eighth(C4));
//! let dotted_half = piano(dotted(half)(C4));
//! let tied_notes = piano(tie(quarter, eighth)(C4));
//! ```
//!
//! ### Chord Support
//! ```rust
//! use symphoxy::prelude::*;
//!
//! // Build chords from pitches
//! let chord = Chord::new([C4, NotePitch::new(329.63), NotePitch::new(392.00)]);
//!
//! // Play all notes simultaneously  
//! let simultaneous = piano(quarter(chord.clone()));
//!
//! // You can also use a "striker" function to play it with a specific pattern
//! fn striker_fn(pitch: NotePitch) -> Line {
//!     piano(quarter(pitch) + eighth(REST) + eighth(pitch))
//! }
//!
//! let simultaneous_patterned = chord.strike(striker_fn);
//!
//! // Or arpeggiate
//! let arpeggio_notes: Vec<Note> = chord.0.into_iter()
//!     .map(|pitch| piano(eighth(pitch)))
//!     .collect();
//! let arpeggio = Line::from(arpeggio_notes);
//! ```
//!
//! ## Architecture
//!
//! The library follows a composable design:
//!
//! 1. **Core Types**: `Note`, `NotePitch`, `NoteLength`, `Timbre`
//! 2. **Collections**: `Line` (sequential), `Piece` (simultaneous), `Chord` (harmonic)
//! 3. **Traits**: `LengthFluid`, `TimbreFluid`, `ChordFluid` for flexibility
//! 4. **Scales**: Support for different musical scales and tuning systems
//! 5. **Instruments**: Guitar fret/tuning support and other instrument-specific tools
//!
//! ## Features
//!
//! - `interactive-tui`: Interactive terminal interface for playback and file export
//! - `wav-output`: Export compositions to WAV audio files  
//! - `live-output`: Real-time audio playback
//!
//! ## Philosophy
//!
//! Rather than relying heavily on traditional music notation, Symphoxy embraces a
//! "piano roll" approach where music is constructed programmatically. This makes it
//! accessible to developers while remaining powerful enough for complex compositions.
//!
//! The design prioritizes:
//! - **Ergonomics**: Intuitive, readable code
//! - **Flexibility**: Multiple ways to express the same musical ideas
//! - **Composability**: Building complex pieces from simple parts
//! - **Performance**: Zero-cost abstractions where possible

#![deny(clippy::arithmetic_side_effects)]
#![warn(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
#![deny(clippy::allow_attributes_without_reason)]
#![deny(clippy::allow_attributes)]
#![warn(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::doc_comment_double_space_linebreaks,
    clippy::doc_include_without_cfg,
    clippy::doc_link_code,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::too_long_first_doc_paragraph,
    clippy::unnecessary_safety_doc
)]
#![warn(clippy::cargo_common_metadata)]

#[cfg(all(feature = "interactive-tui", not(any(feature = "wav-output", feature = "live-output"))))]
compile_error!("The `interactive-tui` feature requires either the `wav-output` or `live-output` feature to be enabled. Please enable one of them in your Cargo.toml.");

/// Instrument-specific tools and utilities.
///
/// Contains guitar fretting tools, tuning systems, and other instrument helpers.
pub mod instrument_tools;

#[cfg(all(feature = "interactive-tui", any(feature = "wav-output", feature = "live-output")))]
mod interactive;

/// Musical note types, timbres, lengths, and related functionality.
///
/// Contains `Note`, `NotePitch`, `NoteLength`, `Timbre`, and `Chord`.
pub mod note;

/// Core musical composition types and functions.
///
/// Contains `Piece` and `Line` for structuring musical compositions.
pub mod piece;

#[cfg(any(feature = "wav-output", feature = "live-output"))]
mod play;

/// Musical scales and tuning systems.
///
/// Contains the `Scale` trait and implementations for various musical scales.
pub mod scales;

pub use instrument_tools::strings::{Frets, GuitarFrets, GuitarTuning, StringTuning};
pub use note::chord::Chord;
pub use note::{bass, drums, electric_guitar, piano, sine};
pub use note::{dotted, double_whole, eighth, half, quarter, sixteenth, tie, whole};
pub use note::{LengthFluid, TimbreFluid};
pub use note::{Note, NoteKind, NoteLength, NotePitch, Timbre, REST};
pub use piece::line::Line;
pub use piece::Piece;
pub use scales::interval::ChordShape;
pub use scales::tet12::{get_note_name, get_note_name_with_octave, Tet12, A4, C4};
pub use scales::Scale;

/// Commonly used types and functions for music composition.
///
/// Import this module to get access to all the essential types and functions
/// needed for creating music with Symphoxy:
///
/// ```rust
/// use symphoxy::prelude::*;
///
/// // Now you have access to all core functionality
/// let melody = piano(quarter(C4)) + piano(quarter(A4));
/// let piece = melody * bass(half(C4));
/// ```
pub mod prelude {
    pub use crate::instrument_tools::strings::*;
    pub use crate::note::chord::*;
    pub use crate::note::*;
    pub use crate::scales::*;
    pub use crate::{Line, Piece};
    pub use crate::{Note, NoteKind, NotePitch, REST};
    pub use crate::{Scale, Tet12};
    pub use crate::{A4, C4};
}

#[cfg(all(feature = "interactive-tui", any(feature = "wav-output", feature = "live-output")))]
pub use crate::interactive::InteractiveTui;

#[cfg(any(feature = "wav-output", feature = "live-output"))]
pub use crate::play::MusicPlayer;
