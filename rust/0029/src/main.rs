#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (dividend, divisor) in [(10, 3), (7, -3), (i32::MIN, -1), (i32::MIN, 1)] {
        let quotient = Solution::divide(dividend, divisor);
        println!("{dividend} / {divisor} -> {quotient}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(dividend: i32, divisor: i32) -> i32 {
        // i64 avoids the single i32 overflow case (MIN / -1); Rust's `/`
        // truncates toward zero, matching the problem statement.
        let quotient = dividend as i64 / divisor as i64;
        quotient.clamp(i32::MIN as i64, i32::MAX as i64) as i32
    }

    #[test]
    fn example_1() {
        assert_eq!(Solution::divide(10, 3), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::divide(7, -3), -2);
    }

    #[test]
    fn truncates_toward_zero_in_all_sign_combinations() {
        assert_eq!(Solution::divide(7, 2), 3);
        assert_eq!(Solution::divide(-7, 2), -3);
        assert_eq!(Solution::divide(7, -2), -3);
        assert_eq!(Solution::divide(-7, -2), 3);
    }

    #[test]
    fn overflow_clamps_to_max() {
        assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
    }

    #[test]
    fn min_dividend_edge_cases() {
        assert_eq!(Solution::divide(i32::MIN, 1), i32::MIN);
        assert_eq!(Solution::divide(i32::MIN, 2), -1_073_741_824);
        assert_eq!(Solution::divide(i32::MIN, i32::MIN), 1);
        assert_eq!(Solution::divide(i32::MIN, i32::MAX), -1);
    }

    #[test]
    fn extreme_divisors() {
        assert_eq!(Solution::divide(i32::MAX, 1), i32::MAX);
        assert_eq!(Solution::divide(i32::MAX, -1), -i32::MAX);
        assert_eq!(Solution::divide(i32::MAX, i32::MAX), 1);
        assert_eq!(Solution::divide(1, i32::MIN), 0);
    }

    #[test]
    fn dividend_smaller_than_divisor() {
        assert_eq!(Solution::divide(3, 10), 0);
        assert_eq!(Solution::divide(-3, 10), 0);
        assert_eq!(Solution::divide(0, 7), 0);
    }

    #[test]
    fn matches_reference_for_all_small_inputs() {
        for dividend in -50..=50 {
            for divisor in -50..=50 {
                if divisor == 0 {
                    continue;
                }
                assert_eq!(
                    Solution::divide(dividend, divisor),
                    reference(dividend, divisor),
                    "dividend: {dividend}, divisor: {divisor}"
                );
            }
        }
    }

    #[test]
    fn matches_reference_near_the_boundaries() {
        let interesting: Vec<i32> = (-3..=3)
            .flat_map(|d| [i32::MIN.saturating_add(d.max(0)), i32::MAX - d.max(0), d])
            .collect();
        for &dividend in &interesting {
            for &divisor in &interesting {
                if divisor == 0 {
                    continue;
                }
                assert_eq!(
                    Solution::divide(dividend, divisor),
                    reference(dividend, divisor),
                    "dividend: {dividend}, divisor: {divisor}"
                );
            }
        }
    }
}
