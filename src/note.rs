use crate::pitch;

pub type Duration = ();
pub type Velocity = i8;

#[derive()]
pub struct Note {
    pitch: pitch::NotePitch,
    duration: Duration,
}
