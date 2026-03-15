use std::{
    fmt::Write,
    iter::Sum,
    ops::{Add, Mul},
};

use itertools::{EitherOrBoth, Itertools};
use line::Line;

use crate::{
    note::{NoteKind, NotePitch, Timbre},
    scales::tet12::{self, A4, C4},
    Note, Tet12,
};

/// Line sequence types and functionality.
///
/// Contains the `Line` type for representing sequential note sequences.
pub mod line;

/// Represents a complete musical composition with multiple simultaneous parts.
///
/// A `Piece` contains multiple `Line`s that play simultaneously, creating
/// harmony and polyphony. This is the top-level structure for complete
/// musical compositions.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// let [c4, d4, e4, f4, g4] = MajorScale(C4).get_degrees([1, 2, 3, 4, 5]);
///
/// // Create individual lines
/// let melody = piano(quarter(c4)) + piano(quarter(d4)) + piano(half(e4));
/// let bass = bass(half(c4)) + bass(half(f4));
/// let chords = electric_guitar(whole(Chord::new([c4, e4, g4]))); // This is actually a piece
///
/// // Combine into a piece
/// let piece = Piece(vec![
///     melody.clone(),
///     bass.clone()
/// ]) * chords.clone();
///
/// // Or use the * operator to stack lines
/// let piece2 = melody * bass * chords;
/// ```
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Piece(pub Vec<Line>);

impl Piece {
    /// Creates a new empty piece.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let empty_piece = Piece::new();
    /// assert_eq!(empty_piece.0.len(), 0);
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new piece with all notes set to the specified volume.
    ///
    /// This applies the volume setting to every note in the piece,
    /// affecting all pitched notes while leaving rests unchanged.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let piece = piano(quarter(C4)) * bass(quarter(C4)); // Two lines playing together
    /// let quiet_piece = piece.volume(0.3); // 30% volume
    /// let loud_piece = piece.volume(1.5);  // 150% volume
    /// ```
    pub fn volume(&self, volume: f32) -> Self {
        Piece(self.0.iter().map(|line| line.volume(volume)).collect())
    }
}

impl From<Line> for Piece {
    fn from(value: Line) -> Self {
        Piece(vec![value])
    }
}

impl From<Note> for Piece {
    fn from(value: Note) -> Self {
        Piece(vec![Line::from(value)])
    }
}

// Additional From implementations for Piece ergonomics
impl From<Vec<Line>> for Piece {
    fn from(lines: Vec<Line>) -> Self {
        Piece(lines)
    }
}

impl<const N: usize> From<[Line; N]> for Piece {
    fn from(lines: [Line; N]) -> Self {
        Piece(lines.to_vec())
    }
}

impl Piece {
    /// Gets all notes that start playing at a specific time instant.
    ///
    /// Returns an iterator over all notes across all lines that begin
    /// at the specified time point. Useful for analysis or custom playback.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let piece = piano(quarter(C4)) * bass(quarter(A4)); // Both start at time 0
    /// let notes_at_start: Vec<_> = piece.get_notes_at_instant(0).collect();
    /// assert_eq!(notes_at_start.len(), 2); // Piano C4 and bass A4
    /// ```
    pub fn get_notes_at_instant(&self, instant: usize) -> impl Iterator<Item = Note> {
        self.0
            .clone()
            .into_iter()
            .flat_map(move |l| l.get_notes_at_instant(instant).collect::<Vec<_>>())
    }

    /// As opposed to `get_notes_at_instant`, this gets any note which would
    /// be playing during a given instant, rather than the notes which start at a given instant.
    #[expect(clippy::arithmetic_side_effects, reason = "Manual bounds checking, almost always safe")]
    pub fn get_notes_during_instant(&self, instant: usize) -> impl Iterator<Item = Note> {
        self.0.clone().into_iter().filter_map(move |l| {
            // get note at time
            let mut time_acc = 0;
            for note in l.notes.clone() {
                if time_acc <= instant && instant < time_acc + note.0 .0 as usize {
                    return Some(note);
                }
                time_acc += note.0 .0 as usize;
            }

            None
        })
    }

    /// Returns the total duration of the piece in time units.
    ///
    /// This is the length of the longest line in the piece, since all lines
    /// play simultaneously and the piece ends when the longest line finishes.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let short_line = piano(quarter(C4));           // 4 time units
    /// let long_line = piano(whole(C4));              // 16 time units  
    /// let piece = short_line * long_line;
    ///
    /// assert_eq!(piece.length(), 16); // Length of the longest line
    /// ```
    pub fn length(&self) -> usize {
        self.0.iter().map(|line| line.length()).max().unwrap_or_default()
    }
}

