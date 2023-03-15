#![deny(clippy::all, clippy::pedantic)]

use std::collections::BTreeMap;

use crate::rhythm;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RelativeTimeline<V> {
    elements: BTreeMap<rhythm::Duration, V>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Timeline<V> {
    relative_timeline: RelativeTimeline<V>,
    tempo: rhythm::Tempo,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Marker {
    name: String,
    position: rhythm::Duration,
}
