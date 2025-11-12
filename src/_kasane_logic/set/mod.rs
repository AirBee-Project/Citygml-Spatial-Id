use crate::id::{DimensionRange, SpaceTimeId};
pub mod and;
pub mod equal;
pub mod from_hash;
pub mod insert;
pub mod not;
pub mod or;
pub mod pure;
pub mod xor;

#[derive(Clone)]
/// This type represents a set of `SpaceTimeId` elements.
///
/// When a new `SpaceTimeId` is added, any overlapping ranges with existing elements
/// will be subtracted from the new one to ensure that the ranges represented by IDs
/// in the set remain disjoint (i.e., no overlap between any two entries).
///
/// As a result, each distinct region in the physical space-time domain is guaranteed
/// to be represented by *only one* `SpaceTimeId` within the set.
/// This ensures unambiguous mapping between a spatial-temporal region and its identifier.

#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]

pub struct SpaceTimeIdSet {
    inner: Vec<SpaceTimeId>,
}

impl SpaceTimeIdSet {
    /// Creates a new, empty `SpaceTimeIdSet`.
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
    /// Returns an iterator over the `SpaceTimeId` elements contained in the set.
    /// This allows read-only access to each element in the set.
    pub fn iter(&self) -> impl Iterator<Item = &SpaceTimeId> {
        self.inner.iter()
    }
    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

use std::{convert::From, fmt};

impl From<SpaceTimeId> for SpaceTimeIdSet {
    fn from(id: SpaceTimeId) -> Self {
        let mut set = SpaceTimeIdSet::new();
        set.insert(id);
        set
    }
}

impl fmt::Display for SpaceTimeIdSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.inner.iter().map(|stid| stid.to_string()).collect();
        write!(f, "{}", elements.join(", "))
    }
}
impl fmt::Debug for SpaceTimeIdSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        for (i, stid) in self.inner.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", stid)?;
        }
        write!(f, " ]")
    }
}

impl IntoIterator for SpaceTimeIdSet {
    type Item = SpaceTimeId;
    type IntoIter = std::vec::IntoIter<SpaceTimeId>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a SpaceTimeIdSet {
    type Item = &'a SpaceTimeId;
    type IntoIter = std::slice::Iter<'a, SpaceTimeId>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl FromIterator<SpaceTimeId> for SpaceTimeIdSet {
    fn from_iter<I: IntoIterator<Item = SpaceTimeId>>(iter: I) -> Self {
        let mut set = SpaceTimeIdSet::new();
        for stid in iter {
            set.insert(stid);
        }
        set
    }
}
