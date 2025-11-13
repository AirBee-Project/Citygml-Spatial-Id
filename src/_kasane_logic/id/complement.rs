use std::collections::HashSet;

use crate::{
    id::{
        z_range::{F_MAX, F_MIN, XY_MAX},
        DimensionRange::{self, AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single},
        SpaceTimeId,
    },
    set::SpaceTimeIdSet,
};

impl SpaceTimeId {
    /// Calculates the complement of the `SpaceTimeId`.
    ///
    /// The complement represents all space and time *not* covered by this ID.
    /// This is achieved by inverting each dimension's range and then computing the
    /// Cartesian product of all resulting inverted ranges.
    ///
    /// # Returns
    ///
    /// A `SpaceTimeIdSet` containing the set of IDs that represent the complement space.
    /// Returns an empty SpaceTimeIdSet if the complement is an empty set (e.g., the complement of "everything").
    pub fn complement(&self) -> SpaceTimeIdSet {
        if self.x == Any && self.y == Any && self.f == Any {
            return SpaceTimeIdSet::new();
        }

        // i=0かつt=Anyの場合は時間次元はそのまま
        let is_pure_space = self.i == 0 && self.t == Any;

        // z=0ではデフォルトで全体を表すので、早期リターンする
        if self.z == 0 {
            return SpaceTimeIdSet::new();
        }

        let x_inversions = Self::split_xy_dimension(&self.x, self.z);
        let y_inversions = Self::split_xy_dimension(&self.y, self.z);
        let f_inversions = Self::split_f_dimension(&self.f, self.z);
        let t_inversions = if is_pure_space {
            vec![self.t] // 時間次元はそのまま
        } else {
            Self::split_t_dimension(&self.t)
        };

        /// None は未確定の値を意味する
        #[derive(Debug, Clone, Copy)]
        struct FXY {
            f: Option<DimensionRange<i32>>,
            x: Option<DimensionRange<u32>>,
            y: Option<DimensionRange<u32>>,
        }

        let mut tmp = Vec::new();
        let mut tmp_hash = HashSet::new();

        // X の値に関して処理
        match x_inversions.as_slice() {
            [] => tmp.push(FXY {
                f: None,
                x: Some(self.x),
                y: None,
            }),
            [a] => {
                tmp.push(FXY {
                    f: Some(Any),
                    x: Some(*a),
                    y: Some(Any),
                });
                tmp.push(FXY {
                    f: None,
                    x: Some(self.x),
                    y: None,
                });
            }
            [a, b] => {
                tmp.push(FXY {
                    f: Some(Any),
                    x: Some(*a),
                    y: Some(Any),
                });
                tmp.push(FXY {
                    f: None,
                    x: Some(self.x),
                    y: None,
                });
                tmp.push(FXY {
                    f: Some(Any),
                    x: Some(*b),
                    y: Some(Any),
                });
            }
            _ => panic!("x_inversions の配列長がおかしい！"),
        }

        // Y の補集合を展開
        tmp = tmp
            .into_iter()
            .flat_map(|a| {
                if a.y.is_none() {
                    match y_inversions.as_slice() {
                        [y1, y2] => vec![
                            FXY {
                                f: Some(Any),
                                y: Some(*y1),
                                ..a
                            },
                            FXY {
                                y: Some(self.y),
                                ..a
                            },
                            FXY {
                                f: Some(Any),
                                y: Some(*y2),
                                ..a
                            },
                        ],
                        [y1] => vec![
                            FXY {
                                f: Some(Any),
                                y: Some(*y1),
                                ..a
                            },
                            FXY {
                                y: Some(self.y),
                                ..a
                            },
                        ],
                        [] => vec![FXY {
                            f: None,
                            y: Some(Any),
                            ..a
                        }],
                        _ => panic!("y_inversions の配列長がおかしい！"),
                    }
                } else {
                    vec![a]
                }
            })
            .collect();

        // F の補集合を展開
        tmp = tmp
            .into_iter()
            .flat_map(|a| {
                if a.f.is_none() {
                    match f_inversions.as_slice() {
                        [f1, f2] => vec![FXY { f: Some(*f1), ..a }, FXY { f: Some(*f2), ..a }],
                        [f1] => vec![FXY { f: Some(*f1), ..a }],
                        [] => vec![],
                        _ => panic!("f_inversions の配列長がおかしい！"),
                    }
                } else {
                    vec![a]
                }
            })
            .collect();

        println!("{:?}", t_inversions);

        // 時間軸の補集合を展開
        for ele in tmp {
            match t_inversions.as_slice() {
                [t1, t2] => {
                    let id = SpaceTimeId::new(
                        self.z,
                        ele.f.expect("f 未設定"),
                        ele.x.expect("x 未設定"),
                        ele.y.expect("y 未設定"),
                        self.i,
                        self.t,
                    )
                    .expect("補集合 SpaceTimeId の生成に失敗");
                    tmp_hash.insert(id);

                    let id = SpaceTimeId::new(1, Any, Any, Any, self.i, *t1)
                        .expect("補集合 SpaceTimeId の生成に失敗");
                    tmp_hash.insert(id);

                    let id = SpaceTimeId::new(1, Any, Any, Any, self.i, *t2)
                        .expect("補集合 SpaceTimeId の生成に失敗");
                    tmp_hash.insert(id);
                }
                [t1] => {
                    let id = SpaceTimeId::new(
                        self.z,
                        ele.f.expect("f 未設定"),
                        ele.x.expect("x 未設定"),
                        ele.y.expect("y 未設定"),
                        self.i,
                        self.t,
                    )
                    .expect("補集合 SpaceTimeId の生成に失敗");
                    tmp_hash.insert(id);

                    let id = SpaceTimeId::new(1, Any, Any, Any, self.i, *t1)
                        .expect("補集合 SpaceTimeId の生成に失敗");
                    tmp_hash.insert(id);
                }
                [] => {
                    let id = SpaceTimeId::new(
                        self.z,
                        ele.f.expect("f 未設定"),
                        ele.x.expect("x 未設定"),
                        ele.y.expect("y 未設定"),
                        self.i,
                        self.t,
                    )
                    .expect("補集合 SpaceTimeId の生成に失敗");
                    tmp_hash.insert(id);
                }
                _ => panic!("t_inversions の配列長がおかしい！"),
            }
        }
        unsafe { SpaceTimeIdSet::from_hash(tmp_hash) }
    }

