#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (nums, target) in [
        (vec![4, 5, 6, 7, 0, 1, 2], 0),
        (vec![4, 5, 6, 7, 0, 1, 2], 3),
        (vec![1], 0),
    ] {
        let answer = Solution::search(nums.clone(), target);
        println!("{nums:?}, target {target} -> {answer}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(nums: &[i32], target: i32) -> i32 {
        nums.iter()
            .position(|&value| value == target)
            .map_or(-1, |index| index as i32)
    }

    fn rotate(values: &[i32], pivot: usize) -> Vec<i32> {
        values[pivot..]
            .iter()
            .chain(&values[..pivot])
            .copied()
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }

    #[test]
    fn handles_single_element_arrays() {
        assert_eq!(Solution::search(vec![7], 7), 0);
        assert_eq!(Solution::search(vec![7], -7), -1);
    }

    #[test]
    fn handles_unrotated_arrays() {
        let nums = vec![-8, -3, 0, 2, 9];
        assert_eq!(Solution::search(nums.clone(), -8), 0);
        assert_eq!(Solution::search(nums.clone(), 9), 4);
        assert_eq!(Solution::search(nums, 1), -1);
    }

    #[test]
    fn finds_every_value_after_every_rotation() {
        let sorted = [-10, -3, 0, 4, 9, 15];

        for pivot in 0..sorted.len() {
            let nums = rotate(&sorted, pivot);
            for &target in &sorted {
                assert_eq!(
                    Solution::search(nums.clone(), target),
                    reference(&nums, target),
                    "nums: {nums:?}, target: {target}"
                );
            }
        }
    }

    #[test]
    fn handles_targets_outside_the_value_range() {
        let nums = vec![3, 5, 8, -4, -1, 0];
        assert_eq!(Solution::search(nums.clone(), -10), -1);
        assert_eq!(Solution::search(nums, 10), -1);
    }

    #[test]
    fn matches_reference_for_all_small_inputs() {
        const VALUES: [i32; 7] = [-3, -2, -1, 0, 1, 2, 3];

        for mask in 1_usize..1 << VALUES.len() {
            let sorted: Vec<i32> = VALUES
                .iter()
                .enumerate()
                .filter_map(|(index, &value)| (mask & (1 << index) != 0).then_some(value))
                .collect();

            for pivot in 0..sorted.len() {
                let nums = rotate(&sorted, pivot);
                for target in -4..=4 {
                    assert_eq!(
                        Solution::search(nums.clone(), target),
                        reference(&nums, target),
                        "nums: {nums:?}, target: {target}"
                    );
                }
            }
        }
    }

    #[test]
    fn handles_the_maximum_input_length() {
        let sorted: Vec<i32> = (-2_500..2_500).collect();
        let nums = rotate(&sorted, 3_217);

        for target in [-2_500, -1, 0, 2_499, 10_000] {
            assert_eq!(
                Solution::search(nums.clone(), target),
                reference(&nums, target),
                "target: {target}"
            );
        }
    }
}
