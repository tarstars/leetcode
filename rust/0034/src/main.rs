#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (nums, target) in [
        (vec![5, 7, 7, 8, 8, 10], 8),
        (vec![5, 7, 7, 8, 8, 10], 6),
        (vec![], 0),
    ] {
        let answer = Solution::search_range(nums.clone(), target);
        println!("{nums:?}, target {target} -> {answer:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(nums: &[i32], target: i32) -> Vec<i32> {
        let first = nums.iter().position(|&value| value == target);
        let last = nums.iter().rposition(|&value| value == target);

        match (first, last) {
            (Some(first), Some(last)) => vec![first as i32, last as i32],
            _ => vec![-1, -1],
        }
    }

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }

    #[test]
    fn handles_single_element_arrays() {
        assert_eq!(Solution::search_range(vec![4], 4), vec![0, 0]);
        assert_eq!(Solution::search_range(vec![4], 3), vec![-1, -1]);
    }

    #[test]
    fn finds_ranges_at_array_boundaries() {
        assert_eq!(
            Solution::search_range(vec![1, 1, 1, 2, 3, 4], 1),
            vec![0, 2]
        );
        assert_eq!(
            Solution::search_range(vec![1, 2, 3, 4, 4, 4], 4),
            vec![3, 5]
        );
    }

    #[test]
    fn handles_an_array_containing_only_the_target() {
        assert_eq!(Solution::search_range(vec![7, 7, 7, 7, 7], 7), vec![0, 4]);
    }

    #[test]
    fn handles_targets_outside_the_value_range() {
        let nums = vec![-3, -1, 0, 0, 2];
        assert_eq!(Solution::search_range(nums.clone(), -4), vec![-1, -1]);
        assert_eq!(Solution::search_range(nums, 3), vec![-1, -1]);
    }

    #[test]
    fn matches_reference_for_all_small_inputs() {
        const VALUES: [i32; 5] = [-2, -1, 0, 1, 2];

        for length in 0..=8 {
            for code in 0..VALUES.len().pow(length as u32) {
                let mut encoded = code;
                let mut nums = Vec::with_capacity(length);

                for _ in 0..length {
                    nums.push(VALUES[encoded % VALUES.len()]);
                    encoded /= VALUES.len();
                }
                nums.sort_unstable();

                for target in -3..=3 {
                    assert_eq!(
                        Solution::search_range(nums.clone(), target),
                        reference(&nums, target),
                        "nums: {nums:?}, target: {target}"
                    );
                }
            }
        }
    }

    #[test]
    fn handles_the_maximum_input_length() {
        let mut nums = vec![-1; 25_000];
        nums.extend(vec![0; 50_000]);
        nums.extend(vec![1; 25_000]);

        assert_eq!(Solution::search_range(nums.clone(), -1), vec![0, 24_999]);
        assert_eq!(
            Solution::search_range(nums.clone(), 0),
            vec![25_000, 74_999]
        );
        assert_eq!(Solution::search_range(nums, 1), vec![75_000, 99_999]);
    }
}
