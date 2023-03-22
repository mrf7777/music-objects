use std::collections::{BTreeSet, HashSet};

use crate::{interval::DirectedSemitoneInterval, pitch};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChordClass {
    note_pitch_classes: HashSet<pitch::NotePitchClass>,
}

impl ChordClass {
    #[must_use]
    pub fn new(note_pitch_classes: HashSet<pitch::NotePitchClass>) -> Self {
        Self { note_pitch_classes }
    }

    #[must_use]
    pub fn note_pitch_classes(&self) -> &HashSet<pitch::NotePitchClass> {
        &self.note_pitch_classes
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RootedChordClass {
    chord_class: ChordClass,
    root: pitch::NotePitchClass,
}

impl RootedChordClass {
    #[must_use]
    pub fn new(chord_class: ChordClass, root: pitch::NotePitchClass) -> Self {
        Self { chord_class, root }
    }

    #[must_use]
    pub fn chord_class(&self) -> &ChordClass {
        &self.chord_class
    }

    #[must_use]
    pub fn root(&self) -> pitch::NotePitchClass {
        self.root
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Chord {
    note_pitches: BTreeSet<pitch::NotePitch>,
}

impl Chord {
    #[must_use]
    pub fn new(note_pitches: BTreeSet<pitch::NotePitch>) -> Self {
        Self { note_pitches }
    }

    #[must_use]
    pub fn note_pitchs(&self) -> &BTreeSet<pitch::NotePitch> {
        &self.note_pitches
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RootedChord {
    chord: Chord,
    root: pitch::NotePitch,
}

impl RootedChord {
    #[must_use]
    pub fn new(chord: Chord, root: pitch::NotePitch) -> Self {
        Self { chord, root }
    }

    #[must_use]
    pub fn chord(&self) -> &Chord {
        &self.chord
    }

    #[must_use]
    pub fn root(&self) -> pitch::NotePitch {
        self.root
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChordPattern {
    intervals: BTreeSet<DirectedSemitoneInterval>,
}

impl ChordPattern {
    #[must_use]
    pub fn new(intervals: BTreeSet<DirectedSemitoneInterval>) -> Self {
        Self { intervals }
    }

    #[must_use]
    pub fn intervals(&self) -> &BTreeSet<DirectedSemitoneInterval> {
        &self.intervals
    }
}
