use std::ops::BitAnd;

use crate::{
    id::{SpaceTimeId, relation},
    set::SpaceTimeIdSet,
};

use crate::id::relation::{Relation, relation};

/// Implements the `&` (intersection) operator for `SpaceTimeIdSet` and its references.
macro_rules! impl_bitand_for_spacetimeidset {
    ($lhs:ty, $rhs:ty) => {
        impl BitAnd<$rhs> for $lhs {
            type Output = SpaceTimeIdSet;

            fn bitand(self, rhs: $rhs) -> Self::Output {
                let mut result = Vec::new();

                for self_stid in &self.inner {
                    for rhs_stid in &rhs.inner {
                        match relation(*self_stid, *rhs_stid) {
                            Relation::Equal(intersection)
                            | Relation::Subset(intersection)
                            | Relation::Superset(intersection)
                            | Relation::Overlap(intersection) => {
                                result.push(intersection);
                            }
                            Relation::Disjoint => {}
                        }
                    }
                }

                SpaceTimeIdSet { inner: result }
            }
        }
    };
}

// Apply BitAnd to combinations of owned and reference types
impl_bitand_for_spacetimeidset!(SpaceTimeIdSet, SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(SpaceTimeIdSet, &SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(SpaceTimeIdSet, &mut SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(&SpaceTimeIdSet, SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(&SpaceTimeIdSet, &SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(&SpaceTimeIdSet, &mut SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(&mut SpaceTimeIdSet, SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(&mut SpaceTimeIdSet, &SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(&mut SpaceTimeIdSet, &mut SpaceTimeIdSet);
