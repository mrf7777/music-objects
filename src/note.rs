use crate::pitch;

pub type Duration = ();
pub type Velocity = i8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Note {
    pitch: pitch::NotePitch,
    duration: Duration,
}
