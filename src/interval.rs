use crate::pitch;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type Semitones = i32;
pub type Octave = i32;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(clippy::module_name_repetitions)]
pub struct SemitoneInterval {
    semitones: Semitones,
}

impl SemitoneInterval {
    #[must_use]
    pub fn new(semitones: Semitones) -> Self {
        Self { semitones }
    }

    #[must_use]
    pub fn new_from_note_pitches(n1: &pitch::NotePitch, n2: &pitch::NotePitch) -> Self {
        let semis_from_only_octaves = (n2.octave() - n1.octave()) * 12;
        let semis_from_note_pitch_class = n2.class() as i32 - n1.class() as i32;
        let semis = semis_from_note_pitch_class + semis_from_only_octaves;
        Self { semitones: semis }
    }

    #[must_use]
    pub fn new_from_direction(semitones: Semitones, direction: Direction) -> Self {
        Self {semitones: match direction {
            Direction::Up => semitones,
            Direction::Down => -semitones,
        }}
    }

    #[must_use]
    pub fn semitones(&self) -> Semitones {
        self.semitones
    }

    #[must_use]
    pub fn direction(&self) -> Option<Direction> {
        match self.semitones.cmp(&0) {
            std::cmp::Ordering::Less => Some(Direction::Down),
            std::cmp::Ordering::Greater => Some(Direction::Up),
            std::cmp::Ordering::Equal => None
        }
    }

    #[must_use]
    pub fn apply_to_note_pitch(&self, note_pitch: &pitch::NotePitch) -> pitch::NotePitch {
        let total_semitones = self.semitones();

        // get octaves and semitones through division
        let octaves = total_semitones / 12;
        let semitones = total_semitones % 12;

        let new_base_octave = note_pitch.octave() + octaves;
        let new_pitch_class_without_modulo = note_pitch.class() as i32 + semitones;

        let fixed_octave = new_base_octave + (new_pitch_class_without_modulo / 12);
        let fixed_pitch_class =
            pitch::NotePitchClass::try_from(new_pitch_class_without_modulo % 12).unwrap();

        pitch::NotePitch::new(fixed_pitch_class, fixed_octave)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Direction {
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use crate::pitch::NotePitch;
    use crate::pitch::NotePitchClass;

    use super::*;

    #[test]
    fn directed_semitone_interval_from_note_pitches_octave() {
        let a4 = NotePitch::new(NotePitchClass::A, 4);
        let a5 = NotePitch::new(NotePitchClass::A, 5);
        let a6 = NotePitch::new(NotePitchClass::A, 6);

        let interval1 = SemitoneInterval::new_from_note_pitches(&a4, &a5);
        assert_eq!(interval1.direction().unwrap(), Direction::Up);
        assert_eq!(interval1.semitones(), 12);

        let interval2 = SemitoneInterval::new_from_note_pitches(&a4, &a6);
        assert_eq!(interval2.direction().unwrap(), Direction::Up);
        assert_eq!(interval2.semitones(), 24);

        let interval3 = SemitoneInterval::new_from_note_pitches(&a5, &a4);
        assert_eq!(interval3.direction().unwrap(), Direction::Down);
        assert_eq!(interval3.semitones(), -12);

        let interval4 = SemitoneInterval::new_from_note_pitches(&a6, &a4);
        assert_eq!(interval4.direction().unwrap(), Direction::Down);
        assert_eq!(interval4.semitones(), -24);
    }

    #[test]
    fn directed_semitone_interval_from_non_octaves() {
        let c4 = NotePitch::new(NotePitchClass::C, 4);
        let g4 = NotePitch::new(NotePitchClass::G, 4);
        let d5 = NotePitch::new(NotePitchClass::D, 5);

        let interval1 = SemitoneInterval::new_from_note_pitches(&c4, &g4);
        assert_eq!(interval1.direction().unwrap(), Direction::Up);
        assert_eq!(interval1.semitones(), 7 );

        let interval2 = SemitoneInterval::new_from_note_pitches(&c4, &d5);
        assert_eq!(interval2.direction().unwrap(), Direction::Up);
        assert_eq!(interval2.semitones(), 14);

        let interval3 = SemitoneInterval::new_from_note_pitches(&g4, &c4);
        assert_eq!(interval3.direction().unwrap(), Direction::Down);
        assert_eq!(interval3.semitones(), -7);

        let interval4 = SemitoneInterval::new_from_note_pitches(&d5, &c4);
        assert_eq!(interval4.direction().unwrap(), Direction::Down);
        assert_eq!(interval4.semitones(), -14);
    }

    #[test]
    fn apply_interval_to_note_pitch_single_octave() {
        let interval1 = SemitoneInterval::new(4);
        let interval2 = SemitoneInterval::new(3);
        let interval3 =
        SemitoneInterval::new(-interval1.semitones());
        let interval4 =
        SemitoneInterval::new(-interval2.semitones());

        let note1 = NotePitch::new(NotePitchClass::C, 4);
        let note2 = NotePitch::new(NotePitchClass::E, 4);
        let note3 = NotePitch::new(NotePitchClass::G, 4);

        assert_eq!(interval1.apply_to_note_pitch(&note1), note2);
        assert_eq!(interval2.apply_to_note_pitch(&note2), note3);

        assert_eq!(interval3.apply_to_note_pitch(&note2), note1);
        assert_eq!(interval4.apply_to_note_pitch(&note3), note2);
    }

    #[test]
    fn apply_interval_to_note_pitch_multi_octave() {
        let interval1 = SemitoneInterval::new(14);
        let interval2 = SemitoneInterval::new(26);
        let interval3 =
        SemitoneInterval::new(-interval1.semitones());
        let interval4 =
        SemitoneInterval::new(-interval2.semitones());

        let note1 = NotePitch::new(NotePitchClass::C, 4);
        let note2 = NotePitch::new(NotePitchClass::D, 5);
        let note3 = NotePitch::new(NotePitchClass::D, 6);

        assert_eq!(interval1.apply_to_note_pitch(&note1), note2);
        assert_eq!(interval2.apply_to_note_pitch(&note1), note3);

        assert_eq!(interval3.apply_to_note_pitch(&note2), note1);
        assert_eq!(interval4.apply_to_note_pitch(&note3), note1);
    }
}
