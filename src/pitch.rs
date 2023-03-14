use crate::interval;

type Pitch = f64;

trait ToPitch {
    fn to_pitch(&self) -> Option<Pitch>;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NotePitchClass {
    C = 0,
    Cs = 1,
    D = 2,
    Ds = 3,
    E = 4,
    F = 5,
    Fs = 6,
    G = 7,
    Gs = 8,
    A = 9,
    As = 10,
    B = 11,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NotePitch {
    class: NotePitchClass,
    octave: interval::Octave,
}

impl NotePitch {
    pub fn new(class: NotePitchClass, octave: interval::Octave) -> Self {
        Self { class, octave }
    }

    pub fn class(&self) -> NotePitchClass {
        self.class
    }

    pub fn octave(&self) -> interval::Octave {
        self.octave
    }
}

impl ToPitch for NotePitch {
    fn to_pitch(&self) -> Option<Pitch> {
        let semitones_from_a4 = interval::DirectedSemitoneInterval::from_note_pitches(
            &NotePitch {
                class: NotePitchClass::A,
                octave: 4,
            },
            self,
        )?
        .directional_semitones();

        // https://pages.mtu.edu/~suits/NoteFreqCalcs.html
        Some(A4_PITCH * EQUAL_TEMPERED_SEMITONE_FACTOR.powi(semitones_from_a4.into()))
    }
}

const A4_PITCH: f64 = 440.0;

// https://pages.mtu.edu/~suits/NoteFreqCalcs.html
const EQUAL_TEMPERED_SEMITONE_FACTOR: f64 = 1.05946309436;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_pitch_to_pitch_with_note_a() {
        let a440 = NotePitch {
            class: NotePitchClass::A,
            octave: 4,
        };
        assert!((a440.to_pitch().unwrap() - 440.00).abs() < 0.05);

        let a220 = NotePitch {
            class: NotePitchClass::A,
            octave: 3,
        };
        assert!((a220.to_pitch().unwrap() - 220.00).abs() < 0.05);

        let a880 = NotePitch {
            class: NotePitchClass::A,
            octave: 5,
        };
        assert!((a880.to_pitch().unwrap() - 880.00).abs() < 0.05);
    }

    #[test]
    fn note_pitch_to_pitch_without_note_a() {
        let c4 = NotePitch {
            class: NotePitchClass::C,
            octave: 4,
        };
        assert!((c4.to_pitch().unwrap() - 261.63).abs() < 0.05);

        let c3 = NotePitch {
            class: NotePitchClass::C,
            octave: 3,
        };
        assert!((c3.to_pitch().unwrap() - 130.81).abs() < 0.05);

        let gs8 = NotePitch {
            class: NotePitchClass::Gs,
            octave: 8,
        };
        assert!((gs8.to_pitch().unwrap() - 6644.88).abs() < 0.05);
    }
}
