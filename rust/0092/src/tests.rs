use super::*;

/// Reverse a slice of a `Vec`. The standard library does the work, and it
/// shares nothing with relinking nodes.
fn reference(values: &[i32], left: i32, right: i32) -> Vec<i32> {
    let mut out = values.to_vec();
    out[left as usize - 1..right as usize].reverse();
    out
}

fn reversed(values: &[i32], left: i32, right: i32) -> Vec<i32> {
    to_values(&Solution::reverse_between(
        from_values(values),
        left,
        right,
    ))
}

fn check(values: &[i32], left: i32, right: i32) {
    assert_eq!(
        reversed(values, left, right),
        reference(values, left, right),
        "for {values:?}, left = {left}, right = {right}"
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
        (0..len)
            .map(|_| self.below(spread) - spread as i32 / 2)
            .collect()
    }
}

#[test]
fn example_1() {
    assert_eq!(reversed(&[1, 2, 3, 4, 5], 2, 4), vec![1, 4, 3, 2, 5]);
}

#[test]
fn example_2() {
    assert_eq!(reversed(&[5], 1, 1), vec![5]);
}

/// A one-node segment reverses to itself, wherever it sits.
#[test]
fn a_single_node_segment_changes_nothing() {
    for position in 1..=5 {
        assert_eq!(
            reversed(&[1, 2, 3, 4, 5], position, position),
            vec![1, 2, 3, 4, 5],
            "at position {position}"
        );
    }
}

/// `left == 1` moves the head itself — the case that needs no dummy node if
/// the cursor is a link slot.
#[test]
fn reversing_a_prefix_moves_the_head() {
    assert_eq!(reversed(&[1, 2, 3, 4, 5], 1, 2), vec![2, 1, 3, 4, 5]);
    assert_eq!(reversed(&[1, 2, 3, 4, 5], 1, 3), vec![3, 2, 1, 4, 5]);
    assert_eq!(reversed(&[1, 2, 3, 4, 5], 1, 5), vec![5, 4, 3, 2, 1]);
    assert_eq!(reversed(&[1, 2], 1, 2), vec![2, 1]);
}

/// `right == n` reverses to the end, so the segment has no tail to reattach.
#[test]
fn reversing_a_suffix() {
    assert_eq!(reversed(&[1, 2, 3, 4, 5], 4, 5), vec![1, 2, 3, 5, 4]);
    assert_eq!(reversed(&[1, 2, 3, 4, 5], 3, 5), vec![1, 2, 5, 4, 3]);
    assert_eq!(reversed(&[1, 2], 2, 2), vec![1, 2]);
}

/// The whole list, which is both a prefix and a suffix at once.
#[test]
fn reversing_everything() {
    for n in 1..=10usize {
        let values: Vec<i32> = (0..n as i32).collect();
        let mut expected = values.clone();
        expected.reverse();
        assert_eq!(reversed(&values, 1, n as i32), expected, "for n = {n}");
    }
}

/// A segment in the middle, with untouched nodes on both sides.
#[test]
fn reversing_the_middle() {
    check(&[1, 2, 3, 4, 5, 6, 7], 3, 5);
    check(&[1, 2, 3, 4, 5, 6, 7], 2, 6);
    check(&[1, 2, 3, 4, 5, 6, 7], 4, 4);
    check(&[1, 2, 3, 4], 2, 3);
}

/// Duplicate values must not confuse anything — the positions decide, not the
/// contents.
#[test]
fn duplicate_values() {
    check(&[7, 7, 7, 7, 7], 2, 4);
    check(&[1, 1, 2, 2, 1, 1], 2, 5);
    check(&[0, 0, 0], 1, 3);
}

/// Negative values and the ends of the permitted range.
#[test]
fn negative_and_extreme_values() {
    check(&[-500, 500, -500, 500], 2, 3);
    check(&[-500, -499, -498], 1, 3);
    check(&[500; 4], 1, 4);
}

/// Every list of length up to 12 against every valid `(left, right)` pair —
/// 364 combinations, which is all of them at these sizes. Values are the
/// positions themselves, so a misplaced node is obvious in the failure message.
#[test]
fn every_short_list_and_segment() {
    let mut checked = 0;
    for n in 1..=12i32 {
        let values: Vec<i32> = (1..=n).collect();
        for left in 1..=n {
            for right in left..=n {
                check(&values, left, right);
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 364, "the sweep changed size");
}

/// Random lists and segments.
#[test]
fn matches_reference_on_random_lists() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for len in [1usize, 2, 3, 8, 40, 200, 500] {
        for _ in 0..20 {
            let values = rng.list(len, 1_001);
            let left = 1 + rng.below(len as u64);
            let right = left + rng.below(len as u64 - left as u64 + 1);
            check(&values, left, right);
        }
    }
}

/// The node count never changes, and the untouched prefix and suffix are
/// exactly as they were.
#[test]
fn everything_outside_the_segment_is_untouched() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);

    for _ in 0..200 {
        let values = rng.list(60, 1_001);
        let left = 1 + rng.below(60);
        let right = left + rng.below(60 - left as u64 + 1);
        let got = reversed(&values, left, right);

        assert_eq!(got.len(), values.len(), "{values:?} changed length");
        let (l, r) = (left as usize - 1, right as usize);
        assert_eq!(got[..l], values[..l], "the prefix moved");
        assert_eq!(got[r..], values[r..], "the suffix moved");
    }
}

/// 500 nodes is the constraint's maximum, in the three boundary segments.
#[test]
fn largest_allowed_list() {
    let values: Vec<i32> = (0..500).map(|i| i - 250).collect();

    check(&values, 1, 500);
    check(&values, 1, 1);
    check(&values, 500, 500);
    check(&values, 2, 499);
    check(&values, 250, 251);
}
