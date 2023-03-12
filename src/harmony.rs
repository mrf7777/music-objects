use std::collections::HashSet;

use crate::pitch;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChordClass {
    notes: HashSet<pitch::NotePitchClass>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RootedChordClass {
    chord_class: ChordClass,
    root: pitch::NotePitch,
}