impl Mul<Piece> for Piece {
    type Output = Piece;

    fn mul(self, rhs: Piece) -> Self::Output {
        Piece([self.0, rhs.0].concat())
    }
}

impl Mul<usize> for Piece {
    type Output = Piece;

    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    fn mul(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            return Piece::new();
        }

        let mut acc = self.clone();
        for _ in 1..rhs {
            acc = acc + self.clone()
        }
        acc
    }
}

impl Add<Piece> for Piece {
    type Output = Piece;

    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    #[expect(clippy::cast_possible_truncation, reason = "I don't want to deal with this right now")]
    fn add(self, rhs: Piece) -> Self::Output {
        let self_length = self.length() as u16;
        let rhs_length = rhs.length() as u16;
        Piece(
            self.0
                .into_iter()
                .zip_longest(rhs.0.iter())
                .map(|either_or_both| match either_or_both {
                    EitherOrBoth::Both(first, second) => first.clone() + second.clone(),
                    EitherOrBoth::Left(first) => first.clone().extend(rhs_length),
                    EitherOrBoth::Right(second) => Line::new().extend(self_length) + second.clone(),
                })
                .collect(),
        )
    }
}

impl Sum<Piece> for Piece {
    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    fn sum<I: Iterator<Item = Piece>>(iter: I) -> Self {
        iter.reduce(|acc, piece| acc + piece).unwrap_or_else(Piece::new)
    }
}

impl Add<Note> for Piece {
    type Output = Piece;

    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    fn add(self, rhs: Note) -> Self::Output {
        let line: Line = rhs.into();
        self + Piece(vec![line])
    }
}

impl Mul<Line> for Piece {
    type Output = Piece;

    #[expect(clippy::cast_possible_truncation, reason = "I don't want to deal with this right now")]
    fn mul(self, rhs: Line) -> Self::Output {
        let self_len = self.length();
        let rhs_len = rhs.length();
        let new_len = usize::max(self_len, rhs_len);

        // Extend pieces to same length for layering
        let extended_self: Vec<_> = self
            .0
            .into_iter()
            .map(|line| {
                let padding = new_len.saturating_sub(self_len) as u16;
                line.extend(padding)
            })
            .collect();

        let padding = new_len.saturating_sub(rhs_len) as u16;
        let extended_rhs = vec![rhs.extend(padding)];

        Piece([extended_self, extended_rhs].concat())
    }
}

impl Mul<Note> for Piece {
    type Output = Piece;

