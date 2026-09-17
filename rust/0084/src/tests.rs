use super::*;

/// The definition, taken literally: try every span, and let the shortest bar in
/// the span set the height. O(n^2), and it shares no machinery whatsoever with
/// a stack-based solution — which is the point of an oracle.
fn reference(heights: &[i32]) -> i32 {
    let mut best = 0;
    for i in 0..heights.len() {
        let mut lowest = i32::MAX;
        for (j, &height) in heights.iter().enumerate().skip(i) {
            lowest = lowest.min(height);
            best = best.max(lowest * (j - i + 1) as i32);
        }
    }
    best
}

fn area(heights: &[i32]) -> i32 {
    Solution::largest_rectangle_area(heights.to_vec())
}

fn check(heights: &[i32]) {
    assert_eq!(area(heights), reference(heights), "for {heights:?}");
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

    fn histogram(&mut self, len: usize, max: u64) -> Vec<i32> {
        (0..len).map(|_| self.below(max + 1)).collect()
    }
}

#[test]
fn example_1() {
    assert_eq!(area(&[2, 1, 5, 6, 2, 3]), 10);
}

#[test]
fn example_2() {
    assert_eq!(area(&[2, 4]), 4);
}

#[test]
fn a_single_bar() {
    assert_eq!(area(&[5]), 5);
    assert_eq!(area(&[0]), 0);
    assert_eq!(area(&[10_000]), 10_000);
}

/// Height zero is permitted, and a histogram of nothing but zeros has no
/// rectangle of positive area.
#[test]
fn zero_heights() {
    assert_eq!(area(&[0, 0, 0, 0]), 0);
    check(&[0, 1, 0]);
    check(&[1, 0, 1]);
    check(&[0, 0, 5, 0, 0]);
}

/// A zero splits the histogram into independent halves; the answer is the best
/// of the two, never something spanning the gap.
#[test]
fn a_zero_splits_the_histogram() {
    assert_eq!(area(&[4, 4, 0, 5, 5]), 10);
    assert_eq!(area(&[5, 5, 5, 0, 4, 4]), 15);
    check(&[3, 3, 3, 0, 2, 2, 2, 2, 2]);
}

/// Strictly increasing: nothing is ever resolved while scanning left to right,
/// so a solution that forgets to drain its stack at the end returns 0 here.
#[test]
fn strictly_increasing() {
    assert_eq!(area(&[1, 2, 3, 4, 5]), 9);
    check(&[1, 2, 3]);
    check(&[1, 2, 3, 4, 5, 6, 7, 8]);
}

/// Strictly decreasing: the mirror image, resolved entirely during the scan.
#[test]
fn strictly_decreasing() {
    assert_eq!(area(&[5, 4, 3, 2, 1]), 9);
    check(&[3, 2, 1]);
    check(&[8, 7, 6, 5, 4, 3, 2, 1]);
}

