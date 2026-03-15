use symphoxy::piece::Reversable;
use symphoxy::scales::tet12::*;
use symphoxy::{electric_guitar, sixteenth, InteractiveTui, Line, Piece, Scale, C4};

fn ascending_descending_scale<S: Scale>(scale: S) -> Line {
    let notes = scale.get_degrees([1, 2, 3, 4, 5, 6, 7, 8]);
    let ascending: Line = notes.iter().cloned().map(sixteenth).sum();
    let descending: Line = notes.iter().cloned().map(sixteenth).rev().sum();
    ascending + descending
}

fn ascending_descending_scale_chords<S: Scale>(scale: S) -> Piece {
    let notes: Piece = (1..=8)
        .map(|degree| {
            let [root, third, fifth] = scale.get_degrees([degree, degree + 2, degree + 4]);

            sixteenth(root) * sixteenth(third) * sixteenth(fifth)
        })
        .sum();

    let ascending: Piece = notes.clone();
    let descending: Piece = notes.reverse();

    ascending + descending
}

fn scales_test() -> impl Into<Piece> {
    let scales = Line::new()
        + ascending_descending_scale(LydianScale(C4))
        + ascending_descending_scale(IonianScale(C4))
        + ascending_descending_scale(MixolydianScale(C4))
        + ascending_descending_scale(DorianScale(C4))
        + ascending_descending_scale(AeolianScale(C4))
        + ascending_descending_scale(PhrygianScale(C4))
        + ascending_descending_scale(LocrianScale(C4))
        + {
            let notes = ChromaticScale(C4).get_degrees([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
            let ascending: Line = notes.iter().cloned().map(sixteenth).sum();
            let descending: Line = notes.iter().cloned().map(sixteenth).rev().sum();
            ascending + descending
        }
        + {
            let notes = WholeToneScale(C4).get_degrees([1, 2, 3, 4, 5, 6, 7]);
            let ascending: Line = notes.iter().cloned().map(sixteenth).sum();
            let descending: Line = notes.iter().cloned().map(sixteenth).rev().sum();
            ascending + descending
        };

    let scale_chords = Line::new()
        + ascending_descending_scale_chords(LydianScale(C4))
        + ascending_descending_scale_chords(IonianScale(C4))
        + ascending_descending_scale_chords(MixolydianScale(C4))
        + ascending_descending_scale_chords(DorianScale(C4))
        + ascending_descending_scale_chords(AeolianScale(C4))
        + ascending_descending_scale_chords(PhrygianScale(C4))
        + ascending_descending_scale_chords(LocrianScale(C4));

    electric_guitar(scales + scale_chords).volume(2.0)
}

fn main() {
    let song: Piece = scales_test().into();
    InteractiveTui::start(song);
}
