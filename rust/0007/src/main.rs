#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for x in [123, -123, 120, 1563847412, i32::MIN] {
        println!("{x} -> {}", Solution::reverse(x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_positive() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn example_2_negative() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn example_3_trailing_zero_is_dropped() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn zero() {
        assert_eq!(Solution::reverse(0), 0);
    }

    #[test]
    fn single_digit() {
        assert_eq!(Solution::reverse(7), 7);
    }

    #[test]
    fn negative_trailing_zero() {
        assert_eq!(Solution::reverse(-10), -1);
    }

    /// 2147483641 < i32::MAX, so this must NOT be rejected as overflow.
    #[test]
    fn largest_result_that_still_fits() {
        assert_eq!(Solution::reverse(1463847412), 2147483641);
    }

    #[test]
    fn smallest_result_that_still_fits() {
        assert_eq!(Solution::reverse(-1463847412), -2147483641);
    }

    /// Reversed this is 2147483651, one past i32::MAX.
    #[test]
    fn positive_overflow_returns_zero() {
        assert_eq!(Solution::reverse(1563847412), 0);
    }

    #[test]
    fn negative_overflow_returns_zero() {
        assert_eq!(Solution::reverse(-1563847412), 0);
    }

    #[test]
    fn i32_max_overflows() {
        assert_eq!(Solution::reverse(i32::MAX), 0);
    }

    /// i32::MIN has no positive counterpart, so `-x` panics in debug builds.
    #[test]
    fn i32_min_overflows() {
        assert_eq!(Solution::reverse(i32::MIN), 0);
    }
}
