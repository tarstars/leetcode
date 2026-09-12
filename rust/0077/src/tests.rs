use super::*;

use std::collections::HashSet;

/// Enumerates the combinations by running over every subset of `1..=n` as a bit
/// mask and keeping those with exactly `k` bits set. No recursion and no
/// backtracking, so it shares nothing with the usual solution.
fn reference(n: i32, k: i32) -> HashSet<Vec<i32>> {
    let n = n as u32;
    let k = k as u32;

    (0u32..1 << n)
        .filter(|mask| mask.count_ones() == k)
        .map(|mask| {
            (0..n)
                .filter(|bit| mask >> bit & 1 == 1)
                .map(|bit| bit as i32 + 1)
                .collect()
        })
        .collect()
}

/// C(n, k), computed multiplicatively so it stays exact.
fn binomial(n: u64, k: u64) -> u64 {
    let k = k.min(n - k);
    let mut result = 1u64;
    for i in 1..=k {
        result = result * (n - k + i) / i;
    }
    result
}

/// Normalises the answer for comparison: the statement allows any order, and a
/// combination is a set, so each one is sorted and the whole is collected into
/// a set. Comparing the raw `Vec<Vec<i32>>` would be checking an ordering the
/// problem does not specify.
fn normalised(combinations: Vec<Vec<i32>>) -> HashSet<Vec<i32>> {
    combinations
        .into_iter()
        .map(|mut c| {
            c.sort_unstable();
            c
        })
        .collect()
}

/// Every rule at once: the right number of combinations, each of the right
/// size, drawn from the right range, without repeats inside a combination or
/// across them — and exactly the set the reference produces.
fn check(n: i32, k: i32) {
    let raw = Solution::combine(n, k);
    let expected_count = binomial(n as u64, k as u64) as usize;

    assert_eq!(
        raw.len(),
        expected_count,
        "n = {n}, k = {k} should give {expected_count} combinations"
    );

    for combination in &raw {
        assert_eq!(
            combination.len(),
            k as usize,
            "n = {n}, k = {k}: {combination:?} is the wrong size"
        );
        assert!(
            combination.iter().all(|&v| (1..=n).contains(&v)),
            "n = {n}, k = {k}: {combination:?} leaves the range"
        );

        let distinct: HashSet<i32> = combination.iter().copied().collect();
        assert_eq!(
            distinct.len(),
            k as usize,
            "n = {n}, k = {k}: {combination:?} repeats a value"
        );
    }

    let got = normalised(raw);
    assert_eq!(
        got.len(),
        expected_count,
        "n = {n}, k = {k} produced duplicate combinations"
    );
    assert_eq!(got, reference(n, k), "n = {n}, k = {k}");
}

#[test]
fn example_1() {
    let got = normalised(Solution::combine(4, 2));
    let want = normalised(vec![
        vec![1, 2],
        vec![1, 3],
        vec![1, 4],
        vec![2, 3],
        vec![2, 4],
        vec![3, 4],
    ]);
    assert_eq!(got, want);
}

#[test]
fn example_2() {
    assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
}

/// Choosing one gives every single value on its own.
#[test]
fn choosing_one() {
    for n in 1..=12 {
        let got = normalised(Solution::combine(n, 1));
        let want: HashSet<Vec<i32>> = (1..=n).map(|v| vec![v]).collect();
        assert_eq!(got, want, "n = {n}");
    }
}

/// Choosing all of them gives exactly one combination, the whole range.
#[test]
fn choosing_everything() {
    for n in 1..=12 {
        let got = Solution::combine(n, n);
        assert_eq!(got.len(), 1, "n = {n}");

        let mut only = got[0].clone();
        only.sort_unstable();
        assert_eq!(only, (1..=n).collect::<Vec<i32>>(), "n = {n}");
    }
}

/// Choosing all but one leaves n combinations, each omitting a single value.
#[test]
fn choosing_all_but_one() {
    for n in 2..=12 {
        check(n, n - 1);
    }
}

/// Every legal (n, k) pair up to n = 12, against the bit-mask reference.
#[test]
fn matches_reference_for_every_small_pair() {
    for n in 1..=12 {
        for k in 1..=n {
            check(n, k);
        }
    }
}

/// The counts must follow Pascal's rule: C(n, k) = C(n-1, k-1) + C(n-1, k).
/// k starts at 2 so that both terms on the right stay inside the stated
/// constraint of 1 <= k <= n — k = 1 would ask for C(n-1, 0), which the
/// signature cannot express.
#[test]
fn the_counts_obey_pascals_rule() {
    for n in 3..=13 {
        for k in 2..n {
            let whole = Solution::combine(n, k).len();
            let with_n = Solution::combine(n - 1, k - 1).len();
            let without_n = Solution::combine(n - 1, k).len();

            assert_eq!(
                whole,
                with_n + without_n,
                "n = {n}, k = {k} breaks C(n,k) = C(n-1,k-1) + C(n-1,k)"
            );
        }
    }
}

/// n = 20 is the constraint's maximum, and k = 10 its widest point: C(20, 10)
/// is 184,756 combinations.
#[test]
fn largest_allowed_input() {
    assert_eq!(Solution::combine(20, 1).len(), 20);
    assert_eq!(Solution::combine(20, 20).len(), 1);
    assert_eq!(Solution::combine(20, 2).len(), 190);
    assert_eq!(Solution::combine(20, 19).len(), 20);

    let widest = Solution::combine(20, 10);
    assert_eq!(widest.len(), 184_756);

    // Well formed, and free of duplicates, without materialising a reference.
    for combination in &widest {
        assert_eq!(combination.len(), 10);
        assert!(combination.iter().all(|&v| (1..=20).contains(&v)));
    }
    assert_eq!(normalised(widest).len(), 184_756);
}
