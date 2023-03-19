#![deny(clippy::all, clippy::pedantic)]

use std::{error::Error, fmt::Display, num::TryFromIntError};

use crate::interval;

pub type Pitch = f64;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub enum TuningSystem {
    #[default]
    EqualTempered,
}

#[allow(clippy::module_name_repetitions)]
pub trait ToPitch {
    type Error;

    fn to_pitch_using_tuning(&self, tuning: TuningSystem) -> Result<Pitch, Self::Error>;

    fn to_pitch(&self) -> Result<Pitch, Self::Error> {
        self.to_pitch_using_tuning(TuningSystem::EqualTempered)
    }
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

#[derive(Clone, Copy, Debug)]
pub struct IntDoesNotMatchEnum;

impl Display for IntDoesNotMatchEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("integer does not map to an enumeration value")?;
        Ok(())
    }
}

impl Error for IntDoesNotMatchEnum {}

impl TryFrom<i32> for NotePitchClass {
    type Error = IntDoesNotMatchEnum;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::C),
            1 => Ok(Self::Cs),
            2 => Ok(Self::D),
            3 => Ok(Self::Ds),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            6 => Ok(Self::Fs),
            7 => Ok(Self::G),
            8 => Ok(Self::Gs),
            9 => Ok(Self::A),
            10 => Ok(Self::As),
            11 => Ok(Self::B),
            _ => Err(IntDoesNotMatchEnum),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[allow(clippy::module_name_repetitions)]
pub struct NotePitch {
    class: NotePitchClass,
    octave: interval::Octave,
}

impl NotePitch {
    #[must_use]
    pub fn new(class: NotePitchClass, octave: interval::Octave) -> Self {
        Self { class, octave }
    }

    #[must_use]
    pub fn class(&self) -> NotePitchClass {
        self.class
    }

    #[must_use]
    pub fn octave(&self) -> interval::Octave {
        self.octave
    }
}

impl Ord for NotePitch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let octave_compare = self.octave().cmp(&other.octave());
        match octave_compare {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => (self.class() as i32).cmp(&(other.class() as i32)),
        }
    }
}

impl PartialOrd for NotePitch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ToPitch for NotePitch {
    type Error = TryFromIntError;

    fn to_pitch_using_tuning(&self, tuning: TuningSystem) -> Result<Pitch, Self::Error> {
        match tuning {
            TuningSystem::EqualTempered => {
                let semitones_from_a4 = interval::DirectedSemitoneInterval::from_note_pitches(
                    &NotePitch {
                        class: NotePitchClass::A,
                        octave: 4,
                    },
                    self,
                )?
                .directional_semitones();

                // https://pages.mtu.edu/~suits/NoteFreqCalcs.html
                Ok(A4_PITCH_ISO_16 * EQUAL_TEMPERED_SEMITONE_FACTOR.powi(semitones_from_a4))
            }
        }
    }
}

const A4_PITCH_ISO_16: f64 = 440.0;

// https://pages.mtu.edu/~suits/NoteFreqCalcs.html
const EQUAL_TEMPERED_SEMITONE_FACTOR: f64 = 1.059_463_094_36;

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
