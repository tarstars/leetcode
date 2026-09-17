use super::*;

/// Collapses each run to a single value. The list is sorted, so consecutive
/// deduplication is the whole specification — and `Vec::dedup` shares nothing
/// with splicing nodes out of a chain.
fn reference(values: &[i32]) -> Vec<i32> {
    let mut result = values.to_vec();
    result.dedup();
    result
}

fn deduplicated(values: &[i32]) -> Vec<i32> {
    to_values(&Solution::delete_duplicates(from_values(values)))
}

fn check(values: &[i32]) {
    assert_eq!(deduplicated(values), reference(values), "for {values:?}");
}

/// Deterministic list generation, so any failure reproduces.
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

    /// A sorted list built by repeating each value a random number of times.
    fn sorted_list(&mut self, len: usize, max_run: u64) -> Vec<i32> {
        let mut values = Vec::with_capacity(len);
        let mut current = self.below(5) - 2;

        while values.len() < len {
            let run = 1 + self.below(max_run) as usize;
            for _ in 0..run.min(len - values.len()) {
                values.push(current);
            }
            current += 1 + self.below(2);
        }

        values
    }
}

#[test]
fn example_1() {
    assert_eq!(deduplicated(&[1, 1, 2]), vec![1, 2]);
}

#[test]
fn example_2() {
    assert_eq!(deduplicated(&[1, 1, 2, 3, 3]), vec![1, 2, 3]);
}

/// Zero nodes is a legal input.
#[test]
fn an_empty_list() {
    assert_eq!(deduplicated(&[]), Vec::<i32>::new());
}

#[test]
fn a_single_node() {
    assert_eq!(deduplicated(&[7]), vec![7]);
    assert_eq!(deduplicated(&[-100]), vec![-100]);
}

/// Nothing repeats, so nothing is removed.
#[test]
fn a_list_without_duplicates() {
    check(&[1, 2, 3, 4, 5]);
    check(&[-100, 0, 100]);
    check(&[1, 2]);
}

/// One value throughout collapses to a single node — never to an empty list.
/// This is the difference from problem 82, where it would vanish entirely.
#[test]
fn a_single_value_throughout() {
    assert_eq!(deduplicated(&[1, 1]), vec![1]);
    assert_eq!(deduplicated(&[1, 1, 1]), vec![1]);
    assert_eq!(deduplicated(&vec![5; 300]), vec![5]);
}

/// A run at the head, at the tail, and at both ends.
#[test]
fn runs_at_the_boundaries() {
    check(&[1, 1, 2, 3]);
    check(&[1, 2, 3, 3]);
    check(&[1, 1, 2, 3, 3]);
    check(&[1, 1, 2, 2, 3]);
    check(&[1, 2, 2, 3]);
}

/// Runs longer than two, which catch a solution that only removes one copy per
/// pass or advances the cursor after a removal.
#[test]
fn long_runs() {
    check(&[1, 1, 1]);
    check(&[1, 1, 1, 2]);
    check(&[0, 1, 1, 1, 1, 1, 2]);
    check(&[1, 2, 2, 2, 2, 3]);
    check(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 2]);
}

/// Adjacent runs, where removing one must not let the cursor skip the next.
#[test]
fn adjacent_runs() {
    check(&[1, 1, 2, 2]);
    check(&[1, 1, 2, 2, 3, 3]);
    check(&[1, 1, 1, 2, 2, 2]);
}

/// Negative values and the extremes of the permitted range are ordinary.
#[test]
fn negative_and_extreme_values() {
    check(&[-100, -100, -99, 0, 100, 100]);
    check(&[-100, -1, 0, 1, 100]);
    check(&[-100, -100]);
}

/// Every sorted list up to length 9 over a four-value alphabet. A
/// non-decreasing sequence of length L over m values is one of C(L+m-1, m-1),
/// so this is 715 lists — every arrangement of runs that short lists admit.
#[test]
fn every_short_sorted_list() {
    fn sorted_lists(len: usize, max: i32) -> Vec<Vec<i32>> {
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

    let mut checked = 0;
    for len in 0..=9 {
        for values in sorted_lists(len, 4) {
            check(&values);
            checked += 1;
        }
    }
    assert_eq!(checked, 715, "the sweep changed size");
}

/// Generated lists with runs of varying length.
#[test]
fn matches_reference_on_generated_lists() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for len in [1usize, 2, 5, 17, 64, 200, 300] {
        for max_run in [1u64, 2, 4, 10] {
            for _ in 0..10 {
                check(&rng.sorted_list(len, max_run));
            }
        }
    }
}

/// The result is always strictly increasing, and holds each input value once.
#[test]
fn the_result_is_strictly_increasing() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);

    for _ in 0..200 {
        let values = rng.sorted_list(60, 4);
        let got = deduplicated(&values);

        assert!(
            got.windows(2).all(|w| w[0] < w[1]),
            "{values:?} -> {got:?} is not strictly increasing"
        );
        for &value in &values {
            assert!(
                got.contains(&value),
                "{values:?} -> {got:?} lost the value {value}"
            );
        }
    }
}

/// 300 nodes is the constraint's maximum.
#[test]
fn largest_allowed_list() {
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);
    check(&rng.sorted_list(300, 3));

    // All distinct: nothing is removed.
    let distinct: Vec<i32> = (-100..=100).collect();
    assert_eq!(deduplicated(&distinct), distinct);

    // Every value exactly twice: each collapses to one.
    let doubled: Vec<i32> = (-100..=49).flat_map(|v| [v, v]).collect();
    assert_eq!(doubled.len(), 300);
    assert_eq!(deduplicated(&doubled), (-100..=49).collect::<Vec<i32>>());
}
