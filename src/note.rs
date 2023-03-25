use crate::pitch;
use crate::rhythm;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Note {
    note_pitch: pitch::NotePitch,
    duration: rhythm::Duration,
}

impl Note {
    #[must_use]
    pub fn new(note_pitch: pitch::NotePitch, duration: rhythm::Duration) -> Self {
        Self {
            note_pitch,
            duration,
        }
    }

    #[must_use]
    pub fn note_pitch(&self) -> &pitch::NotePitch {
        &self.note_pitch
    }

    #[must_use]
    pub fn duration(&self) -> &rhythm::Duration {
        &self.duration
    }
}
