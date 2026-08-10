#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (nums, target) in [(vec![-1, 2, 1, -4], 1), (vec![0, 0, 0], 1)] {
        println!(
            "{nums:?} target {target} -> {}",
            Solution::three_sum_closest(nums.clone(), target)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn closest(nums: &[i32], target: i32) -> i32 {
        Solution::three_sum_closest(nums.to_vec(), target)
    }

    #[test]
    fn example_1() {
        assert_eq!(closest(&[-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(closest(&[0, 0, 0], 1), 0);
    }

    /// Only three elements: no choice at all.
    #[test]
    fn minimum_length_input() {
        assert_eq!(closest(&[0, 1, 2], 3), 3);
        assert_eq!(closest(&[-5, -5, -5], 100), -15);
    }

    /// An exact hit must be returned as-is.
    #[test]
    fn exact_match() {
        assert_eq!(closest(&[1, 2, 4, 8, 16, 32, 64, 128], 82), 82);
        assert_eq!(closest(&[4, 0, 5, -5, 3, 3, 0, -4, -5], -2), -2);
    }

    /// The target is far below every reachable sum.
    #[test]
    fn target_below_every_sum() {
        assert_eq!(closest(&[1, 1, 1, 0], -100), 2);
    }

    #[test]
    fn negative_target() {
        assert_eq!(closest(&[-3, -2, -5, 3, -4], -1), -2);
    }

    #[test]
    fn duplicates_do_not_confuse_it() {
        assert_eq!(closest(&[0, 2, 1, -3], 1), 0);
    }

    /// Extreme values at both ends of the allowed range.
    #[test]
    fn extreme_values() {
        assert_eq!(closest(&[-1000, -1000, -1000, 1000, 1000, 1000], 500), 1000);
    }

    /// 500 elements — the constraint's upper bound. The target is unreachable,
    /// so the answer is the largest sum available.
    #[test]
    fn large_input_target_above_range() {
        let nums: Vec<i32> = (0..500).map(|i: i64| ((i * i * 7) % 2001 - 1000) as i32).collect();
        assert_eq!(Solution::three_sum_closest(nums, 9999), 2944);
    }

    /// Same array, target below every sum.
    #[test]
    fn large_input_target_below_range() {
        let nums: Vec<i32> = (0..500).map(|i: i64| ((i * i * 7) % 2001 - 1000) as i32).collect();
        assert_eq!(Solution::three_sum_closest(nums, -9999), -2986);
    }

    /// Same array with a target that is exactly reachable.
    #[test]
    fn large_input_exact_hit() {
        let nums: Vec<i32> = (0..500).map(|i: i64| ((i * i * 7) % 2001 - 1000) as i32).collect();
        assert_eq!(Solution::three_sum_closest(nums, 137), 137);
    }
}
