use super::*;
use std::collections::{HashMap, HashSet};

/// Builds the complete set of strings reachable from `s` by the scrambling
/// procedure, then membership answers the question. This is constructive where
/// the real solution is a decision procedure, so the two share no machinery.
/// Exponential in spirit, but memoised per distinct substring, which keeps it
/// usable up to about length 8.
fn all_scrambles<'a>(s: &'a str, memo: &mut HashMap<&'a str, HashSet<String>>) -> HashSet<String> {
    if let Some(known) = memo.get(s) {
        return known.clone();
    }

    let mut reachable = HashSet::new();
    reachable.insert(s.to_string());

    for split in 1..s.len() {
        let left = all_scrambles(&s[..split], memo);
        let right = all_scrambles(&s[split..], memo);
        for a in &left {
            for b in &right {
                reachable.insert(format!("{a}{b}"));
                reachable.insert(format!("{b}{a}"));
            }
        }
    }

    memo.insert(s, reachable.clone());
    reachable
}

fn reference(s1: &str, s2: &str) -> bool {
    all_scrambles(s1, &mut HashMap::new()).contains(s2)
}

fn scrambled(s1: &str, s2: &str) -> bool {
    Solution::is_scramble(s1.to_string(), s2.to_string())
}

fn check(s1: &str, s2: &str) {
    assert_eq!(
        scrambled(s1, s2),
        reference(s1, s2),
        "for {s1:?} -> {s2:?}"
    );
}

/// Every string of the given length over the first `letters` lowercase letters.
fn all_strings(len: usize, letters: u32) -> Vec<String> {
    let mut out = vec![String::new()];
    for _ in 0..len {
        let mut next = Vec::with_capacity(out.len() * letters as usize);
        for prefix in &out {
            for letter in 0..letters {
                let mut grown = prefix.clone();
                grown.push((b'a' + letter as u8) as char);
                next.push(grown);
            }
        }
        out = next;
    }
    out
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

    fn below(&mut self, bound: u64) -> usize {
        (self.next_value() % bound) as usize
    }

    fn word(&mut self, len: usize, letters: u64) -> String {
        (0..len)
            .map(|_| (b'a' + self.below(letters) as u8) as char)
            .collect()
    }

    /// Runs the scrambling procedure from the problem statement, so the result
    /// is a genuine scramble of `s` by construction.
    fn scramble(&mut self, s: &str) -> String {
        if s.len() <= 1 {
            return s.to_string();
        }
        let split = 1 + self.below(s.len() as u64 - 1);
        let left = self.scramble(&s[..split]);
        let right = self.scramble(&s[split..]);
        if self.next_value() % 2 == 0 {
            format!("{left}{right}")
        } else {
            format!("{right}{left}")
        }
    }
}

#[test]
fn example_1() {
    assert!(scrambled("great", "rgeat"));
}

#[test]
fn example_2() {
    assert!(!scrambled("abcde", "caebd"));
}

#[test]
fn example_3() {
    assert!(scrambled("a", "a"));
}

/// Length one, the base case.
#[test]
fn single_characters() {
    assert!(scrambled("a", "a"));
    assert!(!scrambled("a", "b"));
    assert!(!scrambled("z", "a"));
}

/// A string is always a scramble of itself — split anywhere and swap nothing.
#[test]
fn a_string_is_a_scramble_of_itself() {
    let mut rng = Rng(0x0BAD_C0DE_1234_5678);
    for len in [1usize, 2, 3, 7, 15, 30] {
        let word = rng.word(len, 4);
        assert!(scrambled(&word, &word), "{word:?} is not its own scramble");
    }
}

/// Length two: only the two orderings are reachable.
#[test]
fn pairs() {
    assert!(scrambled("ab", "ab"));
    assert!(scrambled("ab", "ba"));
    assert!(!scrambled("ab", "aa"));
    assert!(scrambled("aa", "aa"));
}

/// Differing character counts can never be fixed by splitting, at any length.
#[test]
fn different_letter_counts_are_never_scrambles() {
    assert!(!scrambled("abc", "abd"));
    assert!(!scrambled("aab", "abb"));
    assert!(!scrambled("abcd", "abce"));
    assert!(!scrambled(&"a".repeat(30), &("a".repeat(29) + "b")));
}

