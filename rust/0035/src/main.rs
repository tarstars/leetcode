#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (nums, target) in [
        (vec![1, 3, 5, 6], 5),
        (vec![1, 3, 5, 6], 2),
        (vec![1, 3, 5, 6], 7),
    ] {
        let answer = Solution::search_insert(nums.clone(), target);
        println!("{nums:?}, target {target} -> {answer}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(nums: &[i32], target: i32) -> i32 {
        nums.partition_point(|&value| value < target) as i32
    }

    #[test]
    fn example_1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn inserts_before_the_first_element() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }

    #[test]
    fn handles_single_element_arrays() {
        assert_eq!(Solution::search_insert(vec![4], 3), 0);
        assert_eq!(Solution::search_insert(vec![4], 4), 0);
        assert_eq!(Solution::search_insert(vec![4], 5), 1);
    }

    #[test]
    fn handles_negative_values() {
        let nums = vec![-10, -4, -1, 3, 8];
        assert_eq!(Solution::search_insert(nums.clone(), -4), 1);
        assert_eq!(Solution::search_insert(nums.clone(), -2), 2);
        assert_eq!(Solution::search_insert(nums, -11), 0);
    }

    #[test]
    fn matches_reference_for_all_small_inputs() {
        const VALUES: [i32; 9] = [-4, -3, -2, -1, 0, 1, 2, 3, 4];

        for mask in 1_usize..1 << VALUES.len() {
            let nums: Vec<i32> = VALUES
                .iter()
                .enumerate()
                .filter_map(|(index, &value)| (mask & (1 << index) != 0).then_some(value))
                .collect();

            for target in -5..=5 {
                assert_eq!(
                    Solution::search_insert(nums.clone(), target),
                    reference(&nums, target),
                    "nums: {nums:?}, target: {target}"
                );
            }
        }
    }

    #[test]
    fn handles_the_maximum_input_length() {
        let nums: Vec<i32> = (-5_000..5_000).collect();

        for target in [-10_000, -5_000, -1, 0, 4_999, 5_000, 10_000] {
            assert_eq!(
                Solution::search_insert(nums.clone(), target),
                reference(&nums, target),
                "target: {target}"
            );
        }
    }
}
