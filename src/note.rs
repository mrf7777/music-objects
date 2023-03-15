use crate::pitch;
use crate::rhythm;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Note {
    note_pitch: pitch::NotePitch,
    duration: rhythm::Duration,
}

impl Note {
    pub fn new(note_pitch: pitch::NotePitch, duration: rhythm::Duration) -> Self {
        Self {
            note_pitch,
            duration,
        }
    }

    pub fn note_pitch(&self) -> &pitch::NotePitch {
        &self.note_pitch
    }

    pub fn duration(&self) -> &rhythm::Duration {
        &self.duration
    }
}
