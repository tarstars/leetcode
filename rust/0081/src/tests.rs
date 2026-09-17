use super::*;

#[test]
fn example_1() {
    assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
}

#[test]
fn example_2() {
    assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
}

#[test]
fn one_element() {
    assert!(Solution::search(vec![1], 1));
    assert!(!Solution::search(vec![1], 0));
}

#[test]
fn an_unrotated_array() {
    let nums = vec![-4, -2, -2, 0, 3, 3, 8];
    for target in [-4, -2, 0, 3, 8] {
        assert!(Solution::search(nums.clone(), target), "target = {target}");
    }
    for target in [-5, -3, 1, 4, 9] {
        assert!(!Solution::search(nums.clone(), target), "target = {target}");
    }
}

#[test]
fn all_elements_are_equal() {
    assert!(Solution::search(vec![7; 20], 7));
    assert!(!Solution::search(vec![7; 20], 6));
}

#[test]
fn duplicates_can_hide_which_half_is_sorted() {
    for nums in [
        vec![1, 0, 1, 1, 1],
        vec![1, 1, 1, 0, 1],
        vec![1, 1, 3, 1],
        vec![1, 3, 1, 1, 1],
    ] {
        let target = if nums.contains(&0) { 0 } else { 3 };
        assert!(Solution::search(nums, target));
    }
}

#[test]
fn every_pivot_of_several_sorted_arrays() {
    let cases = [
        vec![1],
        vec![1, 1],
        vec![0, 0, 1],
        vec![-2, -2, -1, 0, 0, 3],
        vec![0, 1, 1, 1, 2, 2],
        vec![-10_000, -10_000, 0, 10_000, 10_000],
    ];

    for sorted in cases {
        for pivot in 0..sorted.len() {
            let mut rotated = sorted.clone();
            rotated.rotate_left(pivot);

            for target in [-10_000, -3, -2, -1, 0, 1, 2, 3, 4, 10_000] {
                assert_eq!(
                    Solution::search(rotated.clone(), target),
                    sorted.contains(&target),
                    "sorted = {sorted:?}, pivot = {pivot}, target = {target}"
                );
            }
        }
    }
}

#[test]
fn target_can_be_at_either_end() {
    assert!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 4));
    assert!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 2));
}

#[test]
fn largest_allowed_array() {
    let mut nums = vec![1; 5000];
    nums[2500] = 0;

    assert!(Solution::search(nums.clone(), 0));
    assert!(Solution::search(nums.clone(), 1));
    assert!(!Solution::search(nums, 2));
}

/// Every rotation of every non-decreasing array up to length 8 over a
/// four-value alphabet, probed with every target in and just outside that
/// range — 19,008 cases, checked against a linear scan.
///
/// Hand-written cases are weak for this problem: the interesting failures live
/// in specific arrangements of duplicates, and there is no intuition for which
/// ones. Enumerating the whole space is cheap enough to remove the guesswork.
#[test]
fn matches_a_linear_scan_exhaustively() {
    fn non_decreasing(len: usize, max: i32) -> Vec<Vec<i32>> {
        let mut out = vec![Vec::new()];
        for _ in 0..len {
            let mut next = Vec::new();
            for prefix in &out {
                let start = *prefix.last().unwrap_or(&1);
                for value in start..=max {
                    let mut grown = prefix.clone();
                    grown.push(value);
                    next.push(grown);
                }
            }
            out = next;
        }
        out
    }

    let mut checked = 0u32;

    for len in 1..=8usize {
        for base in non_decreasing(len, 4) {
            for pivot in 0..len {
                let rotated: Vec<i32> = base[pivot..]
                    .iter()
                    .chain(base[..pivot].iter())
                    .copied()
                    .collect();

                for target in 0..=5 {
                    assert_eq!(
                        Solution::search(rotated.clone(), target),
                        rotated.contains(&target),
                        "{rotated:?} target {target}"
                    );
                    checked += 1;
                }
            }
        }
    }

    assert_eq!(checked, 19_008, "the sweep changed size");
}
