#![cfg_attr(rustfmt, rustfmt_skip)]

use symphoxy::{ChordShape, prelude::*, scales::interval::{ChordFluid, Interval}};

use lazy_static::lazy_static;

lazy_static! {
    static ref OPEN_FIVE_SHAPE: ChordShape = ChordShape::from_intervals([
        Interval::UNISON,
        Interval::PERFECT_FIFTH
    ]);
}

fn downbeat_bass(n1: NotePitch) -> Line {
    bass(eighth(n1) + eighth(REST) * 3)
}

pub fn brain_stew(fifth: bool) -> Piece {
    let gs3 = C4.semitone(8).octave(-2);
    let [fs3, f3, e3, ds3] = gs3.semitones([-2, -3, -4, -5]);

    fn mixup(p: NotePitch) -> Line {
        bass(eighth(p) + eighth(REST))
    }

    let line = downbeat_bass(gs3) + downbeat_bass(fs3) + downbeat_bass(f3) + mixup(e3) + mixup(ds3);

    if fifth {
        // Add a fifth to every note
        line.with_chord_shape(&OPEN_FIVE_SHAPE)
    } else {
        Piece::from(line)
    }
}

pub fn bridge_bass() -> Piece {
    let gs3 = C4.semitone(8).octave(-2);
    let [a4, as4, c4] = gs3.semitones([1, 2, 4]);

    (downbeat_bass(gs3) + downbeat_bass(a4) + downbeat_bass(as4) + downbeat_bass(c4))
        .with_chord_shape(&OPEN_FIVE_SHAPE)
}