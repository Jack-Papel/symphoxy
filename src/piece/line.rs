use std::{
    iter::{Product, Sum},
    ops::{Add, Mul, Neg, Not},
};

use crate::{
    note::{NoteKind, NoteLength},
    Note,
};

use super::Piece;

/// Represents a sequence of musical notes played one after another (melody/rhythm).
///
/// A `Line` is a linear sequence of notes that represents a single melodic or
/// rhythmic line. Lines can be concatenated with `+` to create longer sequences,
/// and multiple lines can be combined into a `Piece` with `*` to play them
/// simultaneously.
///
/// ## Pickup Notes
/// Lines support "pickup" notes - notes that are played before the main sequence
/// when the line is concatenated to another line. This is useful for musical
/// phrases that begin before the main beat.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// let [c4, d4, e4, g4, a4, b4] = MajorScale(C4).get_degrees([1, 2, 3, 5, 6, 7]);
///
/// // Create a simple melody
/// let melody = piano(quarter(c4)) + piano(quarter(d4)) + piano(half(e4));
///
/// // Create a line with pickup notes
/// let mut line_with_pickup = piano(quarter(g4)) + piano(quarter(a4));
/// line_with_pickup.pickup = vec![piano(eighth(b4))]; // Pickup before the line
///
/// // Combine lines
/// // The last note of the previous line is truncated to fit the pickup
/// let longer_melody = melody + line_with_pickup;
/// ```
/// You can also create a pickup using a slightly different syntax:
/// ```
/// use symphoxy::prelude::*;
///
/// let [c4, d4, e4, g4, a4, b4] = MajorScale(C4).get_degrees([1, 2, 3, 5, 6, 7]);
/// // Create a line with a pickup that holds into the first note
/// // The `-` operator makes the line a pickup line, and the `!` operator
/// // indicates that the pickup should be held into the first note of the main sequence.
/// let mut line_with_pickup = -!piano(eighth(b4) + eighth(g4)) + piano(quarter(g4)) + piano(quarter(a4));
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Line {
    /// The main sequence of notes in the line
    pub notes: Vec<Note>,
    /// Notes played before the main sequence when this line follows another
    pub pickup: Vec<Note>,
    /// Whether the pickup should be held into the first note of the main sequence
    pub hold_pickup: bool,
}

impl Line {
    /// Creates a new empty line.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let empty_line = Line::new();
    /// assert_eq!(empty_line.notes.len(), 0);
    /// ```
    pub fn new() -> Line {
        Line::default()
    }
    /// Extends the line by adding a rest of the specified duration.
    ///
    /// This is mostly used internally for convenience, but can also be used
    /// to add rests to a melody or rhythm line.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let melody = piano(quarter(C4)) + piano(quarter(A4));
    /// let extended = melody.extend(4); // Add a quarter rest (4 time units)
    /// ```
    pub fn extend(&self, extend_by: u16) -> Self {
        if extend_by == 0 {
            return self.clone();
        }
        #[expect(clippy::arithmetic_side_effects, reason = "User is expected to handle this error")]
        return self.clone() + Note(NoteLength(extend_by), NoteKind::Rest);
    }
    /// Returns the total duration of the line in time units.
    ///
    /// This sums up the durations of all notes in the main sequence.
    /// Pickup notes are not included in this calculation.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let line = piano(quarter(C4)) + piano(half(A4)); // 4 + 8 = 12 time units
    /// assert_eq!(line.length(), 12);
    /// ```
    pub fn length(&self) -> usize {
        self.notes.iter().map(|note| note.0 .0 as usize).sum()
    }

    /// Creates a new line with all notes set to the specified volume.
    ///
    /// This sets the volume of all pitched notes to the given volume.
    /// Rest notes are unaffected.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let loud_line = piano(quarter(C4)) + piano(quarter(A4));
    /// let quiet_line = loud_line.volume(0.5); // Half volumebeats
    /// let very_loud = loud_line.volume(2.0);  // Double volume
    /// ```
    pub fn volume(&self, volume: f32) -> Line {
        Line {
            notes: self.notes.iter().map(|note| note.volume(volume)).collect(),
            pickup: self.pickup.iter().map(|note| note.volume(volume)).collect(),
            hold_pickup: self.hold_pickup,
        }
    }

    /// Gets the note that starts playing at a specific time instant.
    ///
    /// Returns an iterator containing the note that begins at the specified
    /// time point, or an empty iterator if no note starts at that instant.
    /// This is useful for timing-based analysis or custom playback systems.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let line = piano(quarter(C4) + half(A4)); // C4 at 0, A4 at 4
    ///
    /// let notes_at_0: Vec<_> = line.get_notes_at_instant(0).collect();
    /// assert_eq!(notes_at_0.len(), 1); // C4 starts at time 0
    ///
    /// let notes_at_4: Vec<_> = line.get_notes_at_instant(4).collect();  
    /// assert_eq!(notes_at_4.len(), 1); // D4 starts at time 4
    ///
    /// let notes_at_2: Vec<_> = line.get_notes_at_instant(2).collect();
    /// assert_eq!(notes_at_2.len(), 0); // No note starts at time 2
    /// ```
    #[expect(clippy::arithmetic_side_effects, reason = "Manual bounds checking, almost always safe")]
    pub fn get_notes_at_instant(&self, instant: usize) -> impl Iterator<Item = Note> {
        let mut time_acc = 0;
        for note in self.notes.clone() {
            if time_acc == instant {
                return Some(note).into_iter();
            }
            time_acc += note.0 .0 as usize
        }

        None.into_iter()
    }
}

