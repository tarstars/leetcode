use super::*;

use std::collections::{HashSet, VecDeque};

/// Breadth-first search over the strings themselves: from `a`, apply every
/// single insert, delete and replace, and count the levels until `b` appears.
/// This is the problem's definition taken literally — no table, no recurrence —
/// so it cannot share a bug with a dynamic-programming solution. Exponential,
/// so it is only used on very short inputs.
///
/// Intermediate strings are never allowed to grow past the longer input, which
/// is safe because inserting a character only to delete it again is wasteful.
/// They are free to shrink, though — that is not symmetric. For "cbac" into
/// "aaba" the cheapest route drops to length three (delete, replace, insert)
/// rather than replacing all four characters, so confining lengths to the band
/// between the two inputs would overcount.
fn breadth_first(a: &str, b: &str) -> usize {
    if a == b {
        return 0;
    }

    let alphabet: Vec<char> = {
        let mut set: Vec<char> = a.chars().chain(b.chars()).collect();
        set.sort_unstable();
        set.dedup();
        set
    };

    let high = a.len().max(b.len());

    let mut seen: HashSet<String> = HashSet::from([a.to_owned()]);
    let mut queue: VecDeque<(String, usize)> = VecDeque::from([(a.to_owned(), 0)]);

    while let Some((current, steps)) = queue.pop_front() {
        let chars: Vec<char> = current.chars().collect();
        let mut neighbours: Vec<String> = Vec::new();

        for i in 0..chars.len() {
            // delete
            let mut shorter = chars.clone();
            shorter.remove(i);
            neighbours.push(shorter.into_iter().collect());
            // replace
            for &c in &alphabet {
                if c != chars[i] {
                    let mut next = chars.clone();
                    next[i] = c;
                    neighbours.push(next.into_iter().collect());
                }
            }
        }
        // insert
        if chars.len() < high {
            for i in 0..=chars.len() {
                for &c in &alphabet {
                    let mut next = chars.clone();
                    next.insert(i, c);
                    neighbours.push(next.into_iter().collect());
                }
            }
        }

        for next in neighbours {
            if next == b {
                return steps + 1;
            }
            if seen.insert(next.clone()) {
                queue.push_back((next, steps + 1));
            }
        }
    }

    unreachable!("{a:?} and {b:?} are always connected by edits")
}

/// The usual table, used where breadth-first search is far too slow. It is
/// checked against `breadth_first` before being trusted.
fn reference(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut row: Vec<usize> = (0..=b.len()).collect();

    for (i, &ca) in a.iter().enumerate() {
        let mut diagonal = row[0];
        row[0] = i + 1;

        for (j, &cb) in b.iter().enumerate() {
            let candidate = if ca == cb {
                diagonal
            } else {
                1 + diagonal.min(row[j]).min(row[j + 1])
            };
            diagonal = row[j + 1];
            row[j + 1] = candidate;
        }
    }

    row[b.len()]
}

fn distance(a: &str, b: &str) -> usize {
    let d = Solution::min_distance(a.to_owned(), b.to_owned());
    assert!(d >= 0, "{a:?} -> {b:?} gave a negative distance {d}");
    d as usize
}

fn check(a: &str, b: &str) {
    assert_eq!(distance(a, b), reference(a, b), "{a:?} -> {b:?}");
}

/// Deterministic word generation, so any failure reproduces.
struct Rng(u64);

impl Rng {
    fn next_value(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next_value() % bound as u64) as usize
    }

    fn word(&mut self, len: usize, alphabet: usize) -> String {
        (0..len)
            .map(|_| (b'a' + self.below(alphabet) as u8) as char)
            .collect()
    }
}

#[test]
fn example_1() {
    assert_eq!(Solution::min_distance("horse".into(), "ros".into()), 3);
}

#[test]
fn example_2() {
    assert_eq!(
        Solution::min_distance("intention".into(), "execution".into()),
        5
    );
}

/// Both words may be empty — the constraints allow length zero.
#[test]
fn empty_words() {
    assert_eq!(Solution::min_distance("".into(), "".into()), 0);
    assert_eq!(Solution::min_distance("".into(), "a".into()), 1);
    assert_eq!(Solution::min_distance("a".into(), "".into()), 1);
    assert_eq!(Solution::min_distance("".into(), "abcde".into()), 5);
    assert_eq!(Solution::min_distance("abcde".into(), "".into()), 5);
}

/// Converting a word to itself costs nothing.
#[test]
fn identical_words_cost_nothing() {
    for word in ["", "a", "ab", "hello", "aaaaaaaa", "abcdefghij"] {
        assert_eq!(distance(word, word), 0, "{word:?}");
    }
}

/// One operation of each kind, in isolation.
#[test]
fn a_single_operation() {
    assert_eq!(distance("cat", "cats"), 1); // insert
    assert_eq!(distance("cats", "cat"), 1); // delete
    assert_eq!(distance("cat", "cut"), 1); // replace
    assert_eq!(distance("cat", "act"), 2); // a swap is two operations
}

