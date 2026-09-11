use super::*;

fn sorted(nums: &[i32]) -> Vec<i32> {
    let mut result = nums.to_vec();
    Solution::sort_colors(&mut result);
    result
}

/// The full specification in two parts: the output is non-decreasing, and it
/// holds exactly the same values as the input. Sorting is a *permutation*, so
/// checking order alone would accept an answer that invented or lost colours.
fn assert_sorted_permutation(input: &[i32], output: &[i32]) {
    assert_eq!(
        output.len(),
        input.len(),
        "length changed: {input:?} -> {output:?}"
    );
    assert!(
        output.windows(2).all(|w| w[0] <= w[1]),
        "not sorted: {input:?} -> {output:?}"
    );

    for colour in 0..=2 {
        let before = input.iter().filter(|&&v| v == colour).count();
        let after = output.iter().filter(|&&v| v == colour).count();
        assert_eq!(
            before, after,
            "colour {colour} count changed: {input:?} -> {output:?}"
        );
    }
}

fn check(nums: &[i32]) {
    let got = sorted(nums);
    assert_sorted_permutation(nums, &got);

    // Also against the library sort, which the solution itself may not use.
    let mut want = nums.to_vec();
    want.sort();
    assert_eq!(got, want, "for {nums:?}");
}

/// Every array of the given length over {0, 1, 2}.
fn every_array(len: usize) -> impl Iterator<Item = Vec<i32>> {
    let total = 3usize.pow(len as u32);

    (0..total).map(move |mut code| {
        let mut nums = Vec::with_capacity(len);
        for _ in 0..len {
            nums.push((code % 3) as i32);
            code /= 3;
        }
        nums
    })
}

/// Deterministic array generation, so any failure reproduces.
struct Rng(u64);

impl Rng {
    fn next_value(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn colours(&mut self, len: usize) -> Vec<i32> {
        (0..len).map(|_| (self.next_value() % 3) as i32).collect()
    }
}

#[test]
fn example_1() {
    assert_eq!(sorted(&[2, 0, 2, 1, 1, 0]), vec![0, 0, 1, 1, 2, 2]);
}

#[test]
fn example_2() {
    assert_eq!(sorted(&[2, 0, 1]), vec![0, 1, 2]);
}

/// A single element is the shortest legal input.
#[test]
fn a_single_element() {
    assert_eq!(sorted(&[0]), vec![0]);
    assert_eq!(sorted(&[1]), vec![1]);
    assert_eq!(sorted(&[2]), vec![2]);
}

/// Only one colour present, including the case where it is absent entirely
/// from the middle.
#[test]
fn a_single_colour() {
    for colour in 0..=2 {
        for len in 1..=10 {
            let nums = vec![colour; len];
            assert_eq!(sorted(&nums), nums, "{len} of colour {colour}");
        }
    }

    check(&[0, 0, 2, 2]); // no 1s
    check(&[1, 1, 2, 2]); // no 0s
    check(&[0, 0, 1, 1]); // no 2s
}

/// Already sorted input must survive untouched.
#[test]
fn already_sorted() {
    check(&[0, 0, 1, 1, 2, 2]);
    check(&[0, 1, 2]);
    check(&[0, 0, 0, 1, 2, 2, 2]);
}

/// Exactly reversed, which moves every element.
#[test]
fn reverse_sorted() {
    check(&[2, 2, 1, 1, 0, 0]);
    check(&[2, 1, 0]);
    check(&[2, 2, 2, 1, 0, 0, 0]);
}

/// Exhaustive over every array up to length 8 — 9,840 in all.
#[test]
fn every_short_array() {
    for len in 1..=8 {
        for nums in every_array(len) {
            check(&nums);
        }
    }
}

/// A two-way swap that ignores the value it swapped in leaves a stray 2 near
/// the front; these are the shapes where that shows up soonest.
#[test]
fn a_two_at_the_front() {
    check(&[2, 0, 0, 0]);
    check(&[2, 1, 1, 1]);
    check(&[2, 2, 0, 1]);
    check(&[2, 0, 2, 0]);
    check(&[1, 2, 0]);
    check(&[2, 0, 1, 2, 0, 1]);
}

/// Longer generated arrays, at several lengths.
#[test]
fn matches_on_generated_arrays() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for len in [9usize, 10, 17, 33, 64, 100, 299] {
        for _ in 0..40 {
            check(&rng.colours(len));
        }
    }
}

/// Skewed inputs, where one colour dominates and the others are rare.
#[test]
fn heavily_skewed_arrays() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);

    for dominant in 0..=2 {
        for len in [20usize, 50, 200] {
            for _ in 0..20 {
                let mut nums = vec![dominant; len];
                for _ in 0..3 {
                    let at = (rng.next_value() % len as u64) as usize;
                    nums[at] = (rng.next_value() % 3) as i32;
                }
                check(&nums);
            }
        }
    }
}

/// 300 elements is the constraint's maximum.
#[test]
fn largest_allowed_input() {
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);

    check(&rng.colours(300));
    check(&vec![2; 300]);
    check(&vec![0; 300]);

    let mut alternating: Vec<i32> = (0..300).map(|i| (i % 3) as i32).collect();
    check(&alternating);

    alternating.reverse();
    check(&alternating);
}
