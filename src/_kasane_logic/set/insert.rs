use crate::id::DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single};
use crate::id::relation::{Relation, relation};
use crate::id::{DimensionRange, SpaceTimeId};
use crate::set::SpaceTimeIdSet;

impl SpaceTimeIdSet {
    /// Inserts a `SpaceTimeId` into the `SpaceTimeIdSet`, avoiding redundant or overlapping entries.
    ///
    /// - If the set is empty, the ID is added directly.
    /// - If an existing ID fully contains the new one, nothing is added.
    /// - If there is a partial overlap, only the non-overlapping portion is inserted.
    /// - If no overlap, it is inserted as-is.
    ///
    /// # Arguments
    ///
    /// * `other` - The `SpaceTimeId` to insert.
    pub fn insert(&mut self, other: SpaceTimeId) {
        if self.is_empty() {
            self.inner.push(Self::optimal_z(Self::optimal_i(other)));
            return;
        }

        let mut should_insert = true;

        for stid in &self.inner {
            match relation(*stid, other) {
                Relation::Equal(v) => {
                    // 既に包含されている or 完全一致 → 追加不要
                    return;
                }
                Relation::Subset(existing) => {
                    //新しいIDが既存のIDを包含している場合、追加が必要なエリアを考えて、計算する
                    let existing_set = SpaceTimeIdSet::from(existing);
                    let new_set = SpaceTimeIdSet::from(other);
                    let difference = new_set & !existing_set;
                    let result = self.clone() | difference;
                    self.inner = result.inner;
                    should_insert = false;
                    break;
                }
                Relation::Superset(existing) => {
                    //この場合には既存のIDが新しいIDを完全に包含している
                    return;
                }
                Relation::Overlap(intersection) => {
                    // 部分的に重なる場合
                    let overlap_set = SpaceTimeIdSet::from(intersection);

                    let new_set = SpaceTimeIdSet::from(other);

                    let difference = new_set & !overlap_set;

                    let result = self.clone() | difference;
                    self.inner = result.inner;
                    should_insert = false;
                    break;
                }
                Relation::Disjoint => {
                    continue;
                }
            }
        }

        if should_insert {
            Self::optimal_push(self, other);
        }
    }

    fn scale_range_for_z_u32(range: DimensionRange<u32>, delta_z: u8) -> DimensionRange<u32> {
        let scale = u32::from(2_u16.pow(delta_z as u32));
        match range {
            Single(_) => {
                panic!("このパターンは上位で除外されているはず");
            }
            LimitRange(s, e) => LimitRange(s / scale, e / scale),
            AfterUnLimitRange(s) => AfterUnLimitRange(s / scale),
            BeforeUnLimitRange(e) => BeforeUnLimitRange(e / scale),
            Any => Any,
        }
    }

    fn scale_range_for_z_i32(range: DimensionRange<i32>, delta_z: u8) -> DimensionRange<i32> {
        let scale = 2_i32.pow(delta_z as u32);

        match range {
            Single(_) => {
                panic!("このパターンは上位で除外されているはず");
            }
            LimitRange(s, e) => LimitRange(s / scale, e / scale),
            AfterUnLimitRange(s) => AfterUnLimitRange(s / scale),
            BeforeUnLimitRange(e) => BeforeUnLimitRange(e / scale),
            Any => Any,
        }
    }

    // Zに関する最適化を行う関数
    fn optimal_z(other: SpaceTimeId) -> SpaceTimeId {
        let x = match Self::optimal_xy_max_z(other.x(), other.z()) {
            Some(v) => v,
            None => return other,
        };

        let y = match Self::optimal_xy_max_z(other.y(), other.z()) {
            Some(v) => v,
            None => return other,
        };
        let f = match Self::optimal_f_max_z(other.f(), other.z()) {
            Some(v) => v,
            None => return other,
        };

        let max_z = x.max(y).max(f);

        let delta_z = other.z() - max_z;

        let new_x = Self::scale_range_for_z_u32(other.x(), delta_z);
        let new_y = Self::scale_range_for_z_u32(other.y(), delta_z);
        let new_f = Self::scale_range_for_z_i32(other.f(), delta_z);

        SpaceTimeId::new(max_z, new_f, new_x, new_y, other.i(), other.t()).unwrap()
    }

