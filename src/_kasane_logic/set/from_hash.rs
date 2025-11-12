use std::collections::HashSet;

use crate::{id::SpaceTimeId, set::SpaceTimeIdSet};

impl SpaceTimeIdSet {
    pub unsafe fn from_hash(other: HashSet<SpaceTimeId>) -> Self {
        Self {
            inner: other.iter().cloned().collect(),
        }
    }
}
