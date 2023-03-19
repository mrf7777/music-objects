#![deny(clippy::all, clippy::pedantic)]

use std::collections::BTreeMap;

use crate::rhythm;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Timeline<V> {
    elements: BTreeMap<rhythm::Duration, V>,
}
