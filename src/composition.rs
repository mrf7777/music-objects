use std::collections::BTreeMap;

use crate::rhythm;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Timeline<V> {
    elements: BTreeMap<rhythm::Duration, V>,
}
