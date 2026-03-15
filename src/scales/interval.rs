use std::ops::Add;

use crate::{Chord, Line, Note, NoteKind, NotePitch, Piece};

#[derive(Clone, Copy)]
pub struct Interval(pub f32);

impl Interval {
    pub const UNISON: Interval = Interval(0.0);

    pub const MINOR_SECOND: Interval = Interval(1.0);
    pub const AUGMENTED_UNISON: Interval = Self::MINOR_SECOND;

    pub const MAJOR_SECOND: Interval = Interval(2.0);

    pub const MINOR_THIRD: Interval = Interval(3.0);
    pub const AUGMENTED_SECOND: Interval = Self::MINOR_THIRD;

    pub const MAJOR_THIRD: Interval = Interval(4.0);

    pub const PERFECT_FOURTH: Interval = Interval(5.0);
    pub const AUGMENTED_THIRD: Interval = Self::PERFECT_FOURTH;

    pub const TRITONE: Interval = Interval(6.0);
    pub const AUGMENTED_FOURTH: Interval = Self::TRITONE;
    pub const DIMINISHED_FIFTH: Interval = Self::TRITONE;

    pub const PERFECT_FIFTH: Interval = Interval(7.0);

    pub const MINOR_SIXTH: Interval = Interval(8.0);
    pub const AUGMENTED_FIFTH: Interval = Self::MINOR_SIXTH;

    pub const MAJOR_SIXTH: Interval = Interval(9.0);

    pub const MINOR_SEVENTH: Interval = Interval(10.0);
    pub const AUGMENTED_SIXTH: Interval = Self::MINOR_SEVENTH;

    pub const MAJOR_SEVENTH: Interval = Interval(11.0);
    pub const DIMINISHED_OCTAVE: Interval = Self::MAJOR_SEVENTH;

    pub const OCTAVE: Interval = Interval(12.0);
}

pub struct ChordShape(pub Vec<Interval>);

impl ChordShape {
    pub fn transpose_to(&self, root: NotePitch) -> Chord {
        let pitches = self
            .0
            .iter()
            .map(|interval| {
                let semitone_factor = 2.0f32.powf(interval.0 / 12.0);
                NotePitch(root.0 * semitone_factor)
            })
            .collect();
        Chord(pitches)
    }

    pub fn from_intervals(intervals: impl IntoIterator<Item = Interval>) -> Self {
        ChordShape(intervals.into_iter().collect())
    }

    pub fn from_stacked_intervals(
        intervals: impl IntoIterator<Item = Interval, IntoIter = impl ExactSizeIterator<Item = Interval>>,
    ) -> Self {
        let intervals = intervals.into_iter();
        let mut cumulative_intervals = Vec::with_capacity(intervals.len());
        let mut total = Interval::UNISON;
        for interval in intervals {
            total = Interval(total.0 + interval.0);
            cumulative_intervals.push(total);
        }
        ChordShape(cumulative_intervals)
    }
}

/// A trait for types that can be transformed using chord shapes.
///
/// This trait enables applying chord structures to musical elements,
/// typically transposing a chord shape to different root notes.
pub trait ChordFluid {
    /// The output type after applying the chord transformation
    type Output;

    /// Creates a chord with the given shape, transposed so that the lowest pitch of the chord meets the relevant pitch.
    fn with_chord_shape(self, chord_shape: &ChordShape) -> Self::Output;
}

impl ChordFluid for NotePitch {
    type Output = Chord;

    fn with_chord_shape(self, chord_shape: &ChordShape) -> Self::Output {
        chord_shape.transpose_to(self)
    }
}

impl ChordFluid for Note {
    type Output = Piece;

    fn with_chord_shape(self, chord_shape: &ChordShape) -> Self::Output {
        match self.1 {
            NoteKind::Rest => Piece(vec![Line {
                notes: vec![self],
                pickup: vec![],
                hold_pickup: false,
            }]),
            NoteKind::Pitched { pitch, timbre, volume } => {
                let chord = pitch.with_chord_shape(chord_shape);

                Piece(
                    chord
                        .0
                        .into_iter()
                        .map(|note_pitch| Line {
                            notes: vec![Note(
                                self.0,
                                NoteKind::Pitched {
                                    pitch: note_pitch,
                                    timbre,
                                    volume,
                                },
                            )],
                            pickup: vec![],
                            hold_pickup: false,
                        })
                        .collect(),
                )
            }
        }
    }
}

impl ChordFluid for Line {
    type Output = Piece;

    fn with_chord_shape(self, chord_shape: &ChordShape) -> Self::Output {
        self.notes
            .into_iter()
            .map(|note| note.with_chord_shape(chord_shape))
            .reduce(Add::add)
            .unwrap_or_else(|| Piece(vec![]))
    }
}
