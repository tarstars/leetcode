use super::*;

/// A separate sorted concatenation, rather than another in-place merge, keeps
/// the oracle independent from the cursor movement the solution must test.
fn reference(nums1: &[i32], m: i32, nums2: &[i32], n: i32) -> Vec<i32> {
    let mut expected = nums1[..m as usize].to_vec();
    expected.extend_from_slice(&nums2[..n as usize]);
    expected.sort_unstable();
    expected
}

fn merged(nums1: &mut Vec<i32>, m: i32, nums2: &[i32], n: i32) -> Vec<i32> {
    Solution::merge(nums1, m, nums2.to_vec(), n);
    nums1.clone()
}

fn check(nums1: Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) {
    let expected = reference(&nums1, m, &nums2, n);
    let mut actual_input = nums1;
    let actual = merged(&mut actual_input, m, &nums2, n);
    assert_eq!(actual, expected);
}

#[test]
fn example_1() {
    check(vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3);
}

#[test]
fn example_2() {
    check(vec![1], 1, vec![], 0);
}

#[test]
fn example_3() {
    check(vec![0], 0, vec![1], 1);
}

#[test]
fn both_inputs_can_be_empty() {
    check(vec![], 0, vec![], 0);
}

#[test]
fn one_side_can_be_empty_with_capacity() {
    check(vec![1, 2, 3], 3, vec![], 0);
    check(vec![0, 0, 0], 0, vec![-3, -1, 8], 3);
}

#[test]
fn duplicates_are_preserved() {
    check(vec![1, 1, 2, 2, 0, 0, 0], 4, vec![1, 2, 2], 3);
    check(vec![0, 0, 0, 0], 0, vec![5, 5, 5, 5], 4);
}

#[test]
fn negative_values_and_extremes() {
    check(
        vec![i32::MIN, -10, 0, 0, 0, 0],
        3,
        vec![i32::MIN, -10, i32::MAX],
        3,
    );
}

#[test]
fn already_ordered_inputs() {
    check(vec![-5, -2, 0, 0, 0], 2, vec![1, 4, 9], 3);
}

#[test]
fn nums2_must_be_inserted_before_every_nums1_value() {
    check(vec![10, 20, 30, 0, 0, 0], 3, vec![1, 2, 3], 3);
}

#[test]
fn nums1_must_be_inserted_before_every_nums2_value() {
    check(vec![-3, -2, -1, 0, 0, 0], 3, vec![4, 5, 6], 3);
}

#[test]
fn zero_placeholders_are_not_data() {
    check(vec![0, 0, 0, 0], 2, vec![1, 3], 2);
    check(vec![-2, 0, 0, 0], 1, vec![0, 0, 4], 3);
}

#[test]
fn repeated_interleaving() {
    check(
        vec![-8, -4, 0, 3, 7, 0, 0, 0, 0, 0],
        5,
        vec![-7, -4, 1, 3, 8],
        5,
    );
}

/// Deterministic pseudo-random values exercise all cursor orderings without
/// making a failing case irreproducible.
struct Rng(u64);

impl Rng {
    fn next_value(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, bound: u64) -> i32 {
        (self.next_value() % bound) as i32
    }
}

#[test]
fn matches_reference_on_random_sorted_inputs() {
    let mut rng = Rng(0xD00D_F00D_1234_5678);

    for _ in 0..500 {
        let m = rng.below(30) as usize;
        let n = rng.below(30) as usize;
        let mut left: Vec<i32> = (0..m).map(|_| rng.below(101) - 50).collect();
        let mut right: Vec<i32> = (0..n).map(|_| rng.below(101) - 50).collect();
        left.sort_unstable();
        right.sort_unstable();

        let mut nums1 = left.clone();
        nums1.resize(m + n, 0);
        check(nums1, m as i32, right, n as i32);
    }
}

#[test]
fn handles_the_constraint_scale() {
    let left: Vec<i32> = (0..200).map(|value| value * 2 - 200).collect();
    let right: Vec<i32> = (0..200).map(|value| value * 2 - 199).collect();
    let mut nums1 = left.clone();
    nums1.resize(400, 0);

    let expected = reference(&nums1, 200, &right, 200);
    assert_eq!(merged(&mut nums1, 200, &right, 200), expected);
}
