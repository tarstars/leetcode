use super::*;
use std::collections::HashSet;

/// Enumerate all 2^n index masks, canonicalise each subset by sorting, and let
/// a `HashSet` collapse the duplicates. Brute force over positions rather than
/// over values, so it shares nothing with a backtracking skip rule.
fn reference(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut distinct = HashSet::new();

    for mask in 0..1u32 << nums.len() {
        let mut subset: Vec<i32> = nums
            .iter()
            .enumerate()
            .filter(|(index, _)| mask >> index & 1 == 1)
            .map(|(_, &value)| value)
            .collect();
        subset.sort_unstable();
        distinct.insert(subset);
    }

    canonical(distinct.into_iter().collect())
}

/// The output order is unspecified, and so is the order within each subset, so
/// both sides get sorted before comparison.
fn canonical(mut subsets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for subset in &mut subsets {
        subset.sort_unstable();
    }
    subsets.sort();
    subsets
}

fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    canonical(Solution::subsets_with_dup(nums.to_vec()))
}

fn check(nums: &[i32]) {
    assert_eq!(subsets(nums), reference(nums), "for {nums:?}");
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
    assert_eq!(
        subsets(&[1, 2, 2]),
        vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2]
        ]
    );
}

#[test]
fn example_2() {
    assert_eq!(subsets(&[0]), vec![Vec::<i32>::new(), vec![0]]);
}

/// The empty subset is always present, for every input.
#[test]
fn the_empty_subset_is_always_included() {
    let mut rng = Rng(0x0BAD_C0DE_1234_5678);
    for _ in 0..50 {
        let len = 1 + rng.below(9) as usize;
        let nums = rng.list(len, 5);
        assert!(
            subsets(&nums).contains(&Vec::new()),
            "{nums:?} is missing the empty subset"
        );
    }
}

/// With no duplicates there are exactly 2^n subsets.
#[test]
fn distinct_values_give_the_full_power_set() {
    for n in 1..=10usize {
        let nums: Vec<i32> = (0..n as i32).collect();
        assert_eq!(subsets(&nums).len(), 1 << n, "wrong count for n = {n}");
        check(&nums);
    }
}

/// All copies of one value: the answer is just "how many do I take".
#[test]
fn a_single_repeated_value() {
    for n in 1..=10usize {
        let nums = vec![7; n];
        assert_eq!(subsets(&nums).len(), n + 1, "wrong count for {n} copies");
        check(&nums);
    }
}

/// The input is not sorted and duplicates are not adjacent.
#[test]
fn unsorted_input_with_scattered_duplicates() {
    check(&[4, 4, 4, 1, 4]);
    check(&[2, 1, 2]);
    check(&[3, 1, 3, 1, 3]);
    check(&[5, -5, 5, -5]);
}

/// Negative values and the ends of the permitted range.
#[test]
fn negative_and_extreme_values() {
    check(&[-10, 10]);
    check(&[-10, -10, 10, 10]);
    check(&[0, -1, 1, 0]);
    check(&[-10; 5]);
}

/// The count is the product of (multiplicity + 1) over distinct values — an
/// arithmetic check that does not depend on the oracle at all.
#[test]
fn the_count_is_a_product_of_multiplicities() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);

    for _ in 0..200 {
        let len = 1 + rng.below(10) as usize;
        let nums = rng.list(len, 6);

        let mut multiplicity = std::collections::HashMap::new();
        for &value in &nums {
            *multiplicity.entry(value).or_insert(0usize) += 1;
        }
        let expected: usize = multiplicity.values().map(|count| count + 1).product();

        assert_eq!(
            subsets(&nums).len(),
            expected,
            "{nums:?} should have {expected} subsets"
        );
    }
}

/// No subset may appear twice, and every subset must be a sub-multiset of the
/// input — the two halves of "the power set, deduplicated".
#[test]
fn the_output_is_a_deduplicated_sub_multiset_family() {
    let mut rng = Rng(0xF00D_FACE_8BAD_F00D);

    for _ in 0..200 {
        let len = 1 + rng.below(10) as usize;
        let nums = rng.list(len, 4);
        let got = subsets(&nums);

        let unique: HashSet<&Vec<i32>> = got.iter().collect();
        assert_eq!(unique.len(), got.len(), "{nums:?} has a duplicate subset");

        for subset in &got {
            let mut available = nums.clone();
            for value in subset {
                let position = available
                    .iter()
                    .position(|candidate| candidate == value)
                    .unwrap_or_else(|| panic!("{nums:?} -> {subset:?} uses {value} too often"));
                available.swap_remove(position);
            }
        }
    }
}

/// Every multiset of length up to 6 over four values — 4,095 inputs, covering
/// every arrangement of duplicates that short inputs admit.
#[test]
fn every_short_input() {
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
    for len in 1..=6 {
        for nums in all_lists(len, 3) {
            check(&nums);
            checked += 1;
        }
    }
    assert_eq!(checked, 5_460, "the sweep changed size");
}

/// Random inputs across lengths and duplicate densities. A narrow spread makes
/// almost everything a duplicate; a wide one makes almost nothing one.
#[test]
fn matches_reference_on_random_inputs() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for len in 1..=10usize {
        for spread in [1u64, 2, 3, 6, 21] {
            for _ in 0..10 {
                check(&rng.list(len, spread));
            }
        }
    }
}

/// The constraint maximum, in the three shapes with a known answer count.
#[test]
fn the_largest_inputs() {
    let all_distinct: Vec<i32> = (-5..5).collect();
    assert_eq!(all_distinct.len(), 10);
    assert_eq!(subsets(&all_distinct).len(), 1024);

    let all_same = vec![-10; 10];
    assert_eq!(subsets(&all_same).len(), 11);

    // Five values twice each: 3^5 = 243.
    let paired: Vec<i32> = (0..5).flat_map(|value| [value, value]).collect();
    assert_eq!(paired.len(), 10);
    assert_eq!(subsets(&paired).len(), 243);
    check(&paired);
}
