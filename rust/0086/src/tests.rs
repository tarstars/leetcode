use super::*;

/// A stable partition done on a plain `Vec`: take everything below `x` in
/// order, then everything at or above it in order. Shares nothing with node
/// surgery.
fn reference(values: &[i32], x: i32) -> Vec<i32> {
    let mut out: Vec<i32> = values.iter().copied().filter(|&v| v < x).collect();
    out.extend(values.iter().copied().filter(|&v| v >= x));
    out
}

fn partitioned(values: &[i32], x: i32) -> Vec<i32> {
    to_values(&Solution::partition(from_values(values), x))
}

fn check(values: &[i32], x: i32) {
    assert_eq!(
        partitioned(values, x),
        reference(values, x),
        "for {values:?}, x = {x}"
    );
}

/// Deterministic generation, so any failure reproduces exactly.
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

    fn list(&mut self, len: usize, spread: u64) -> Vec<i32> {
        (0..len).map(|_| self.below(spread) - spread as i32 / 2).collect()
    }
}

#[test]
fn example_1() {
    assert_eq!(partitioned(&[1, 4, 3, 2, 5, 2], 3), vec![1, 2, 2, 4, 3, 5]);
}

#[test]
fn example_2() {
    assert_eq!(partitioned(&[2, 1], 2), vec![1, 2]);
}

/// Zero nodes is a legal input, whatever `x` is.
#[test]
fn an_empty_list() {
    assert_eq!(partitioned(&[], 0), Vec::<i32>::new());
    assert_eq!(partitioned(&[], -200), Vec::<i32>::new());
    assert_eq!(partitioned(&[], 200), Vec::<i32>::new());
}

#[test]
fn a_single_node() {
    assert_eq!(partitioned(&[5], 10), vec![5]);
    assert_eq!(partitioned(&[5], 5), vec![5]);
    assert_eq!(partitioned(&[5], 1), vec![5]);
}

/// A value equal to `x` belongs to the right half, not the left. This is the
/// asymmetry the problem statement hides in plain sight.
#[test]
fn a_value_equal_to_x_goes_right() {
    assert_eq!(partitioned(&[3, 1], 3), vec![1, 3]);
    assert_eq!(partitioned(&[3, 3, 3], 3), vec![3, 3, 3]);
    assert_eq!(partitioned(&[3, 2, 3, 2], 3), vec![2, 2, 3, 3]);
}

/// One half comes out empty, so the join has to cope with a missing piece.
#[test]
fn one_side_is_empty() {
    assert_eq!(partitioned(&[1, 2, 3], 100), vec![1, 2, 3]);
    assert_eq!(partitioned(&[1, 2, 3], -100), vec![1, 2, 3]);
    assert_eq!(partitioned(&[5, 5, 5], 5), vec![5, 5, 5]);
    assert_eq!(partitioned(&[-100, -100], 200), vec![-100, -100]);
}

/// Already partitioned: nothing may move.
#[test]
fn an_already_partitioned_list() {
    check(&[1, 2, 7, 8], 5);
    check(&[1, 1, 1, 9, 9, 9], 5);
    check(&[0, 5], 5);
}

/// Fully reversed: every node crosses the boundary.
#[test]
fn a_reversed_list() {
    assert_eq!(partitioned(&[9, 8, 1, 2], 5), vec![1, 2, 9, 8]);
    assert_eq!(partitioned(&[9, 1, 9, 1], 5), vec![1, 1, 9, 9]);
}

/// Distinct values make relative order observable: within each half the nodes
/// must appear in exactly their input order, never reversed or shuffled.
#[test]
fn relative_order_is_preserved() {
    let values = [7, 1, 8, 2, 9, 3, 10, 4];
    assert_eq!(partitioned(&values, 5), vec![1, 2, 3, 4, 7, 8, 9, 10]);

    // A solution that prepends instead of appending yields this instead.
    assert_ne!(partitioned(&values, 5), vec![4, 3, 2, 1, 10, 9, 8, 7]);
}

/// Negative values and the extremes of the permitted ranges.
#[test]
fn negative_and_extreme_values() {
    check(&[-100, 100, -100, 100], 0);
    check(&[-100, -99, -98], -99);
    check(&[100, 99, 98], 99);
    check(&[0, -1, 1], 0);
}

/// Every list up to length 6 over four values, against every `x` that can
/// possibly split them differently — 27,305 cases.
#[test]
fn every_short_list_and_split() {
    fn all_lists(len: usize, max: i32) -> Vec<Vec<i32>> {
        let mut out = vec![Vec::new()];
        for _ in 0..len {
            let mut next = Vec::with_capacity(out.len() * (max as usize + 1));
            for prefix in &out {
                for value in 0..=max {
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
    for len in 0..=6 {
        for values in all_lists(len, 3) {
            for x in 0..=4 {
                check(&values, x);
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 27_305, "the sweep changed size");
}

/// Random lists across lengths and value spreads.
#[test]
fn matches_reference_on_random_lists() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for len in [1usize, 2, 3, 10, 50, 200] {
        for spread in [2u64, 5, 40, 200] {
            for _ in 0..10 {
                let values = rng.list(len, spread);
                let x = rng.below(spread) - spread as i32 / 2;
                check(&values, x);
            }
        }
    }
}

/// Every node must survive exactly once: same multiset in, same multiset out.
/// This catches a solution that drops a node or splices one in twice.
#[test]
fn no_node_is_lost_or_duplicated() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);

    for _ in 0..300 {
        let values = rng.list(40, 10);
        let x = rng.below(10) - 5;
        let got = partitioned(&values, x);

        let mut before = values.clone();
        let mut after = got.clone();
        before.sort_unstable();
        after.sort_unstable();
        assert_eq!(before, after, "{values:?}, x = {x} -> {got:?}");

        let split = got.iter().position(|&v| v >= x).unwrap_or(got.len());
        assert!(
            got[..split].iter().all(|&v| v < x) && got[split..].iter().all(|&v| v >= x),
            "{values:?}, x = {x} -> {got:?} is not partitioned"
        );
    }
}

/// 200 nodes is the constraint's maximum, in three arrangements.
#[test]
fn largest_allowed_list() {
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);
    check(&rng.list(200, 200), 0);

    // Strictly alternating across the boundary.
    let alternating: Vec<i32> = (0..200).map(|i| if i % 2 == 0 { 100 } else { -100 }).collect();
    check(&alternating, 0);

    // Everything on one side, then everything on the other.
    let sorted: Vec<i32> = (0..200).map(|i| i - 100).collect();
    check(&sorted, 0);
    check(&sorted, -200);
    check(&sorted, 200);
}