    /// その次元範囲に対する最適ZoomLevelを計算する（z: `u16`, 戻り値: `Option<u16>`）
    /// 最適値が今と同じ場合はNoneを返す
    fn optimal_max_z_for_range<T, F>(range: DimensionRange<T>, z: u8, to_u64: F) -> Option<u8>
    where
        F: Fn(T) -> u64,
    {
        let result = match range {
            Single(_) => return None,
            LimitRange(s, e) => {
                let len = to_u64(e).saturating_sub(to_u64(s)).saturating_add(1);
                z.saturating_sub(Self::count_trailing_zeros(len))
            }
            BeforeUnLimitRange(e) => {
                let len = to_u64(e).saturating_add(1);
                z.saturating_sub(Self::count_trailing_zeros(len))
            }
            AfterUnLimitRange(s) => {
                let max = 1u64 << z;
                let len = max.saturating_sub(to_u64(s));
                z.saturating_sub(Self::count_trailing_zeros(len))
            }
            Any => 0,
        };

        if result == z { None } else { Some(result) }
    }

    /// XY（u32）次元用
    fn optimal_xy_max_z(range: DimensionRange<u32>, z: u8) -> Option<u8> {
        Self::optimal_max_z_for_range(range, z, |x| x as u64)
    }

    /// F（i32）次元用
    fn optimal_f_max_z(range: DimensionRange<i32>, z: u8) -> Option<u8> {
        match range {
            Single(_) => None,
            LimitRange(s, e) => {
                if e == !0 {
                    return None;
                } else if e % 2 == 1 {
                    let len = e.saturating_sub(s).unsigned_abs() as u64 + 1;
                    Some(z.saturating_sub(Self::count_trailing_zeros(len))).filter(|&res| res != z)
                } else {
                    return None;
                }
            }
            BeforeUnLimitRange(e) => Self::optimal_f_max_z(LimitRange(0, e), z),
            AfterUnLimitRange(s) => {
                let max = 1i32 << z;
                Self::optimal_f_max_z(LimitRange(s, max), z)
            }
            Any => Some(0),
        }
    }

    /// その数が2で何回割れるかを返す（戻り値: u16）
    fn count_trailing_zeros(mut n: u64) -> u8 {
        let mut count = 0u8;
        while n % 2 == 0 && n != 0 {
            n /= 2;
            count += 1;
        }
        count
    }

    //Iに関する最適化を行う関数
    fn optimal_i(other: SpaceTimeId) -> SpaceTimeId {
        let start;
        let end;

        match other.t() {
            LimitRange(s, e) => {
                start = s;
                end = e + 1
            }
            BeforeUnLimitRange(e) => {
                start = 0;
                end = e + 1
            }
            AfterUnLimitRange(_) => return other,
            Single(s) => {
                start = s;
                end = s + 1
            }
            Any => return other,
        }
        let start = other.i() * start;
        let end = other.i() * end;

        let gcd = SpaceTimeId::gcd(start, end);

        if gcd == other.i() {
            return other;
        } else {
            return SpaceTimeId::new(
                other.z(),
                other.f(),
                other.x(),
                other.y(),
                gcd,
                LimitRange(start / gcd, end / gcd - 1),
            )
            .unwrap();
        }
    }

    //連続最適化を行う関数
    fn optimal_push(&mut self, other: SpaceTimeId) {
        for stid in &mut self.inner {
            // Zoom level and interval must match to allow merging
            if stid.z() != other.z() || stid.i() != other.i() {
                continue;
            }

            let matches = [
                stid.x() == other.x(),
                stid.y() == other.y(),
                stid.f() == other.f(),
                stid.t() == other.t(),
            ];

            let match_count = matches.iter().filter(|&&m| m).count();

            if match_count != 3 {
                continue;
            }

            let merged = if !matches[0] {
                Self::to_continuous_xy(stid.x(), other.x())
                    .ok()
                    .flatten()
                    .map(|merged_x| {
                        SpaceTimeId::new(stid.z(), stid.f(), merged_x, stid.y(), stid.i(), stid.t())
                    })
            } else if !matches[1] {
                Self::to_continuous_xy(stid.y(), other.y())
                    .ok()
                    .flatten()
                    .map(|merged_y| {
                        SpaceTimeId::new(stid.z(), stid.f(), stid.x(), merged_y, stid.i(), stid.t())
                    })
            } else if !matches[2] {
                Self::to_continuous_f(stid.f(), other.f())
                    .ok()
                    .flatten()
                    .map(|merged_f| {
                        SpaceTimeId::new(stid.z(), merged_f, stid.x(), stid.y(), stid.i(), stid.t())
                    })
            } else if !matches[3] {
                Self::to_continuous_t(stid.t(), other.t())
                    .ok()
                    .flatten()
                    .map(|merged_t| {
                        SpaceTimeId::new(stid.z(), stid.f(), stid.x(), stid.y(), stid.i(), merged_t)
                    })
            } else {
                None
            };

            if let Some(Ok(new_stid)) = merged {
                *stid = Self::optimal_z(Self::optimal_i(new_stid));
                return; // merged successfully
            }
        }
        //ZとIに関して粒度の最適化を実施
        self.inner.push(Self::optimal_z(Self::optimal_i(other)));
    }

