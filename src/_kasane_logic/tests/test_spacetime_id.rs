use crate::id::SpaceTimeId;
use crate::id::DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single};

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for SpaceTimeId::new() - Success cases
    #[test]
    fn test_spacetime_id_new_valid() {
        let stid = SpaceTimeId::new(
            4,
            Single(10),
            Single(5),
            Single(3),
            60,
            Single(100),
        );
        assert!(stid.is_ok());
        
        let stid = stid.unwrap();
        assert_eq!(stid.z(), 4);
        assert_eq!(stid.f(), Single(10));
        assert_eq!(stid.x(), Single(5));
        assert_eq!(stid.y(), Single(3));
        assert_eq!(stid.i(), 60);
        assert_eq!(stid.t(), Single(100));
    }

    #[test]
    fn test_spacetime_id_new_with_ranges() {
        let stid = SpaceTimeId::new(
            3,
            LimitRange(-2, 4),
            LimitRange(0, 3),
            LimitRange(1, 5),
            30,
            LimitRange(10, 20),
        );
        assert!(stid.is_ok());
    }

    #[test]
    fn test_spacetime_id_new_with_any() {
        let stid = SpaceTimeId::new(
            2,
            Any,
            Any,
            Any,
            0,
            Any,
        );
        assert!(stid.is_ok());
    }

    // Tests for SpaceTimeId::new() - Error cases
    #[test]
    fn test_spacetime_id_new_z_too_large() {
        let result = SpaceTimeId::new(
            32,
            Single(0),
            Single(0),
            Single(0),
            0,
            Any,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Zoom level z must be less than 32"));
    }

    #[test]
    fn test_spacetime_id_new_z_at_boundary() {
        let result = SpaceTimeId::new(
            31,
            Single(0),
            Single(0),
            Single(0),
            0,
            Any,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_spacetime_id_new_x_out_of_bounds() {
        let result = SpaceTimeId::new(
            2,
            Single(0), // f
            Single(4), // x - max is 3 for z=2
            Single(0), // y
            0,
            Any,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid x dimension"));
    }

    #[test]
    fn test_spacetime_id_new_y_out_of_bounds() {
        let result = SpaceTimeId::new(
            2,
            Single(0), // f
            Single(0), // x
            Single(5), // y - max is 3 for z=2
            0,
            Any,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid y dimension"));
    }

    #[test]
    fn test_spacetime_id_new_f_out_of_bounds_positive() {
        let result = SpaceTimeId::new(
            2,
            Single(4), // f - max is 3 for z=2
            Single(0), // x
            Single(0), // y
            0,
            Any,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid f dimension"));
    }

    #[test]
    fn test_spacetime_id_new_f_out_of_bounds_negative() {
        let result = SpaceTimeId::new(
            2,
            Single(-5), // min is -4 for z=2
            Single(0),
            Single(0),
            0,
            Any,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid f dimension"));
    }

    #[test]
    fn test_spacetime_id_new_range_start_greater_than_end() {
        let result = SpaceTimeId::new(
            3,
            LimitRange(5, 2), // start > end
            Single(0),
            Single(0),
            0,
            Any,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("start value 5 cannot be greater than end value 2"));
    }

    #[test]
    fn test_spacetime_id_new_t_not_any_when_i_zero() {
        let result = SpaceTimeId::new(
            2,
            Single(0),
            Single(0),
            Single(0),
            0,
            Single(100), // must be Any when i=0
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("t must be Any when time interval i is 0"));
    }

    // Tests for normalization
    #[test]
    fn test_spacetime_id_normalization_equal_range_to_single() {
        let stid = SpaceTimeId::new(
            3,
            Single(0), // f
            LimitRange(2, 2), // x - equal start and end
            Single(0), // y
            60,
            Single(10),
        ).unwrap();
        
        assert_eq!(stid.x(), Single(2));
    }

    #[test]
    fn test_spacetime_id_normalization_full_range_to_any() {
        let stid = SpaceTimeId::new(
            2,
            LimitRange(-4, 3), // full range for z=2
            LimitRange(0, 3), // full range for z=2
            Single(1),
            60,
            Single(10),
        ).unwrap();
        
        assert_eq!(stid.f(), Any);
        assert_eq!(stid.x(), Any);
    }

    #[test]
    fn test_spacetime_id_normalization_start_zero_to_before_unlimit() {
        let stid = SpaceTimeId::new(
            3,
            LimitRange(-8, 5), // start is min_f (-8) for z=3
            Single(1),
            Single(0),
            60,
            Single(10),
        ).unwrap();
        
        assert_eq!(stid.f(), BeforeUnLimitRange(5));
    }

    #[test]
    fn test_spacetime_id_normalization_end_max_to_after_unlimit() {
        let stid = SpaceTimeId::new(
            3,
            LimitRange(2, 7), // end is max (7) for z=3
            Single(1),
            Single(0),
            60,
            Single(10),
        ).unwrap();
        
        assert_eq!(stid.f(), AfterUnLimitRange(2));
    }

    #[test]
    fn test_spacetime_id_normalization_after_unlimit_start_zero() {
        let stid = SpaceTimeId::new(
            2,
            AfterUnLimitRange(-4), // start is min_f (-4) for z=2
            Single(1),
            Single(0),
            60,
            Single(10),
        ).unwrap();
        
        assert_eq!(stid.f(), Any);
    }

    #[test]
    fn test_spacetime_id_normalization_after_unlimit_start_max() {
        let stid = SpaceTimeId::new(
            2,
            Single(0), // f
            AfterUnLimitRange(3), // x - start is max (3) for z=2
            Single(0), // y
            60,
            Single(10),
        ).unwrap();
        
        assert_eq!(stid.x(), Single(3));
    }

    #[test]
    fn test_spacetime_id_normalization_before_unlimit_end_max() {
        let stid = SpaceTimeId::new(
            2,
            BeforeUnLimitRange(3), // end is max (3) for z=2
            Single(1),
            Single(0),
            60,
            Single(10),
        ).unwrap();
        
        assert_eq!(stid.f(), Any);
    }

    #[test]
    fn test_spacetime_id_normalization_before_unlimit_end_zero() {
        let stid = SpaceTimeId::new(
            2,
            Single(0), // f
            BeforeUnLimitRange(0), // x - end is 0 (minimum for x)
            Single(0), // y
            60,
            Single(10),
        ).unwrap();
        
        assert_eq!(stid.x(), Single(0));
    }

    // Tests for getter functions
    #[test]
    fn test_spacetime_id_getters() {
        let stid = SpaceTimeId::new(
            5,
            BeforeUnLimitRange(-5),
            LimitRange(10, 20),
            AfterUnLimitRange(15),
            120,
            Any,
        ).unwrap();

        assert_eq!(stid.z(), 5);
        assert_eq!(stid.f(), BeforeUnLimitRange(-5));
        assert_eq!(stid.x(), LimitRange(10, 20));
        assert_eq!(stid.y(), AfterUnLimitRange(15));
        assert_eq!(stid.i(), 120);
        assert_eq!(stid.t(), Any);
    }

    // Tests for Display trait
    #[test]
    fn test_spacetime_id_display() {
        let stid = SpaceTimeId::new(
            4,
            Single(5),
            Single(3),
            Single(10),
            60,
            Single(100),
        ).unwrap();

        assert_eq!(stid.to_string(), "4/5/3/10_60/100");
    }

    #[test]
    fn test_spacetime_id_display_with_ranges() {
        let stid = SpaceTimeId::new(
            3,
            AfterUnLimitRange(-2),
            LimitRange(1, 5),
            Any,
            0,
            Any,
        ).unwrap();

        assert_eq!(stid.to_string(), "3/-2:-/1:5/-_0/-");
    }

    // Tests for equality and cloning
    #[test]
    fn test_spacetime_id_equality() {
        let stid1 = SpaceTimeId::new(
            4,
            Single(10),
            Single(5),
            Single(3),
            60,
            Single(100),
        ).unwrap();

        let stid2 = SpaceTimeId::new(
            4,
            Single(10),
            Single(5),
            Single(3),
            60,
            Single(100),
        ).unwrap();

        let stid3 = SpaceTimeId::new(
            4,
            Single(10),
            Single(6), // different x
            Single(3),
            60,
            Single(100),
        ).unwrap();

        assert_eq!(stid1, stid2);
        assert_ne!(stid1, stid3);
    }

    #[test]
    fn test_spacetime_id_clone() {
        let stid = SpaceTimeId::new(
            4,
            Single(5),
            Single(3),
            Single(10),
            60,
            Single(100),
        ).unwrap();

        let cloned = stid.clone();
        assert_eq!(stid, cloned);
    }

    // Edge cases and boundary conditions
    #[test]
    fn test_spacetime_id_zero_zoom() {
        let stid = SpaceTimeId::new(
            0,
            Single(0),
            Single(0),
            Single(0),
            0,
            Any,
        );
        assert!(stid.is_ok());
        
        let stid = stid.unwrap();
        assert_eq!(stid.x(), Single(0));
        assert_eq!(stid.y(), Single(0));
        assert_eq!(stid.f(), Single(0));
    }

    #[test]
    fn test_spacetime_id_max_zoom() {
        let stid = SpaceTimeId::new(
            31,
            Single(0),
            Single(0),
            Single(0),
            0,
            Any,
        );
        assert!(stid.is_ok());
    }

    #[test]
    fn test_spacetime_id_large_values() {
        let stid = SpaceTimeId::new(
            10,
            Single(1023), // 2^10 - 1
            Single(1023),
            Single(1023), // 2^10 - 1
            u32::MAX,
            Single(u32::MAX),
        );
        assert!(stid.is_ok());
    }

    #[test]
    fn test_spacetime_id_negative_f_bounds() {
        let stid = SpaceTimeId::new(
            3,
            Single(-8), // min for z=3 is -8
            Single(0),
            Single(0),
            60,
            Single(0),
        );
        assert!(stid.is_ok());
    }
}