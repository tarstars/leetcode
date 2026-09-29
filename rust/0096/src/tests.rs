use super::*;
use std::collections::HashSet;

/// The published Catalan numbers, written out rather than computed.
const KNOWN: [i64; 20] = [
    1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012, 742900, 2674440, 9694845,
    35357670, 129644790, 477638700, 1767263190,
];

#[test]
fn example_1() {
    assert_eq!(Solution::num_trees(3), 5);
}

#[test]
fn example_2() {
    assert_eq!(Solution::num_trees(1), 1);
}

/// Two nodes, two trees: either one can be the root.
#[test]
fn the_smallest_cases() {
    assert_eq!(Solution::num_trees(2), 2);
    assert_eq!(Solution::num_trees(4), 14);
}

#[test]
fn it_matches_the_published_catalan_numbers() {
    for n in 1..=19i32 {
        assert_eq!(
            i64::from(Solution::num_trees(n)),
            KNOWN[n as usize],
            "wrong count for n = {n}"
        );
    }
}

/// The largest allowed input, which is also the one where a careless
/// intermediate overflows an i32 even though the answer does not.
#[test]
fn the_largest_input_is_exact() {
    assert_eq!(Solution::num_trees(19), 1_767_263_190);
    assert!(
        1_767_263_190i64 < i64::from(i32::MAX),
        "the answer really does fit in an i32"
    );
}

/// The closed form C(2n, n) / (n + 1), computed in u128 so nothing overflows —
/// a different formula, not the recurrence.
#[test]
fn it_matches_the_closed_form() {
    fn closed_form(n: u128) -> u128 {
        let mut binomial: u128 = 1;
        for k in 0..n {
            binomial = binomial * (2 * n - k) / (k + 1);
        }
        binomial / (n + 1)
    }

    for n in 1..=19i32 {
        assert_eq!(
            u128::from(Solution::num_trees(n) as u32),
            closed_form(n as u128),
            "wrong count for n = {n}"
        );
    }
}

/// Catalan numbers also count balanced bracket strings of length 2n. Brute
/// forcing every bit pattern of that length and keeping the balanced ones
/// shares nothing at all with counting trees.
#[test]
fn it_matches_the_balanced_bracket_count() {
    fn balanced_strings(n: u32) -> i64 {
        let mut found = 0;
        for pattern in 0u32..(1u32 << (2 * n)) {
            let mut depth: i32 = 0;
            let mut ok = true;
            for position in 0..2 * n {
                depth += if pattern >> position & 1 == 1 { 1 } else { -1 };
                if depth < 0 {
                    ok = false;
                    break;
                }
            }
            if ok && depth == 0 {
                found += 1;
            }
        }
        found
    }

    for n in 1..=8u32 {
        assert_eq!(
            i64::from(Solution::num_trees(n as i32)),
            balanced_strings(n),
            "wrong count for n = {n}"
        );
    }
}

/// Building every distinct tree shape and counting the set — enumeration with
/// deduplication rather than arithmetic, so a wrong recurrence cannot agree
/// with it by accident.
#[test]
fn it_matches_an_explicit_enumeration() {
    /// Every distinct shape with `n` nodes, rendered as a parenthesisation.
    fn shapes(n: usize) -> Vec<String> {
        if n == 0 {
            return vec![".".to_string()];
        }
        let mut out = Vec::new();
        for left_size in 0..n {
            for left in shapes(left_size) {
                for right in shapes(n - 1 - left_size) {
                    out.push(format!("({left} {right})"));
                }
            }
        }
        out
    }

    for n in 1..=9usize {
        let all = shapes(n);
        let distinct: HashSet<&String> = all.iter().collect();
        assert_eq!(distinct.len(), all.len(), "the enumeration itself repeats");
        assert_eq!(
            Solution::num_trees(n as i32) as usize,
            all.len(),
            "wrong count for n = {n}"
        );
    }
}

/// The counts grow strictly, so an off-by-one in the loop bounds that shifts
/// the whole sequence gets caught even where the values look plausible.
#[test]
fn the_sequence_grows_strictly() {
    let counts: Vec<i32> = (1..=19).map(Solution::num_trees).collect();
    for pair in counts.windows(2) {
        assert!(
            pair[1] > pair[0],
            "the sequence stalls or falls: {} then {}",
            pair[0],
            pair[1]
        );
    }
    assert!(counts.iter().all(|&c| c > 0), "a count went negative");
}

/// Repeated calls must agree. A solution caching into a `static mut` or a
/// thread local, or one that mutates shared state, breaks here.
#[test]
fn repeated_calls_agree() {
    for n in 1..=19i32 {
        let first = Solution::num_trees(n);
        for _ in 0..3 {
            assert_eq!(Solution::num_trees(n), first, "unstable for n = {n}");
        }
    }

    // Descending order too, in case a solution assumes it is called upward.
    for n in (1..=19i32).rev() {
        assert_eq!(i64::from(Solution::num_trees(n)), KNOWN[n as usize]);
    }
}