    /// Inverts a spatial dimension range (x or y) for complement calculation.
    fn split_xy_dimension(dim_range: &DimensionRange<u32>, z: u8) -> Vec<DimensionRange<u32>> {
        let max = XY_MAX[z as usize]; // << 変更
        match dim_range {
            Single(v) => {
                if *v == 0 {
                    vec![DimensionRange::LimitRange(1, max)]
                } else if *v == max {
                    vec![DimensionRange::LimitRange(0, max - 1)]
                } else {
                    vec![
                        DimensionRange::LimitRange(0, v - 1),
                        DimensionRange::LimitRange(v + 1, max),
                    ]
                }
            }
            DimensionRange::LimitRange(s, e) => {
                let mut res = Vec::new();
                if *s > 0 {
                    res.push(DimensionRange::LimitRange(0, s - 1));
                }
                if *e < max {
                    res.push(DimensionRange::LimitRange(e + 1, max));
                }
                res
            }
            DimensionRange::AfterUnLimitRange(s) => {
                if *s == 0 {
                    vec![]
                } else {
                    vec![DimensionRange::LimitRange(0, s - 1)]
                }
            }
            DimensionRange::BeforeUnLimitRange(e) => {
                if *e == max {
                    vec![]
                } else {
                    vec![DimensionRange::LimitRange(e + 1, max)]
                }
            }
            DimensionRange::Any => vec![],
        }
    }

    /// F の補集合を作る
    fn split_f_dimension(dim_range: &DimensionRange<i32>, z: u8) -> Vec<DimensionRange<i32>> {
        let max = F_MAX[z as usize]; // << 変更
        let min = F_MIN[z as usize]; // << 変更
        match dim_range {
            Single(v) => {
                if *v == min {
                    vec![DimensionRange::LimitRange(min + 1, max)]
                } else if *v == max {
                    vec![DimensionRange::LimitRange(min, max - 1)]
                } else {
                    vec![
                        DimensionRange::LimitRange(min, v - 1),
                        DimensionRange::LimitRange(v + 1, max),
                    ]
                }
            }
            DimensionRange::LimitRange(s, e) => {
                let mut res = Vec::new();
                if *s > min {
                    res.push(DimensionRange::LimitRange(min, s - 1));
                }
                if *e < max {
                    res.push(DimensionRange::LimitRange(e + 1, max));
                }
                res
            }
            DimensionRange::AfterUnLimitRange(s) => {
                if *s <= min {
                    vec![]
                } else {
                    vec![DimensionRange::LimitRange(min, s - 1)]
                }
            }
            DimensionRange::BeforeUnLimitRange(e) => {
                if *e >= max {
                    vec![]
                } else {
                    vec![DimensionRange::LimitRange(e + 1, max)]
                }
            }
            DimensionRange::Any => vec![],
        }
    }

    /// Inverts a temporal dimension range (t) for complement calculation.
    fn split_t_dimension(dim_range: &DimensionRange<u32>) -> Vec<DimensionRange<u32>> {
        match dim_range {
            Single(v) => {
                if *v == 0 {
                    vec![AfterUnLimitRange(1)]
                } else {
                    vec![LimitRange(0, v - 1), AfterUnLimitRange(v + 1)]
                }
            }
            LimitRange(s, e) => {
                let mut res = Vec::new();
                if *s > 0 {
                    res.push(LimitRange(0, s - 1));
                }
                res.push(AfterUnLimitRange(e + 1));
                res
            }
            AfterUnLimitRange(s) => {
                if *s == 0 {
                    vec![]
                } else {
                    vec![LimitRange(0, s - 1)]
                }
            }
            BeforeUnLimitRange(e) => vec![AfterUnLimitRange(e + 1)],
            Any => vec![],
        }
    }
}
