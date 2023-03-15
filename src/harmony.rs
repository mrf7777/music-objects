#![deny(clippy::all, clippy::pedantic)]

use std::collections::HashSet;

use crate::pitch;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChordClass {
    note_pitch_classes: HashSet<pitch::NotePitchClass>,
}

impl ChordClass {
    pub fn new(note_pitch_classes: HashSet<pitch::NotePitchClass>) -> Self {
        Self { note_pitch_classes }
    }

    pub fn note_pitch_classes(&self) -> &HashSet<pitch::NotePitchClass> {
        &self.note_pitch_classes
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RootedChordClass {
    chord_class: ChordClass,
    root: pitch::NotePitchClass,
}

impl RootedChordClass {
    pub fn new(chord_class: ChordClass, root: pitch::NotePitchClass) -> Self {
        Self { chord_class, root }
    }

    pub fn chord_class(&self) -> &ChordClass {
        &self.chord_class
    }

    pub fn root(&self) -> pitch::NotePitchClass {
        self.root
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Chord {
    note_pitches: HashSet<pitch::NotePitch>,
}

impl Chord {
    pub fn new(note_pitches: HashSet<pitch::NotePitch>) -> Self {
        Self { note_pitches }
    }

    pub fn note_pitchs(&self) -> &HashSet<pitch::NotePitch> {
        &self.note_pitches
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RootedChord {
    chord: Chord,
    root: pitch::NotePitch,
}

impl RootedChord {
    pub fn new(chord: Chord, root: pitch::NotePitch) -> Self {
        Self { chord, root }
    }

    pub fn chord(&self) -> &Chord {
        &self.chord
    }

    pub fn root(&self) -> pitch::NotePitch {
        self.root
    }
}