/// Nothing in common: replacing every character, plus the length difference.
#[test]
fn disjoint_words() {
    assert_eq!(distance("abc", "xyz"), 3);
    assert_eq!(distance("abc", "wxyz"), 4);
    assert_eq!(distance("abcd", "xy"), 4);
}

/// The reference table must agree with literal breadth-first search before it
/// is trusted as an oracle for the larger cases below.
#[test]
fn the_references_agree() {
    let mut rng = Rng(0xA5A5_5A5A_C3C3_3C3C);

    for a_len in 0..=4 {
        for b_len in 0..=4 {
            for _ in 0..6 {
                let a = rng.word(a_len, 3);
                let b = rng.word(b_len, 3);
                assert_eq!(breadth_first(&a, &b), reference(&a, &b), "{a:?} -> {b:?}");
            }
        }
    }
}

/// Against literal breadth-first search, for the sizes where it is affordable.
#[test]
fn matches_breadth_first_search_on_tiny_words() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);

    for a_len in 0..=4 {
        for b_len in 0..=4 {
            for _ in 0..6 {
                let a = rng.word(a_len, 3);
                let b = rng.word(b_len, 3);
                assert_eq!(distance(&a, &b), breadth_first(&a, &b), "{a:?} -> {b:?}");
            }
        }
    }
}

/// Against the reference table, over a wide spread of lengths and alphabets.
/// A small alphabet forces many coincidental matches; a large one forces few.
#[test]
fn matches_reference_on_generated_words() {
    let mut rng = Rng(0xDEAD_BEEF_1234_5678);

    for alphabet in [1usize, 2, 4, 26] {
        for a_len in 0..=12 {
            for b_len in 0..=12 {
                let a = rng.word(a_len, alphabet);
                let b = rng.word(b_len, alphabet);
                check(&a, &b);
            }
        }
    }
}

/// The distance is symmetric: insert and delete are each other's inverse, and
/// replace is its own.
#[test]
fn the_distance_is_symmetric() {
    let mut rng = Rng(0x0BAD_F00D_CAFE_BABE);

    for _ in 0..300 {
        let (a_len, b_len) = (rng.below(15), rng.below(15));
        let a = rng.word(a_len, 4);
        let b = rng.word(b_len, 4);
        assert_eq!(distance(&a, &b), distance(&b, &a), "{a:?} vs {b:?}");
    }
}

/// It is bounded below by the length difference — no operation changes the
/// length by more than one — and above by the longer word, since replacing in
/// place and then padding always works.
#[test]
fn the_distance_stays_within_its_bounds() {
    let mut rng = Rng(0xFACE_B00C_5EED_1234);

    for _ in 0..300 {
        let (a_len, b_len) = (rng.below(20), rng.below(20));
        let a = rng.word(a_len, 3);
        let b = rng.word(b_len, 3);

        let d = distance(&a, &b);
        let low = a.len().abs_diff(b.len());
        let high = a.len().max(b.len());

        assert!(d >= low, "{a:?} -> {b:?} gave {d}, below {low}");
        assert!(d <= high, "{a:?} -> {b:?} gave {d}, above {high}");
    }
}

/// The triangle inequality: going through a third word is never cheaper.
#[test]
fn the_triangle_inequality_holds() {
    let mut rng = Rng(0x7E57_1234_ABCD_EF01);

    for _ in 0..200 {
        let (a_len, b_len, c_len) = (rng.below(10), rng.below(10), rng.below(10));
        let a = rng.word(a_len, 3);
        let b = rng.word(b_len, 3);
        let c = rng.word(c_len, 3);

        assert!(
            distance(&a, &c) <= distance(&a, &b) + distance(&b, &c),
            "{a:?} -> {c:?} exceeds going via {b:?}"
        );
    }
}

/// A prefix costs exactly the characters that must be added.
#[test]
fn a_prefix_costs_the_remainder() {
    let word = "abcdefghijklmnop";

    for cut in 0..=word.len() {
        assert_eq!(distance(&word[..cut], word), word.len() - cut, "cut {cut}");
        assert_eq!(distance(word, &word[..cut]), word.len() - cut, "cut {cut}");
    }
}

/// 500 characters is the constraint's maximum on both sides — a table of
/// 250,000 cells, which rules out unmemoised recursion.
#[test]
fn longest_allowed_input() {
    let a = "a".repeat(500);
    let b = "b".repeat(500);
    assert_eq!(distance(&a, &b), 500);
    assert_eq!(distance(&a, &a), 0);
    assert_eq!(distance(&a, ""), 500);
    assert_eq!(distance("", &b), 500);

    let mut rng = Rng(0x5A5A_1111_2222_3333);
    for (a_len, b_len) in [(500, 500), (500, 1), (1, 500), (500, 499), (250, 500)] {
        let a = rng.word(a_len, 4);
        let b = rng.word(b_len, 4);
        check(&a, &b);
    }
}
