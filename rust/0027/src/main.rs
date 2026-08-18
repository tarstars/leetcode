#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (values, val) in [
        (vec![3, 2, 2, 3], 3),
        (vec![0, 1, 2, 2, 3, 0, 4, 2], 2),
        (vec![], 1),
    ] {
        let mut nums = values.clone();
        let k = Solution::remove_element(&mut nums, val) as usize;
        println!("{values:?}, val = {val} -> k = {k}, nums = {:?}", &nums[..k]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The order of the kept elements does not matter, so compare sorted,
    // as the LeetCode judge does.
    fn remove(values: &[i32], val: i32) -> Vec<i32> {
        let mut nums = values.to_vec();
        let k = Solution::remove_element(&mut nums, val) as usize;
        assert!(k <= nums.len());
        nums.truncate(k);
        nums.sort();
        nums
    }

    fn reference(values: &[i32], val: i32) -> Vec<i32> {
        let mut result: Vec<i32> = values.iter().copied().filter(|&v| v != val).collect();
        result.sort();
        result
    }

    #[test]
    fn example_1() {
        assert_eq!(remove(&[3, 2, 2, 3], 3), vec![2, 2]);
    }

    #[test]
    fn example_2() {
        assert_eq!(remove(&[0, 1, 2, 2, 3, 0, 4, 2], 2), vec![0, 0, 1, 3, 4]);
    }

    #[test]
    fn empty_input() {
        assert_eq!(remove(&[], 1), vec![]);
    }

    #[test]
    fn removes_everything() {
        assert_eq!(remove(&[5, 5, 5], 5), vec![]);
    }

    #[test]
    fn removes_nothing() {
        assert_eq!(remove(&[1, 2, 3], 7), vec![1, 2, 3]);
    }

    #[test]
    fn single_kept() {
        assert_eq!(remove(&[4], 9), vec![4]);
    }

    #[test]
    fn single_removed() {
        assert_eq!(remove(&[9], 9), vec![]);
    }

    #[test]
    fn matches_reference_for_all_small_inputs() {
        // Every sequence of length 0..=5 over values 0..=2, removing each value 0..=2.
        fn go(prefix: &mut Vec<i32>, max_len: usize) {
            for val in 0..=2 {
                assert_eq!(
                    remove(prefix, val),
                    reference(prefix, val),
                    "input: {prefix:?}, val: {val}"
                );
            }
            if prefix.len() == max_len {
                return;
            }
            for value in 0..=2 {
                prefix.push(value);
                go(prefix, max_len);
                prefix.pop();
            }
        }
        go(&mut Vec::new(), 5);
    }

    #[test]
    fn longest_allowed_input() {
        let values: Vec<i32> = (0..100).map(|i| i % 51).collect();
        assert_eq!(remove(&values, 25), reference(&values, 25));
    }
}
