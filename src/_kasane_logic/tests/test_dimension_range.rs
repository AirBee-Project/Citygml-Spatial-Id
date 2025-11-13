use crate::id::DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single};

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for DimensionRange Display trait
    #[test]
    fn test_dimension_range_display() {
        assert_eq!(Single(5u32).to_string(), "5");
        assert_eq!(LimitRange(10u32, 20u32).to_string(), "10:20");
        assert_eq!(BeforeUnLimitRange(15u32).to_string(), "-:15");
        assert_eq!(AfterUnLimitRange(25u32).to_string(), "25:-");
        assert_eq!(Any::<u32>.to_string(), "-");
    }

    #[test]
    fn test_dimension_range_display_negative() {
        assert_eq!(Single(-5i32).to_string(), "-5");
        assert_eq!(LimitRange(-10i32, -5i32).to_string(), "-10:-5");
        assert_eq!(BeforeUnLimitRange(-1i32).to_string(), "-:-1");
        assert_eq!(AfterUnLimitRange(-5i32).to_string(), "-5:-");
    }

    #[test]
    fn test_dimension_range_equality() {
        assert_eq!(Single(5u32), Single(5u32));
        assert_eq!(LimitRange(1u32, 10u32), LimitRange(1u32, 10u32));
        assert_eq!(Any::<u32>, Any::<u32>);

        assert_ne!(Single(5u32), Single(6u32));
        assert_ne!(LimitRange(1u32, 10u32), LimitRange(1u32, 11u32));
    }

    #[test]
    fn test_dimension_range_copy_clone() {
        let range = Single(42u32);
        let copied = range;
        let cloned = range.clone();

        assert_eq!(range, copied);
        assert_eq!(range, cloned);
    }
}