/// The relation is symmetric: undoing each swap scrambles `s2` back into `s1`.
#[test]
fn the_relation_is_symmetric() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);

    for _ in 0..60 {
        let s1 = rng.word(8, 3);
        let s2 = rng.word(8, 3);
        assert_eq!(
            scrambled(&s1, &s2),
            scrambled(&s2, &s1),
            "{s1:?} and {s2:?} disagree when swapped"
        );
    }
}

/// Every pair of strings of length up to 7 over two letters, and up to 4 over
/// three letters — about 29,000 pairs, checked against the generated set.
#[test]
fn every_short_pair() {
    let mut checked = 0;
    for (letters, max_len) in [(2u32, 7usize), (3, 4)] {
        for len in 1..=max_len {
            let words = all_strings(len, letters);
            for s1 in &words {
                let reachable = all_scrambles(s1, &mut HashMap::new());
                for s2 in &words {
                    assert_eq!(
                        scrambled(s1, s2),
                        reachable.contains(s2),
                        "for {s1:?} -> {s2:?}"
                    );
                    checked += 1;
                }
            }
        }
    }
    assert_eq!(checked, 29_224, "the sweep changed size");
}

/// Strings built by actually running the scrambling procedure must be accepted.
/// These are the positives that a too-strict solution fails.
#[test]
fn constructed_scrambles_are_accepted() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for len in [2usize, 3, 5, 9, 16, 25, 30] {
        for letters in [1u64, 2, 4, 26] {
            for _ in 0..10 {
                let s1 = rng.word(len, letters);
                let s2 = rng.scramble(&s1);
                assert!(scrambled(&s1, &s2), "{s1:?} -> {s2:?} should be accepted");
            }
        }
    }
}

/// Random pairs at a length the generated-set oracle can still reach, mostly
/// negatives, which is where an over-eager solution fails.
#[test]
fn matches_reference_on_random_pairs() {
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);

    for len in [2usize, 3, 4, 5, 6, 7, 8] {
        for letters in [2u64, 3] {
            for _ in 0..12 {
                let s1 = rng.word(len, letters);
                let s2 = rng.word(len, letters);
                check(&s1, &s2);
            }
        }
    }
}

/// Anagram pairs specifically: the letter-count shortcut cannot decide these,
/// so the recursion has to do the work.
#[test]
fn anagram_pairs_at_moderate_length() {
    let mut rng = Rng(0xF00D_FACE_8BAD_F00D);

    for _ in 0..40 {
        let s1 = rng.word(8, 3);
        let mut letters: Vec<char> = s1.chars().collect();
        // Fisher-Yates, so s2 is always an anagram but usually not a scramble.
        for i in (1..letters.len()).rev() {
            letters.swap(i, rng.below(i as u64 + 1));
        }
        let s2: String = letters.into_iter().collect();
        check(&s1, &s2);
    }
}

/// A genuine `false` pair at the maximum length, over three letters. The answer
/// is only reached after every split has been refuted, so a solution with
/// neither memoisation nor an anagram early exit does not finish this — the
/// unoptimised recursion was still going after 200 million calls. With either
/// one it takes a couple of hundred.
#[test]
fn a_pathological_negative_at_full_length() {
    let s1 = "aaccccabccaccabbbcabbcabbaaabb";
    let s2 = "cabbcacbaccacaabbaacacbbacbcbb";
    assert_eq!(s1.len(), 30);

    let mut sorted1: Vec<char> = s1.chars().collect();
    let mut sorted2: Vec<char> = s2.chars().collect();
    sorted1.sort_unstable();
    sorted2.sort_unstable();
    assert_eq!(sorted1, sorted2, "the pair must be anagrams to be interesting");

    assert!(!scrambled(s1, s2));
}

/// The constraint maximum, in the shapes that make a naive recursion explode:
/// one letter repeated, and two letters alternating. Both are true, and both
/// require memoisation to answer in reasonable time.
#[test]
fn the_largest_inputs() {
    let all_same = "a".repeat(30);
    assert!(scrambled(&all_same, &all_same));

    let alternating: String = (0..30)
        .map(|i| if i % 2 == 0 { 'a' } else { 'b' })
        .collect();
    let reversed: String = alternating.chars().rev().collect();
    assert!(scrambled(&alternating, &alternating));
    assert!(scrambled(&alternating, &reversed));

    // A near-miss at full length: one letter moved, breaking the anagram.
    let mut broken: Vec<char> = alternating.chars().collect();
    broken[0] = 'c';
    assert!(!scrambled(&alternating, &broken.into_iter().collect::<String>()));
}
