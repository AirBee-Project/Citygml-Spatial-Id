//! Negation (`!`) logic for `SpaceTimeIdSet`.

use crate::set::SpaceTimeIdSet;
use std::ops::Not;

/// Implements the `Not` trait (`!`) for SpaceTimeIdSet and its references.
///
/// This allows complement calculation of the set via:
/// - `!set`
/// - `!&set`
/// - `!&mut set`
///
/// The result is a new `SpaceTimeIdSet` that includes all regions not currently
/// represented in the original set.
///
/// # Example
/// ```
/// use logic::{id::{DimensionRange, SpaceTimeId}, set::SpaceTimeIdSet};
///
/// // Create a spatial ID (i=0, t=Any)
/// let stid = SpaceTimeId::new(2, Single(1), Single(1),
///                             Single(1), 0, Any).unwrap();
/// let a = SpaceTimeIdSet::from(stid);
/// let not_a = !&a; // complement of a using reference to avoid move
/// ```
///
macro_rules! impl_not_for_spacetimeidset {
    ($($t:ty),+) => {
        $(
            impl Not for $t {
                type Output = SpaceTimeIdSet;

                fn not(self) -> Self::Output {
                    let mut result = SpaceTimeIdSet::new();
                    for stid in &self.inner {
                        result = result | stid.complement();
                    }
                    result
                }
            }
        )+
    };
}

// Implement for T, &T, &mut T
impl_not_for_spacetimeidset!(SpaceTimeIdSet, &SpaceTimeIdSet, &mut SpaceTimeIdSet);
