use crate::{
    note::NotePitch,
    scales::{interval::Interval, tet12::A4},
};

/// 12-tone equal temperament system and related scales.
///
/// Contains scale implementations and pitch manipulation functions.
pub mod tet12;

pub mod interval;

pub use tet12::modes::*;

/// A trait for musical scales that can generate pitches from scale degrees.
///
/// Musical scales are sequences of pitches that follow specific interval patterns.
/// This trait provides a common interface for different scale types (major, minor,
/// modes, etc.) to generate pitches based on scale degree numbers.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// let c_major = MajorScale(C4);
/// let first_degree = c_major.get_degree(1);  // C4
/// let fifth_degree = c_major.get_degree(5);  // G4
///
/// // Get multiple degrees at once
/// let triad = c_major.get_degrees([1, 3, 5]); // C-E-G chord
/// ```
pub trait Scale {
    fn intervals() -> &'static [Interval];

    /// Gets the pitch at the specified scale degree.
    ///
    /// Scale degrees are typically numbered starting from 1 (the root/tonic).
    /// Negative degrees and degrees beyond the scale length are supported
    /// and will wrap appropriately with octave transposition.
    /// The zeroth degree is considered the same as the first degree.
    ///
    /// # Parameters
    /// - `degree`: The scale degree (1 = root, 2 = second, etc.)
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let scale = MajorScale(C4);
    /// let root = scale.get_degree(1);     // C4
    /// let octave = scale.get_degree(8);   // C5 (octave higher)
    /// let below = scale.get_degree(-1);    // B3 (below root)
    /// ```
    fn get_degree(&self, degree: isize) -> NotePitch;

    /// Gets multiple pitches at once from an array of scale degrees.
    ///
    /// This is a convenience method for getting several scale degrees
    /// simultaneously, useful for constructing chords or arpeggios.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let scale = MajorScale(C4);
    /// let major_triad = scale.get_degrees([1, 3, 5]);      // C-E-G
    /// let major_seventh = scale.get_degrees([1, 3, 5, 7]); // C-E-G-B
    /// ```
    fn get_degrees<const N: usize>(&self, degrees: [isize; N]) -> [NotePitch; N] {
        degrees.map(|degree| self.get_degree(degree))
    }

    fn get_chord(&self, degrees: &[isize]) -> crate::note::chord::Chord {
        let pitches = degrees.iter().map(|&degree| self.get_degree(degree)).collect();
        crate::note::chord::Chord(pitches)
    }
}