    fn mul(self, rhs: Note) -> Self::Output {
        Piece([self.0, vec![rhs.into()]].concat())
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let black_keys = [
            false, true, false, true, false, false, true, false, true, false, true, false,
        ];

        for bar_group in 0..self.length().div_ceil(64) {
            let (highest_semitone, lowest_semitone) = {
                let (mut highest, mut lowest) = (i16::MIN, i16::MAX);
                #[expect(clippy::arithmetic_side_effects, reason = "Guaranteed to be safe, manual bounds checking")]
                for time in (bar_group * 64)..(bar_group * 64 + 64) {
                    for note in self.get_notes_during_instant(time) {
                        if let NoteKind::Pitched {
                            pitch: NotePitch(frequency),
                            ..
                        } = note.1
                        {
                            let semitone_diff_from_c4 = 12.0 * f32::log2(frequency / C4.0);

                            #[expect(clippy::cast_possible_truncation, reason = "Intentional precision loss")]
                            if highest < semitone_diff_from_c4 as i16 {
                                highest = semitone_diff_from_c4 as i16;
                            } else if lowest > semitone_diff_from_c4 as i16 {
                                lowest = semitone_diff_from_c4 as i16;
                            }
                        }
                    }
                }
                (highest, lowest)
            };

            f.write_str(&"═".repeat(74))?;
            f.write_str("╗\n")?;

            #[expect(clippy::arithmetic_side_effects, reason = "User's fault")]
            for semitone in (lowest_semitone - 2..=highest_semitone + 2).rev() {
                let pitch = C4.semitone(semitone);
                let mut line_str = String::new();

                if [4, -1, -5, -10, -15, -20].contains(&semitone) {
                    f.write_char('!')?;
                } else {
                    f.write_char(' ')?;
                }

                for bar_group_time in 0..64 {
                    let time = 64 * bar_group + bar_group_time;
                    let black_key = black_keys[(semitone.rem_euclid(12)) as usize];

                    // Add barline
                    if bar_group_time % 16 == 0 {
                        if bar_group_time == 0 {
                            line_str.push_str(&format!("{: <3}", tet12::get_note_name_with_octave(pitch, A4)));
                            if black_key {
                                line_str.push_str("║ ║");
                            } else {
                                line_str.push_str("║█║");
                            }
                        } else {
                            line_str.push('|');
                        }
                    }

                    let blank_space = if black_key { ' ' } else { '░' };

                    let note_matches_line = |note: &Note| match note.1 {
                        NoteKind::Rest => false,
                        NoteKind::Pitched {
                            pitch: note_pitch,
                            timbre,
                            ..
                        } => {
                            !matches!(timbre, Timbre::Drums)
                                && (note_pitch.0 / pitch.0 - 1.0).abs() < (2.0f32.powf(1.0 / 24.0) - 1.0)
                        }
                    };

                    // Find notes at this time on this line
                    if let Some(_note) = self.get_notes_at_instant(time).find(note_matches_line) {
                        line_str.push('■');
                    } else if let Some(_note) = self.get_notes_during_instant(time).find(note_matches_line) {
                        line_str.push('≡');
                    } else {
                        line_str.push(blank_space);
                    }
                }

                line_str.push_str("║\n");
                f.write_str(&line_str)?;
            }

            f.write_str(&("═".repeat(74) + "╣" + "\n"))?;

            for kind in ["crash", "hi-hat", "snare", "kick"] {
                let mut line_str = String::new();

                for bar_group_time in 0..64 {
                    #[expect(clippy::arithmetic_side_effects, reason = "User's fault")]
                    let time = 64 * bar_group + bar_group_time;

                    // Add barline
                    if bar_group_time % 16 == 0 {
                        if bar_group_time == 0 {
                            line_str.push_str(&format!("{kind: <6}"));
                            line_str.push('║');
                        } else {
                            line_str.push('|');
                        }
                    }

                    let note_matches_line = |note: &Note| match note.1 {
                        NoteKind::Rest => false,
                        NoteKind::Pitched { pitch, timbre, .. } => {
                            matches!(timbre, crate::note::Timbre::Drums)
                                && match kind {
                                    "crash" => pitch.0 > C4.octave(1).semitone(6).0,
                                    "hi-hat" => C4.octave(1).semitone(6).0 > pitch.0 && pitch.0 > C4.semitone(6).0,
                                    "snare" => C4.semitone(-6).0 < pitch.0 && pitch.0 < C4.semitone(6).0,
                                    "kick" => pitch.0 < C4.semitone(-6).0,
                                    _ => false,
                                }
                        }
                    };

                    // Find notes at this time on this line
                    if let Some(_note) = self.get_notes_at_instant(time).find(note_matches_line) {
                        line_str.push('■');
                    } else if let Some(_note) = self.get_notes_during_instant(time).find(note_matches_line) {
                        line_str.push('≡');
                    } else {
                        line_str.push(' ');
                    }
                }

                line_str.push_str("║\n");
                f.write_str(&line_str)?;
            }

            f.write_str(&"═".repeat(74))?;
            f.write_str("╝\n\n\n")?;
        }

        Ok(())
    }
}

pub trait Reversable {
    fn reverse(self) -> Self;
}

impl Reversable for Piece {
    #[expect(clippy::arithmetic_side_effects, reason = "Cannot fail")]
    #[expect(clippy::cast_possible_truncation, reason = "Should be fine for reasonable piece lengths")]
    fn reverse(self) -> Self {
        // First, make every line the same length by padding with rests
        let mut piece = self;

        let max_length = piece.length();
        piece.0 = piece
            .0
            .into_iter()
            .map(|line| line.extend((max_length - line.length()) as u16))
            .collect();

        Piece(piece.0.into_iter().map(|line| line.reverse()).collect())
    }
}

impl Reversable for Line {
    fn reverse(self) -> Self {
        Line {
            notes: [
                self.notes.into_iter().rev().collect::<Vec<_>>(),
                self.pickup.into_iter().rev().collect::<Vec<_>>(),
            ]
            .concat(),
            pickup: Vec::new(),
            hold_pickup: false,
        }
    }
}
