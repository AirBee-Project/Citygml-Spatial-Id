use crate::{id::SpaceTimeId, set::SpaceTimeIdSet};

impl SpaceTimeIdSet {
    /// Expands all `SpaceTimeId` elements in the set to their pure (single-value) form.
    ///
    /// This method calls the `pure()` method on each `SpaceTimeId` in the set and collects
    /// all resulting individual IDs into a single vector. Each returned ID will have
    /// single values for spatial dimensions (F, X, Y) while preserving the original
    /// time dimension ranges.
    ///
    /// # Returns
    ///
    /// A `Vec<SpaceTimeId>` containing all individual space-time IDs with single values
    /// that collectively represent the same space-time regions as the original set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kasane_logic::{id::{DimensionRange, SpaceTimeId}, set::SpaceTimeIdSet};
    ///
    /// let stid = SpaceTimeId::new(2, DimensionRange::LimitRange(0, 1),
    ///                            DimensionRange::Single(1), DimensionRange::Single(1),
    ///                            0, DimensionRange::Any).unwrap();
    /// let set = SpaceTimeIdSet::from(stid);
    /// let pure_ids = set.pure(); // Returns multiple IDs with F dimension expanded
    /// ```
    pub fn pure(&self) -> Vec<SpaceTimeId> {
        let mut result = vec![];
        for stid in &self.inner {
            let stid_pure = stid.pure();
            for pure in stid_pure {
                result.push(pure);
            }
        }
        return result;
    }
}
