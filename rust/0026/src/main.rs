#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for values in [vec![1, 1, 2], vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], vec![7]] {
        let mut nums = values.clone();
        let k = Solution::remove_duplicates(&mut nums) as usize;
        println!("{values:?} -> k = {k}, nums = {:?}", &nums[..k]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dedup(values: &[i32]) -> Vec<i32> {
        let mut nums = values.to_vec();
        let k = Solution::remove_duplicates(&mut nums) as usize;
        assert!(k <= nums.len());
        nums.truncate(k);
        nums
    }

    fn reference(values: &[i32]) -> Vec<i32> {
        let mut result: Vec<i32> = values.to_vec();
        result.dedup();
        result
    }

    #[test]
    fn example_1() {
        assert_eq!(dedup(&[1, 1, 2]), vec![1, 2]);
    }

    #[test]
    fn example_2() {
        assert_eq!(dedup(&[0, 0, 1, 1, 1, 2, 2, 3, 3, 4]), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn single_element() {
        assert_eq!(dedup(&[7]), vec![7]);
    }

    #[test]
    fn no_duplicates() {
        assert_eq!(dedup(&[1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn all_equal() {
        assert_eq!(dedup(&[3, 3, 3, 3]), vec![3]);
    }

    #[test]
    fn negative_values() {
        assert_eq!(dedup(&[-100, -100, -1, 0, 0, 100]), vec![-100, -1, 0, 100]);
    }

    #[test]
    fn matches_reference_for_all_small_inputs() {
        // Every non-decreasing sequence of length 1..=6 over values 0..=3.
        fn go(prefix: &mut Vec<i32>, max_len: usize) {
            if !prefix.is_empty() {
                assert_eq!(dedup(prefix), reference(prefix), "input: {prefix:?}");
            }
            if prefix.len() == max_len {
                return;
            }
            let start = *prefix.last().unwrap_or(&0);
            for value in start..=3 {
                prefix.push(value);
                go(prefix, max_len);
                prefix.pop();
            }
        }
        go(&mut Vec::new(), 6);
    }

    #[test]
    fn longest_allowed_input() {
        let values: Vec<i32> = (0..30_000).map(|i| i / 300 - 100).collect();
        assert_eq!(dedup(&values), reference(&values));
    }
}