/// Equal neighbours are where `<` versus `<=` in the popping rule shows up.
#[test]
fn runs_of_equal_height() {
    assert_eq!(area(&[3, 3, 3, 3]), 12);
    check(&[2, 2, 1, 1, 2, 2]);
    check(&[4, 4, 4, 2, 4, 4, 4]);
    check(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

/// A tall spike surrounded by short bars, and a valley between tall ones — the
/// two shapes where the greedy "extend as far as you can" answer is wrong.
#[test]
fn spikes_and_valleys() {
    assert_eq!(area(&[1, 100, 1]), 100);
    assert_eq!(area(&[5, 1, 5]), 5);
    check(&[6, 2, 5, 4, 5, 1, 6]);
    check(&[2, 1, 4, 5, 1, 3, 3]);
}

/// Widths are measured between the *strictly* shorter bars on each side. An
/// off-by-one there is the single most common bug in this problem.
#[test]
fn width_boundaries() {
    assert_eq!(area(&[2, 1, 2]), 3);
    assert_eq!(area(&[1, 2, 2, 1]), 4);
    assert_eq!(area(&[3, 1, 3, 3, 1, 3]), 6);
}

/// Every histogram of length up to 7 over heights 0..=3 — 21,844 of them. Any
/// bug that a short input can express is in here somewhere.
#[test]
fn every_short_histogram() {
    fn all_vectors(len: usize, max: i32) -> Vec<Vec<i32>> {
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
    for len in 1..=7 {
        for heights in all_vectors(len, 3) {
            check(&heights);
            checked += 1;
        }
    }
    assert_eq!(checked, 21_844, "the sweep changed size");
}

/// Random histograms, across both narrow and wide height ranges — narrow ones
/// produce many ties, wide ones produce almost none.
#[test]
fn matches_reference_on_random_histograms() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for len in [1usize, 2, 3, 10, 50, 200, 400] {
        for max in [1u64, 2, 5, 50, 10_000] {
            for _ in 0..10 {
                check(&rng.histogram(len, max));
            }
        }
    }
}

/// Reversing the histogram mirrors every rectangle, so the answer cannot change.
/// This catches a scan that handles one end differently from the other.
#[test]
fn the_answer_is_symmetric() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);

    for _ in 0..200 {
        let heights = rng.histogram(60, 8);
        let mut reversed = heights.clone();
        reversed.reverse();
        assert_eq!(
            area(&heights),
            area(&reversed),
            "{heights:?} disagrees with its mirror"
        );
    }

    let big = rng.histogram(100_000, 10_000);
    let mut reversed = big.clone();
    reversed.reverse();
    assert_eq!(area(&big), area(&reversed), "large mirror disagrees");
}

/// Two bounds that hold for every histogram: one full-height bar on its own,
/// and one full-width rectangle at the minimum height.
#[test]
fn obvious_rectangles_are_never_missed() {
    let mut rng = Rng(0x0BAD_C0DE_1234_5678);

    for _ in 0..300 {
        let len = 1 + rng.below(80) as usize;
        let heights = rng.histogram(len, 1_000);
        let got = area(&heights);
        let tallest = *heights.iter().max().unwrap();
        let full_width = heights.iter().min().unwrap() * heights.len() as i32;

        assert!(got >= tallest, "{heights:?} -> {got} misses a single bar");
        assert!(
            got >= full_width,
            "{heights:?} -> {got} misses the full-width rectangle"
        );
    }
}

/// The constraint maximum: 10^5 bars all at height 10^4. The answer is 10^9,
/// which fits in `i32` — but an accumulator that goes wider first does not.
#[test]
fn the_largest_possible_answer() {
    assert_eq!(area(&vec![10_000; 100_000]), 1_000_000_000);
    assert_eq!(area(&vec![0; 100_000]), 0);
    assert_eq!(area(&vec![1; 100_000]), 100_000);
}

/// For a non-decreasing histogram every bar reaches the right edge, so the
/// answer is `max of heights[i] * (n - i)` — a closed form that needs no
/// quadratic scan, and so is usable at full size.
#[test]
fn large_non_decreasing() {
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);
    let n = 100_000;

    let mut heights = rng.histogram(n, 10_000);
    heights.sort_unstable();

    let expected = heights
        .iter()
        .enumerate()
        .map(|(i, &h)| h * (n - i) as i32)
        .max()
        .unwrap();
    assert_eq!(area(&heights), expected);
}

/// The mirror: every bar reaches the left edge.
#[test]
fn large_non_increasing() {
    let mut rng = Rng(0xF00D_FACE_8BAD_F00D);
    let n = 100_000;

    let mut heights = rng.histogram(n, 10_000);
    heights.sort_unstable_by(|a, b| b.cmp(a));

    let expected = heights
        .iter()
        .enumerate()
        .map(|(i, &h)| h * (i + 1) as i32)
        .max()
        .unwrap();
    assert_eq!(area(&heights), expected);
}

/// A sawtooth at full size: no bar ever has a neighbour of its own height, so
/// the stack churns on every single step. Purely a shape/performance probe —
/// a quadratic solution will not finish this.
#[test]
fn large_sawtooth() {
    let n = 100_000;
    let heights: Vec<i32> = (0..n)
        .map(|i| if i % 2 == 0 { 10_000 } else { 1 })
        .collect();

    // Height 1 spans everything; height 10000 spans a single bar.
    assert_eq!(area(&heights), 100_000);
}
