pub use modes::*;

use crate::{
    instrument_tools::strings::StringTuning,
    note::{chord::Chord, NotePitch},
};

/// Gets the note name (without octave) for a given pitch.
///
/// Returns the note name in standard Western notation (C, C#, D, D#, E, F, F#, G, G#, A, A#, B)
/// relative to the provided A4 reference pitch.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
/// use symphoxy::scales::tet12::get_note_name;
///
/// let note_name = get_note_name(C4, A4);
/// assert_eq!(note_name, "C");
///
/// let sharp_name = get_note_name(NotePitch::new(277.18), A4); // C#4
/// assert_eq!(sharp_name, "C#");
/// ```
pub fn get_note_name(note: NotePitch, a4: NotePitch) -> String {
    let name = get_note_name_with_octave(note, a4);
    name.trim_end_matches(char::is_numeric).to_string()
}

/// Gets the note name with octave number for a given pitch.
///
/// Returns the note name with octave in standard Western notation (e.g., "C4", "A#5")
/// relative to the provided A4 reference pitch.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
/// use symphoxy::scales::tet12::get_note_name_with_octave;
///
/// let note_name = get_note_name_with_octave(C4, A4);
/// assert_eq!(note_name, "C4");
///
/// let higher_note = get_note_name_with_octave(NotePitch::new(880.0), A4); // A5
/// assert_eq!(higher_note, "A5");
/// ```
pub fn get_note_name_with_octave(note: NotePitch, a4: NotePitch) -> String {
    let c4 = a4.semitone(3).octave(-1);

    let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

    let diff = f32::log2(note.0 / c4.0);

    #[expect(clippy::cast_possible_truncation, reason = "log_2 of a non-infinite f32 has at most 7 bits")]
    let (octave_diff, semitone_diff) = (diff.floor() as i16, ((diff * 12.0).round() as i16).rem_euclid(12));

    #[expect(clippy::cast_sign_loss, reason = "semitone_diff is always in range 0..12")]
    let note_name = String::from(note_names[semitone_diff as usize]);

    #[expect(clippy::arithmetic_side_effects, reason = "This is guaranteed to fit in i16.")]
    let octave_number = octave_diff + 4;

    #[expect(clippy::arithmetic_side_effects, reason = "This is a simple string concatenation")]
    let out = note_name + &(octave_number).to_string();

    out
}

