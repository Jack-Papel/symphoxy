use std::ops::Add;

use crate::scales::interval::ChordShape;
use crate::{Line, Note, NoteKind, NotePitch, Piece, Scale, Tet12, C4};

/// Represents a musical chord - a collection of pitches played simultaneously.
///
/// A chord contains multiple `NotePitch` values that can be played together
/// to create harmony. Chords can be constructed from individual pitches,
/// scale degrees, or common chord shapes.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// // Create a chord from individual pitches
/// let c_major = Chord::new([
///     NotePitch::new(261.63), // C4
///     NotePitch::new(329.63), // E4  
///     NotePitch::new(392.00), // G4
/// ]);
///
/// // Create from scale degrees (C major scale, root + third + fifth)
/// let scale = MajorScale(C4); // C major scale
/// let c_major = Chord::from_degrees(&scale, &[1, 3, 5]);
///
/// // Create common chord shapes
/// let major_shape = Chord::shape_from_semitone_offsets([4, 7]); // +4 and +7 semitones from root
/// let a_major = major_shape.transpose_to(A4);
/// ```
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Chord(pub Vec<NotePitch>);

impl Chord {
    /// Creates a new chord from an iterator of pitches.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let pitches = vec![C4, NotePitch::new(329.63), NotePitch::new(392.00)];
    /// let chord = Chord::new(pitches);
    /// ```
    pub fn new(pitches: impl IntoIterator<Item = NotePitch>) -> Self {
        Chord(pitches.into_iter().collect())
    }

    /// Creates a chord from scale degrees.
    ///
    /// Given a scale and an array of degree numbers, creates a chord
    /// containing the pitches at those scale degrees.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let scale = MajorScale(C4); // C major scale
    /// let c_major_triad = Chord::from_degrees(&scale, &[1, 3, 5]); // C-E-G
    /// let c_major_seventh = Chord::from_degrees(&scale, &[1, 3, 5, 7]); // C-E-G-B
    /// ```
    pub fn from_degrees(scale: &impl Scale, degrees: &[isize]) -> Self {
        let pitches = degrees.iter().map(|&degree| scale.get_degree(degree)).collect();
        Chord(pitches)
    }

    /// Plays all notes in the chord simultaneously as a piece.
    ///
    /// Takes a function that converts a single pitch into a line of music,
    /// then applies it to each pitch in the chord and combines them into
    /// a piece where all lines play at the same time.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let chord = Chord::new([C4, NotePitch::new(329.63), NotePitch::new(392.00)]);
    ///
    /// // Strike all notes as quarter notes with piano timbre
    /// let piece = chord.strike(|pitch| Line::from(piano(quarter(pitch))));
    ///
    /// // Or with a more complex pattern
    /// let piece = chord.strike(|pitch| {
    ///     piano(quarter(pitch)) + piano(eighth(pitch)) + piano(eighth(REST))
    /// });
    /// ```
    pub fn strike(&self, striker: fn(NotePitch) -> Line) -> Piece {
        Piece(self.0.iter().map(|&pitch| striker(pitch)).collect())
    }

    /// Transposes the chord to a new target pitch.
    /// If the chord is empty, it returns a clone of itself.
    /// The transposition is done by scaling the pitches so that the lowest pitch matches the target pitch.
    #[expect(clippy::missing_panics_doc, reason = "Won't panic, manual check")]
    pub fn transpose_to(&self, target: NotePitch) -> Self {
        if self.0.is_empty() {
            return self.clone();
        }
        let offset = target.0 / self.0.iter().map(|p| p.0).reduce(f32::min).unwrap();
        Chord(self.0.iter().map(|&pitch| NotePitch(pitch.0 * offset)).collect())
    }
}

// From implementations for ergonomic chord creation
impl From<Vec<NotePitch>> for Chord {
    fn from(pitches: Vec<NotePitch>) -> Self {
        Chord(pitches)
    }
}

impl From<NotePitch> for Chord {
    fn from(pitch: NotePitch) -> Self {
        Chord(vec![pitch])
    }
}

impl<const N: usize> From<[NotePitch; N]> for Chord {
    fn from(pitches: [NotePitch; N]) -> Self {
        Chord(pitches.to_vec())
    }
}

// AsRef and Deref implementations for ergonomic access
impl AsRef<Vec<NotePitch>> for Chord {
    fn as_ref(&self) -> &Vec<NotePitch> {
        &self.0
    }
}

use std::ops::Deref;

impl Deref for Chord {
    type Target = Vec<NotePitch>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Display implementation for better debugging
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Chord {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.0.is_empty() {
            write!(f, "Empty chord")
        } else {
            write!(f, "Chord[")?;
            for (i, pitch) in self.0.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{pitch:?}")?;
            }
            write!(f, "]")
        }
    }
}
