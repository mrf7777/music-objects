#![deny(clippy::all, clippy::pedantic)]

use std::num::TryFromIntError;

use crate::pitch;

pub type Semitones = u16;
pub type DirectionalSemitones = i32;
pub type Octave = i32;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
    pub fn semitones(&self) -> Semitones {
        self.semitones
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct DirectedSemitoneInterval {
    interval: SemitoneInterval,
    direction: Direction,
}

impl Ord for DirectedSemitoneInterval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_semis = self.directional_semitones();
        let other_semis = other.directional_semitones();

        self_semis.cmp(&other_semis)
    }
}

impl PartialOrd for DirectedSemitoneInterval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl DirectedSemitoneInterval {
    #[must_use]
    pub fn new(semitones: SemitoneInterval, direction: Direction) -> Self {
        Self {
            interval: semitones,
            direction,
        }
    }

    #[must_use]
    pub fn from_note_pitches(
        n1: &pitch::NotePitch,
        n2: &pitch::NotePitch,
    ) -> Result<DirectedSemitoneInterval, TryFromIntError> {
        let semis_from_only_octaves = (n2.octave() - n1.octave()) * 12;
        let semis_from_note_pitch_class = n2.class() as i32 - n1.class() as i32;
        let semis = semis_from_note_pitch_class + semis_from_only_octaves;
        if semis >= 0 {
            Ok(Self {
                interval: SemitoneInterval::new(Semitones::try_from(semis)?),
                direction: Direction::Up,
            })
        } else {
            Ok(Self {
                interval: SemitoneInterval::new(Semitones::try_from(-semis)?),
                direction: Direction::Down,
            })
        }
    }

    #[must_use]
    pub fn directional_semitones(&self) -> DirectionalSemitones {
        match self.direction {
            Direction::Up => DirectionalSemitones::from(self.interval.semitones),
            Direction::Down => -DirectionalSemitones::from(self.interval.semitones),
        }
    }

    #[must_use]
    pub fn interval(&self) -> &SemitoneInterval {
        &self.interval
    }

    #[must_use]
    pub fn direction(&self) -> Direction {
        self.direction
    }

    #[must_use]
    pub fn apply_to_note_pitch(
        &self,
        note_pitch: &pitch::NotePitch,
    ) -> Result<pitch::NotePitch, ()> {
        let total_semitones = self.directional_semitones();

        // get octaves and semitones through division
        let octaves = total_semitones / 12;
        let semitones = total_semitones % 12;

        let new_base_octave = note_pitch.octave() + octaves;
        let new_pitch_class_without_modulo = note_pitch.class() as i32 + semitones;

        let fixed_octave = new_base_octave + (new_pitch_class_without_modulo / 12);
        let fixed_pitch_class: pitch::NotePitchClass =
            (new_pitch_class_without_modulo % 12).try_into()?;

        Ok(pitch::NotePitch::new(fixed_pitch_class, fixed_octave))
    }
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

        let interval1 = DirectedSemitoneInterval::from_note_pitches(&a4, &a5).unwrap();
        assert_eq!(interval1.direction, Direction::Up);
        assert_eq!(interval1.interval, SemitoneInterval { semitones: 12 });

        let interval2 = DirectedSemitoneInterval::from_note_pitches(&a4, &a6).unwrap();
        assert_eq!(interval2.direction, Direction::Up);
        assert_eq!(interval2.interval, SemitoneInterval { semitones: 24 });

        let interval3 = DirectedSemitoneInterval::from_note_pitches(&a5, &a4).unwrap();
        assert_eq!(interval3.direction, Direction::Down);
        assert_eq!(interval3.interval, SemitoneInterval { semitones: 12 });

        let interval4 = DirectedSemitoneInterval::from_note_pitches(&a6, &a4).unwrap();
        assert_eq!(interval4.direction, Direction::Down);
        assert_eq!(interval4.interval, SemitoneInterval { semitones: 24 });
    }

    #[test]
    fn directed_semitone_interval_from_non_octaves() {
        let c4 = NotePitch::new(NotePitchClass::C, 4);
        let g4 = NotePitch::new(NotePitchClass::G, 4);
        let d5 = NotePitch::new(NotePitchClass::D, 5);

        let interval1 = DirectedSemitoneInterval::from_note_pitches(&c4, &g4).unwrap();
        assert_eq!(interval1.direction, Direction::Up);
        assert_eq!(interval1.interval, SemitoneInterval { semitones: 7 });

        let interval2 = DirectedSemitoneInterval::from_note_pitches(&c4, &d5).unwrap();
        assert_eq!(interval2.direction, Direction::Up);
        assert_eq!(interval2.interval, SemitoneInterval { semitones: 14 });

        let interval3 = DirectedSemitoneInterval::from_note_pitches(&g4, &c4).unwrap();
        assert_eq!(interval3.direction, Direction::Down);
        assert_eq!(interval3.interval, SemitoneInterval { semitones: 7 });

        let interval4 = DirectedSemitoneInterval::from_note_pitches(&d5, &c4).unwrap();
        assert_eq!(interval4.direction, Direction::Down);
        assert_eq!(interval4.interval, SemitoneInterval { semitones: 14 });
    }

    #[test]
    fn apply_interval_to_note_pitch_single_octave() {
        let interval1 = DirectedSemitoneInterval::new(SemitoneInterval::new(4), Direction::Up);
        let interval2 = DirectedSemitoneInterval::new(SemitoneInterval::new(3), Direction::Up);
        let interval3 =
            DirectedSemitoneInterval::new(interval1.interval().clone(), Direction::Down);
        let interval4 =
            DirectedSemitoneInterval::new(interval2.interval().clone(), Direction::Down);

        let note1 = NotePitch::new(NotePitchClass::C, 4);
        let note2 = NotePitch::new(NotePitchClass::E, 4);
        let note3 = NotePitch::new(NotePitchClass::G, 4);

        assert_eq!(interval1.apply_to_note_pitch(&note1).unwrap(), note2);
        assert_eq!(interval2.apply_to_note_pitch(&note2).unwrap(), note3);

        assert_eq!(interval3.apply_to_note_pitch(&note2).unwrap(), note1);
        assert_eq!(interval4.apply_to_note_pitch(&note3).unwrap(), note2);
    }

    #[test]
    fn apply_interval_to_note_pitch_multi_octave() {
        let interval1 = DirectedSemitoneInterval::new(SemitoneInterval::new(14), Direction::Up);
        let interval2 = DirectedSemitoneInterval::new(SemitoneInterval::new(26), Direction::Up);
        let interval3 =
            DirectedSemitoneInterval::new(interval1.interval().clone(), Direction::Down);
        let interval4 =
            DirectedSemitoneInterval::new(interval2.interval().clone(), Direction::Down);

        let note1 = NotePitch::new(NotePitchClass::C, 4);
        let note2 = NotePitch::new(NotePitchClass::D, 5);
        let note3 = NotePitch::new(NotePitchClass::D, 6);

        assert_eq!(interval1.apply_to_note_pitch(&note1).unwrap(), note2);
        assert_eq!(interval2.apply_to_note_pitch(&note1).unwrap(), note3);

        assert_eq!(interval3.apply_to_note_pitch(&note2).unwrap(), note1);
        assert_eq!(interval4.apply_to_note_pitch(&note3).unwrap(), note1);
    }
}
