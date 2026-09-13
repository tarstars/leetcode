use super::*;

use std::collections::HashSet;

/// Normalises for comparison: the statement allows any order, and a subset is a
/// set, so each one is sorted and the whole collected into a set. Comparing the
/// raw nesting would test an ordering the problem does not specify.
fn normalised(subsets: Vec<Vec<i32>>) -> HashSet<Vec<i32>> {
    subsets
        .into_iter()
        .map(|mut s| {
            s.sort_unstable();
            s
        })
        .collect()
}

/// Builds the power set by doubling: start from the empty subset and, for each
/// element in turn, add a copy of everything so far with that element joined
/// on. No bit masks and no recursion, so it shares nothing with the usual
/// solutions.
fn reference(nums: &[i32]) -> HashSet<Vec<i32>> {
    let mut subsets: Vec<Vec<i32>> = vec![Vec::new()];

    for &value in nums {
        let grown: Vec<Vec<i32>> = subsets
            .iter()
            .map(|s| {
                let mut next = s.clone();
                next.push(value);
                next
            })
            .collect();
        subsets.extend(grown);
    }

    normalised(subsets)
}

/// The whole specification, and it needs no enumeration to be complete: there
/// are exactly 2^n subsets of an n-element set, so producing 2^n *distinct*
/// *valid* subsets is only possible by producing all of them.
fn check(nums: &[i32]) {
    let raw = Solution::subsets(nums.to_vec());
    let expected_count = 1usize << nums.len();
    let available: HashSet<i32> = nums.iter().copied().collect();

    assert_eq!(
        raw.len(),
        expected_count,
        "{nums:?} should give {expected_count} subsets"
    );

    for subset in &raw {
        let distinct: HashSet<i32> = subset.iter().copied().collect();
        assert_eq!(
            distinct.len(),
            subset.len(),
            "{nums:?}: {subset:?} repeats a value"
        );
        assert!(
            distinct.is_subset(&available),
            "{nums:?}: {subset:?} holds a value that is not in the input"
        );
    }

    let got = normalised(raw);
    assert_eq!(
        got.len(),
        expected_count,
        "{nums:?} produced duplicate subsets"
    );

    // Belt and braces: also compare against the doubling reference.
    assert_eq!(got, reference(nums), "{nums:?}");
}

#[test]
fn example_1() {
    let got = normalised(Solution::subsets(vec![1, 2, 3]));
    let want = normalised(vec![
        vec![],
        vec![1],
        vec![2],
        vec![3],
        vec![1, 2],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3],
    ]);
    assert_eq!(got, want);
}

#[test]
fn example_2() {
    let got = normalised(Solution::subsets(vec![0]));
    assert_eq!(got, normalised(vec![vec![], vec![0]]));
}

/// The empty subset is always part of the answer, and so is the whole input.
#[test]
fn the_empty_and_the_full_subset_are_both_present() {
    for nums in [vec![1], vec![1, 2], vec![4, -2, 7], vec![1, 2, 3, 4, 5]] {
        let got = normalised(Solution::subsets(nums.clone()));

        assert!(got.contains(&Vec::new()), "{nums:?} lost the empty subset");

        let mut whole = nums.clone();
        whole.sort_unstable();
        assert!(got.contains(&whole), "{nums:?} lost the full subset");
    }
}

/// A single element gives exactly the empty subset and itself.
#[test]
fn a_single_element() {
    for value in [-10, -1, 0, 1, 10] {
        let got = normalised(Solution::subsets(vec![value]));
        assert_eq!(got, normalised(vec![vec![], vec![value]]), "{value}");
    }
}

/// Negative values and zero are ordinary elements.
#[test]
fn negative_values() {
    check(&[-10, -5, 0]);
    check(&[-1, 1]);
    check(&[-10, -9, -8, -7]);
}

/// The input is not necessarily sorted, and nothing says the output must follow
/// the input's order.
#[test]
fn unsorted_input() {
    check(&[3, 1, 2]);
    check(&[10, -10, 5, -5]);
    check(&[2, 1]);
}

/// Every length the constraints allow, against the doubling reference and the
/// counting argument.
#[test]
fn every_allowed_length() {
    for len in 1..=10 {
        let nums: Vec<i32> = (1..=len as i32).collect();
        check(&nums);
    }
}

/// Each element must appear in exactly half of the subsets — 2^(n-1) of them —
/// since the others are free to be present or absent independently.
#[test]
fn each_element_appears_in_half_the_subsets() {
    for len in 1..=10usize {
        let nums: Vec<i32> = (1..=len as i32).collect();
        let subsets = Solution::subsets(nums.clone());
        let expected = 1usize << (len - 1);

        for &value in &nums {
            let count = subsets.iter().filter(|s| s.contains(&value)).count();
            assert_eq!(count, expected, "{value} in a {len}-element input");
        }
    }
}

/// The sizes must follow the binomial distribution: C(n, k) subsets of size k.
#[test]
fn the_subset_sizes_follow_the_binomials() {
    for len in 1..=10usize {
        let nums: Vec<i32> = (1..=len as i32).collect();
        let subsets = Solution::subsets(nums);

        for k in 0..=len {
            let mut binomial = 1usize;
            for i in 1..=k {
                binomial = binomial * (len - k + i) / i;
            }

            let count = subsets.iter().filter(|s| s.len() == k).count();
            assert_eq!(count, binomial, "size {k} in a {len}-element input");
        }
    }
}

/// 10 elements is the constraint's maximum: 1024 subsets.
#[test]
fn largest_allowed_input() {
    let nums: Vec<i32> = vec![-10, -8, -6, -4, -2, 0, 2, 4, 6, 8];
    check(&nums);
    assert_eq!(Solution::subsets(nums).len(), 1024);
}
