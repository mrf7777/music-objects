#![deny(clippy::all, clippy::pedantic)]

use crate::pitch;

pub type Semitones = u16;
pub type DirectionalSemitones = i32;
pub type Octave = i8;

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

        if self_semis < other_semis {
            std::cmp::Ordering::Less
        } else if self_semis > other_semis {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for DirectedSemitoneInterval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl DirectedSemitoneInterval {
    #[must_use]
    pub fn from_note_pitches(
        n1: &pitch::NotePitch,
        n2: &pitch::NotePitch,
    ) -> Option<DirectedSemitoneInterval> {
        let semis_from_only_octaves = isize::from((n2.octave() - n1.octave()) * 12);
        let semis_from_note_pitch_class = n2.class() as isize - n1.class() as isize;
        let semis = semis_from_note_pitch_class + semis_from_only_octaves;
        if semis >= 0 {
            Some(Self {
                interval: SemitoneInterval::new(Semitones::try_from(semis).ok()?),
                direction: Direction::Up,
            })
        } else {
            Some(Self {
                interval: SemitoneInterval::new(Semitones::try_from(-semis).ok()?),
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
}
