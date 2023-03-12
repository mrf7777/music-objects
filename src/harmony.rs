use std::collections::HashSet;

use crate::pitch;

#[derive()]
pub struct ChordClass {
    notes: HashSet<pitch::NotePitchClass>,
}

#[derive()]
pub struct RootedChordClass {
    chord_class: ChordClass,
    root: pitch::NotePitch,
}
