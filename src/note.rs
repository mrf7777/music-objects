use crate::pitch;
use crate::rhythm;

pub type Velocity = i8;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Note {
    pitch: pitch::NotePitch,
    duration: rhythm::Duration,
}