    fn to_continuous_xy(
        target: DimensionRange<u32>,
        other: DimensionRange<u32>,
    ) -> Result<Option<DimensionRange<u32>>, String> {
        Self::to_continuous_range(target, other)
    }

    fn to_continuous_f(
        target: DimensionRange<i32>,
        other: DimensionRange<i32>,
    ) -> Result<Option<DimensionRange<i32>>, String> {
        Self::to_continuous_range(target, other)
    }

    fn to_continuous_t(
        target: DimensionRange<u32>,
        other: DimensionRange<u32>,
    ) -> Result<Option<DimensionRange<u32>>, String> {
        Self::to_continuous_range(target, other)
    }

    fn to_continuous_range<T>(
        target: DimensionRange<T>,
        other: DimensionRange<T>,
    ) -> Result<Option<DimensionRange<T>>, String>
    where
        T: Copy
            + PartialOrd
            + Eq
            + std::fmt::Debug
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + From<u8>,
    {
        match target {
            Single(v) => match other {
                Single(s) => {
                    if v + T::from(1) == s {
                        Ok(Some(LimitRange(v, s)))
                    } else if s + T::from(1) == v {
                        Ok(Some(LimitRange(s, v)))
                    } else {
                        Ok(None)
                    }
                }
                LimitRange(s, e) => {
                    if s > v {
                        if s - T::from(1) == v {
                            Ok(Some(LimitRange(v, e)))
                        } else {
                            Ok(None)
                        }
                    } else if e < v {
                        if e + T::from(1) == v {
                            Ok(Some(LimitRange(s, v)))
                        } else {
                            Ok(None)
                        }
                    } else {
                        Ok(None)
                    }
                }
                AfterUnLimitRange(s) => {
                    if s - T::from(1) == v {
                        Ok(Some(AfterUnLimitRange(v)))
                    } else {
                        Ok(None)
                    }
                }
                BeforeUnLimitRange(e) => {
                    if e + T::from(1) == v {
                        Ok(Some(BeforeUnLimitRange(v)))
                    } else {
                        Ok(None)
                    }
                }
                Any => Err("重なりがある値が入力されました".to_string()),
            },
            LimitRange(vs, ve) => match other {
                Single(_) => Self::to_continuous_range(other, target),
                LimitRange(s, e) => {
                    if ve + T::from(1) == s {
                        Ok(Some(LimitRange(vs, e)))
                    } else if e + T::from(1) == vs {
                        Ok(Some(LimitRange(s, ve)))
                    } else {
                        Ok(None)
                    }
                }
                AfterUnLimitRange(s) => {
                    if ve + T::from(1) == s {
                        Ok(Some(AfterUnLimitRange(vs)))
                    } else {
                        Ok(None)
                    }
                }
                BeforeUnLimitRange(e) => {
                    if e + T::from(1) == vs {
                        Ok(Some(BeforeUnLimitRange(ve)))
                    } else {
                        Ok(None)
                    }
                }
                Any => Err("重なりがある値が入力されました".to_string()),
            },
            AfterUnLimitRange(vs) => match other {
                BeforeUnLimitRange(e) => {
                    if vs + T::from(1) == e {
                        Ok(Some(Any))
                    } else {
                        Ok(None)
                    }
                }
                AfterUnLimitRange(_) => Err("重なりがある値が入力されました".to_string()),
                Any => Err("重なりがある値が入力されました".to_string()),
                _ => Self::to_continuous_range(other, target),
            },
            BeforeUnLimitRange(_) => match other {
                BeforeUnLimitRange(_) => Err("重なりがある値が入力されました".to_string()),
                Any => Err("重なりがある値が入力されました".to_string()),
                _ => Self::to_continuous_range(other, target),
            },
            Any => Err("重なりがある値が入力されました".to_string()),
        }
    }
}
