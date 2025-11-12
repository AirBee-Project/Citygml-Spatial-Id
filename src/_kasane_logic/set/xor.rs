//! Exclusive OR (`^`) logic for `SpaceTimeIdSet`.

use std::ops::BitXor;

use crate::set::SpaceTimeIdSet;

/// Implements the `BitXor` trait (`^`) for `SpaceTimeIdSet` and its references.
///
/// This enables XOR (symmetric difference) operations like:
/// - `a ^ b`
/// - `&a ^ &b`
/// - `&mut a ^ &mut b`
///
/// The result is a new `SpaceTimeIdSet` that includes regions present in one set
/// but not in both.
macro_rules! impl_bitxor_for_spacetimeidset {
    ($($lhs:ty, $rhs:ty),+ $(,)?) => {
        $(
            impl BitXor<$rhs> for $lhs {
                type Output = SpaceTimeIdSet;

                fn bitxor(self, rhs: $rhs) -> Self::Output {
                    let lhs_ref: &SpaceTimeIdSet = match self {
                        ref s => s,
                    };
                    let rhs_ref: &SpaceTimeIdSet = match rhs {
                        ref s => s,
                    };
                    (lhs_ref & !rhs_ref) | (!lhs_ref & rhs_ref)
                }
            }
        )+
    };
}

// 所有権と参照の組合せをすべてカバー
impl_bitxor_for_spacetimeidset!(
    SpaceTimeIdSet,
    SpaceTimeIdSet,
    &SpaceTimeIdSet,
    &SpaceTimeIdSet,
    &mut SpaceTimeIdSet,
    &mut SpaceTimeIdSet,
    SpaceTimeIdSet,
    &SpaceTimeIdSet,
    &SpaceTimeIdSet,
    SpaceTimeIdSet,
    &mut SpaceTimeIdSet,
    &SpaceTimeIdSet,
    &SpaceTimeIdSet,
    &mut SpaceTimeIdSet,
    SpaceTimeIdSet,
    &mut SpaceTimeIdSet,
    &mut SpaceTimeIdSet,
    SpaceTimeIdSet,
);