#[test]
fn test_get_note_name() {
    let notes = A4.semitones([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

    let expected_names = [
        "A4", "A#4", "B4", "C5", "C#5", "D5", "D#5", "E5", "F5", "F#5", "G5", "G#5",
    ];

    for (note, expected_name) in notes.iter().zip(expected_names.iter()) {
        let name = get_note_name_with_octave(*note, A4);
        assert_eq!(name, *expected_name);
    }
}

/// Standard pitch reference - A above middle C at 440 Hz.
///
/// This is the international standard tuning reference pitch.
pub const A4: NotePitch = NotePitch(440.0);
/// Middle C pitch at approximately 261.626 Hz.
///
/// This is a common reference point for musical compositions.
pub const C4: NotePitch = NotePitch(261.626);

/// A trait for 12-tone equal temperament pitch manipulation.
///
/// This trait provides methods for transposing pitches by octaves and semitones
/// within the 12-tone equal temperament system, where each octave is divided
/// into 12 equal semitones.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// // Octave transposition
/// let c5 = C4.octave(1);   // C4 up one octave = C5
/// let c3 = C4.octave(-1);  // C4 down one octave = C3
///
/// // Semitone transposition  
/// let cs4 = C4.semitone(1);  // C4 up one semitone = C#4
/// let b3 = C4.semitone(-1);  // C4 down one semitone = B3
///
/// // Multiple semitones at once
/// let major_triad_intervals = C4.semitones([0, 4, 7]); // C4, E4, G4
/// ```
pub trait Tet12 {
    /// Transposes the pitch by the specified number of octaves.
    ///
    /// Positive values transpose up, negative values transpose down.
    /// Each octave represents a doubling (or halving) of frequency.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let c5 = C4.octave(1);   // Up one octave
    /// let c2 = C4.octave(-2);  // Down two octaves
    /// ```
    fn octave(&self, change: i32) -> Self;

    /// Transposes the pitch by the specified number of semitones.
    ///
    /// Positive values transpose up, negative values transpose down.
    /// In 12-tone equal temperament, 12 semitones equal one octave.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let cs4 = C4.semitone(1);   // Up one semitone (C# / Db)
    /// let f4 = C4.semitone(5);    // Up five semitones (perfect fourth)
    /// let g4 = C4.semitone(7);    // Up seven semitones (perfect fifth)
    /// ```
    fn semitone(&self, change: i16) -> Self;

    /// Get several notes from this note by specifying a list of semitone offsets.
    fn semitones<const N: usize>(&self, changes: [i16; N]) -> [Self; N]
    where
        Self: Sized + Clone + Copy,
    {
        let mut result = [*self; N];
        for (i, &change) in changes.iter().enumerate() {
            result[i] = result[i].semitone(change);
        }
        result
    }
}

impl Tet12 for NotePitch {
    fn octave(&self, change: i32) -> Self {
        Self(self.0 * 2.0f32.powi(change))
    }

    fn semitone(&self, change: i16) -> Self {
        Self(self.0 * 2.0f32.powf(change as f32 / 12.0))
    }
}

impl<const N: usize> Tet12 for StringTuning<N> {
    fn octave(&self, change: i32) -> Self {
        StringTuning(self.0.map(|note| note.octave(change)))
    }

    fn semitone(&self, change: i16) -> Self {
        StringTuning(self.0.map(|note| note.semitone(change)))
    }
}

impl Tet12 for Chord {
    fn octave(&self, change: i32) -> Self {
        Chord::new(self.0.iter().map(|&note| note.octave(change)))
    }

    fn semitone(&self, change: i16) -> Self {
        Chord::new(self.0.iter().map(|&note| note.semitone(change)))
    }
}

fn get_degree_with_pattern_and_root(degree: isize, root: NotePitch, intervals: &'static [Interval]) -> NotePitch {
    #[expect(clippy::cast_possible_wrap, reason = "Only used internally, and correctly")]
    let len = intervals.len() as isize;

    #[expect(clippy::arithmetic_side_effects, reason = "Manual overflow checking")]
    let adjusted_degree = if degree > 0 { degree - 1 } else { degree };
    #[expect(clippy::cast_precision_loss, reason = "Precision loss impossible if len is properly sized")]
    let octave_power = adjusted_degree.div_euclid(len) as f32;

    let interval_power = intervals[adjusted_degree.rem_euclid(len) as usize].0 / 12.0;

    let factor = 2.0f32.powf(octave_power + interval_power);

    let pitch = root.0 * factor;

    NotePitch(pitch)
}

macro_rules! implement_tet12_scale {
    ($name:ident, $len:expr, $intervals:expr, $doc:expr) => {
        #[doc = $doc]
        pub struct $name(pub NotePitch);

        impl $name {
            const INTERVALS: [Interval; $len] = $intervals;
        }

        impl Scale for $name {
            fn intervals() -> &'static [Interval] {
                &Self::INTERVALS
            }

            fn get_degree(&self, degree: isize) -> NotePitch {
                get_degree_with_pattern_and_root(degree, self.0, &Self::intervals())
            }
        }
    };
}

/// Musical modes and scale implementations.
///
/// Contains implementations of various musical scales and modes
/// (major, minor, dorian, lydian, etc.) in the 12-tone equal temperament system.
pub mod modes {
    use crate::scales::interval::Interval;
    use crate::{scales::tet12::get_degree_with_pattern_and_root, NotePitch, Scale};

    implement_tet12_scale!(
        LydianScale,
        7,
        [
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MAJOR_THIRD,
            Interval::AUGMENTED_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SIXTH,
            Interval::MAJOR_SEVENTH
        ],
        "Lydian mode - a major-type scale with a raised 4th degree, creating a bright, dreamy sound."
    );
    implement_tet12_scale!(
        MajorScale,
        7,
        [
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SIXTH,
            Interval::MAJOR_SEVENTH
        ],
        "Major scale - the most common Western scale, providing a happy, bright sound. Also known as Ionian mode."
    );
    implement_tet12_scale!(
        MixolydianScale,
        7,
        [
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SIXTH,
            Interval::MINOR_SEVENTH
        ],
        "Mixolydian mode - a major-type scale with a flattened 7th degree."
    );
    implement_tet12_scale!(
        DorianScale,
        7,
        [
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SIXTH,
            Interval::MINOR_SEVENTH
        ],
        "Dorian mode - a minor-type scale with a raised 6th degree."
    );
    implement_tet12_scale!(
        MinorScale,
        7,
        [
            Interval::UNISON,
            Interval::MAJOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SIXTH,
            Interval::MINOR_SEVENTH
        ],
        "Natural minor scale - provides a sad, melancholic sound. Also known as Aeolian mode."
    );
    implement_tet12_scale!(
        PhrygianScale,
        7,
        [
            Interval::UNISON,
            Interval::MINOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SIXTH,
            Interval::MINOR_SEVENTH
        ],
        "Phrygian mode - a minor-type scale with a flattened 2nd degree."
    );
    implement_tet12_scale!(
        LocrianScale,
        7,
        [
            Interval::UNISON,
            Interval::MINOR_SECOND,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FOURTH,
            Interval::TRITONE,
            Interval::MINOR_SIXTH,
            Interval::MINOR_SEVENTH
        ],
        "Locrian mode - a diminished-type scale with both flattened 2nd and 5th degrees."
    );

    pub use MajorScale as IonianScale;
    pub use MinorScale as AeolianScale;
}

use super::Scale;
use crate::scales::interval::Interval;

implement_tet12_scale!(
    ChromaticScale,
    12,
    [
        Interval::UNISON,
        Interval::MINOR_SECOND,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::TRITONE,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
        Interval::MAJOR_SEVENTH
    ],
    "Chromatic scale - includes all twelve notes in the octave."
);

implement_tet12_scale!(
    WholeToneScale,
    6,
    [
        Interval::UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::AUGMENTED_FIFTH,
        Interval::AUGMENTED_SIXTH
    ],
    "Whole tone scale - consists entirely of whole steps, creating a dreamy, ambiguous sound."
);

pub use ChromaticScale as TwelveToneScale;