impl Neg for Line {
    type Output = Line;

    fn neg(self) -> Self::Output {
        Self {
            notes: vec![],
            pickup: self.notes,
            hold_pickup: self.hold_pickup,
        }
    }
}

impl Not for Line {
    type Output = Line;

    fn not(self) -> Self::Output {
        Self {
            hold_pickup: true,
            ..self
        }
    }
}

impl From<Note> for Line {
    fn from(value: Note) -> Self {
        Line::from(vec![value])
    }
}

impl From<Vec<Note>> for Line {
    fn from(notes: Vec<Note>) -> Line {
        Line {
            notes,
            pickup: vec![],
            hold_pickup: false,
        }
    }
}

impl<const N: usize> From<[Note; N]> for Line {
    fn from(notes: [Note; N]) -> Self {
        Line::from(notes.to_vec())
    }
}

// AsRef implementations for ergonomic access
impl AsRef<Vec<Note>> for Line {
    fn as_ref(&self) -> &Vec<Note> {
        &self.notes
    }
}

// Display implementation for debugging
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Line[{} notes", self.notes.len())?;
        if !self.pickup.is_empty() {
            write!(f, ", {} pickup", self.pickup.len())?;
        }
        write!(f, "]")
    }
}

impl Add<Piece> for Line {
    type Output = Piece;

    /// This implementation puts this line as the first line of the piece
    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    #[expect(clippy::cast_possible_truncation, reason = "I don't want to deal with this right now")]
    fn add(self, rhs: Piece) -> Self::Output {
        if !rhs.0.is_empty() {
            let mut piece = rhs.clone();
            let self_len = self.length();

            piece.0[0] = self + piece.0[0].clone();
            for line_no in 1..piece.0.len() {
                piece.0[line_no] = Line::new().extend(self_len as u16) + piece.0[line_no].clone()
            }

            piece
        } else {
            self.into()
        }
    }
}

impl Add<Note> for Line {
    type Output = Line;

    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    fn add(self, rhs: Note) -> Self::Output {
        self + Line::from(rhs)
    }
}

impl Sum<Note> for Line {
    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    fn sum<I: Iterator<Item = Note>>(iter: I) -> Self {
        iter.fold(Line::new(), |line, note| line + note)
    }
}

impl Add<Line> for Line {
    type Output = Line;

    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    #[expect(clippy::cast_possible_truncation, reason = "Manual Bounds Checking")]
    fn add(self, rhs: Line) -> Self::Output {
        let mut notes = self.notes.clone();

        let mut pickup_line = Line::from(rhs.pickup);
        let pickup_length = pickup_line.length();

        let mut time_removed = 0;
        let mut notes_to_remove = 0;
        let mut note_to_add = None;
        for note in notes.iter().rev() {
            if pickup_length <= time_removed {
                break;
            }

            if pickup_length >= time_removed + note.0 .0 as usize {
                time_removed += note.0 .0 as usize;
                notes_to_remove += 1;
            } else {
                // Need to remove part of a note
                notes_to_remove += 1;
                note_to_add = Some(Note(
                    NoteLength(note.0 .0 - (pickup_length - time_removed) as u16),
                    note.1,
                ));
                break;
            }
        }

        for _ in 0..notes_to_remove {
            notes.pop();
        }

        if let Some(note) = note_to_add {
            notes.push(note);
        }

        notes.append(&mut pickup_line.notes);

        let mut rhs_notes = rhs.notes;

        if rhs.hold_pickup {
            if let Some(last_note) = notes.iter().last() {
                let last_index = notes.len() - 1;

                notes[last_index] = Note(NoteLength(last_note.0 .0 + rhs_notes[0].0 .0), last_note.1);

                rhs_notes.remove(0);
            }
        }

        Line {
            notes: [notes, rhs_notes].concat(),
            pickup: self.pickup,
            hold_pickup: self.hold_pickup,
        }
    }
}

impl Sum<Line> for Line {
    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    fn sum<I: Iterator<Item = Line>>(iter: I) -> Self {
        iter.fold(Line::new(), |line, next_line| line + next_line)
    }
}

impl Mul<usize> for Line {
    type Output = Line;

    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    fn mul(self, rhs: usize) -> Self::Output {
        let mut current_line = self.clone();

        for _ in 0..(rhs - 1) {
            current_line = current_line + self.clone();
        }

        current_line
    }
}

impl Mul<Line> for Line {
    type Output = Piece;

    fn mul(self, rhs: Line) -> Self::Output {
        Piece(vec![self, rhs])
    }
}

impl Mul<Note> for Line {
    type Output = Piece;

    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    fn mul(self, rhs: Note) -> Self::Output {
        self * Line::from(rhs)
    }
}
