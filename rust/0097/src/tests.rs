use super::*;
use std::collections::HashSet;

fn interleaves(s1: &str, s2: &str, s3: &str) -> bool {
    Solution::is_interleave(s1.to_string(), s2.to_string(), s3.to_string())
}

/// Every string obtainable by interleaving, built by brute force. Exponential
/// in the total length, so only for short inputs — but it shares nothing with
/// any dynamic programme.
fn all_interleavings(s1: &[u8], s2: &[u8]) -> HashSet<String> {
    fn walk(s1: &[u8], s2: &[u8], prefix: &mut Vec<u8>, out: &mut HashSet<String>) {
        if s1.is_empty() && s2.is_empty() {
            out.insert(String::from_utf8(prefix.clone()).unwrap());
            return;
        }
        if let Some((head, rest)) = s1.split_first() {
            prefix.push(*head);
            walk(rest, s2, prefix, out);
            prefix.pop();
        }
        if let Some((head, rest)) = s2.split_first() {
            prefix.push(*head);
            walk(s1, rest, prefix, out);
            prefix.pop();
        }
    }

    let mut out = HashSet::new();
    walk(s1, s2, &mut Vec::new(), &mut out);
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

    fn below(&mut self, bound: usize) -> usize {
        (self.next_value() % bound as u64) as usize
    }

    /// A string of `len` letters drawn from the first `alphabet` lowercase ones.
    fn word(&mut self, len: usize, alphabet: usize) -> String {
        (0..len)
            .map(|_| (b'a' + self.below(alphabet) as u8) as char)
            .collect()
    }

    /// Interleaves the two, choosing at each step which side to take from, so
    /// the result is a genuine interleaving by construction.
    fn interleave(&mut self, s1: &str, s2: &str) -> String {
        let (mut i, mut j) = (0, 0);
        let (a, b) = (s1.as_bytes(), s2.as_bytes());
        let mut out = Vec::with_capacity(a.len() + b.len());

        while i < a.len() || j < b.len() {
            let take_first = if i == a.len() {
                false
            } else if j == b.len() {
                true
            } else {
                self.below(2) == 0
            };
            if take_first {
                out.push(a[i]);
                i += 1;
            } else {
                out.push(b[j]);
                j += 1;
            }
        }

        String::from_utf8(out).unwrap()
    }
}

#[test]
fn example_1() {
    assert!(interleaves("aabcc", "dbbca", "aadbbcbcac"));
}

#[test]
fn example_2() {
    assert!(!interleaves("aabcc", "dbbca", "aadbbbaccc"));
}

#[test]
fn example_3() {
    assert!(interleaves("", "", ""));
}

/// An empty side leaves the other string, unchanged, as the only answer.
#[test]
fn one_side_empty() {
    assert!(interleaves("", "abc", "abc"));
    assert!(interleaves("abc", "", "abc"));
    assert!(!interleaves("", "abc", "acb"));
    assert!(!interleaves("abc", "", "ab"));
    assert!(!interleaves("", "", "a"));
    assert!(!interleaves("a", "", ""));
}

/// The lengths have to add up, and nothing else needs checking when they do not.
#[test]
fn the_lengths_must_add_up() {
    assert!(!interleaves("a", "b", "ab c"));
    assert!(!interleaves("abc", "def", "abcde"));
    assert!(!interleaves("abc", "def", "abcdefg"));
    assert!(!interleaves("aaa", "aaa", "aaaaaaa"));
    assert!(!interleaves("aaa", "aaa", "aaaaa"));
    // Right characters, right multiset, wrong total.
    assert!(!interleaves("ab", "cd", "abcd".repeat(2).as_str()));
}

/// Both orders of the same two strings, and the trivial single character.
#[test]
fn the_smallest_cases() {
    assert!(interleaves("a", "b", "ab"));
    assert!(interleaves("a", "b", "ba"));
    assert!(!interleaves("a", "b", "aa"));
    assert!(!interleaves("a", "b", "bb"));
    assert!(interleaves("a", "a", "aa"));
}

/// Cases that defeat taking from whichever side matches first. In each of
/// these the greedy walk — prefer `s1` whenever its next character fits —
/// reaches a dead end, yet a genuine interleaving exists.
#[test]
fn greedy_choices_are_not_enough() {
    // Greedy takes 'a' from s1, then cannot place 'b'; s2 had to go first.
    assert!(interleaves("a", "ab", "aba"));
    assert!(interleaves("ab", "a", "aba"));
    assert!(interleaves("b", "ba", "bab"));
    // The forced choice several characters in.
    assert!(interleaves("a", "aab", "aaba"));
    assert!(interleaves("a", "aba", "abaa"));
    assert!(interleaves("a", "abb", "abab"));
    assert!(interleaves("a", "abb", "abba"));
}

/// Near misses: the sources share a long prefix with the target, so a solution
/// only has grounds to reject after exploring well past the start.
#[test]
fn near_misses_are_rejected() {
    assert!(!interleaves("aab", "aac", "aaaaaa"));
    assert!(!interleaves("aab", "aac", "aaaaab"));
    assert!(!interleaves("aab", "aac", "aaaaba"));
    assert!(!interleaves("aab", "aac", "aaaabb"));
    assert!(!interleaves("aab", "aac", "aaaaca"));
    // But the two orderings of the tails are both reachable.
    assert!(interleaves("aab", "aac", "aaaabc"));
    assert!(interleaves("aab", "aac", "aaaacb"));
}

/// Every character right and in the right quantity, but out of order within a
/// side. The multiset is no help — the relative order inside each side is what
/// matters.
#[test]
fn the_multiset_is_not_enough() {
    assert!(!interleaves("ab", "cd", "badc"));
    assert!(!interleaves("abc", "xyz", "cbazyx"));
    assert!(interleaves("abc", "xyz", "axbycz"));
    assert!(interleaves("abc", "xyz", "abcxyz"));
    assert!(interleaves("abc", "xyz", "xyzabc"));
}

/// Checked exhaustively against brute-force enumeration: for short inputs,
/// every string of the right length over a small alphabet is classified, so
/// both the true and the false answers are pinned down.
#[test]
fn it_matches_brute_force_on_short_inputs() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for _ in 0..200 {
        let len1 = rng.below(5);
        let len2 = rng.below(5);
        let s1 = rng.word(len1, 3);
        let s2 = rng.word(len2, 3);
        let expected = all_interleavings(s1.as_bytes(), s2.as_bytes());

        // Candidates that are genuine interleavings.
        for candidate in &expected {
            assert!(
                interleaves(&s1, &s2, candidate),
                "{s1:?} + {s2:?} does make {candidate:?}"
            );
        }

        // Random candidates of the same length, true or false per brute force.
        for _ in 0..20 {
            let candidate = rng.word(len1 + len2, 3);
            assert_eq!(
                interleaves(&s1, &s2, &candidate),
                expected.contains(&candidate),
                "for {s1:?} + {s2:?} -> {candidate:?}"
            );
        }
    }
}

/// A two-letter alphabet maximises the number of near misses, where many
/// prefixes match and only the tail decides.
#[test]
fn it_matches_brute_force_on_a_tiny_alphabet() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);

    for _ in 0..100 {
        let len1 = rng.below(6);
        let len2 = rng.below(6);
        let s1 = rng.word(len1, 2);
        let s2 = rng.word(len2, 2);
        let expected = all_interleavings(s1.as_bytes(), s2.as_bytes());

        for _ in 0..30 {
            let candidate = rng.word(len1 + len2, 2);
            assert_eq!(
                interleaves(&s1, &s2, &candidate),
                expected.contains(&candidate),
                "for {s1:?} + {s2:?} -> {candidate:?}"
            );
        }
    }
}

/// Strings built by actually interleaving must be accepted. This runs at the
/// constraint limit, where anything exponential in the input will not finish.
#[test]
fn constructed_interleavings_are_accepted() {
    let mut rng = Rng(0x0BAD_C0FF_EE0D_DF00);

    for &(len1, len2) in &[(100, 100), (100, 0), (0, 100), (99, 1), (1, 99), (50, 50)] {
        for alphabet in [1usize, 2, 26] {
            let s1 = rng.word(len1, alphabet);
            let s2 = rng.word(len2, alphabet);
            let s3 = rng.interleave(&s1, &s2);

            assert_eq!(s3.len(), len1 + len2);
            assert!(
                interleaves(&s1, &s2, &s3),
                "a constructed interleaving was rejected: {s1:?} + {s2:?} -> {s3:?}"
            );
        }
    }
}

/// The worst case for a solution that explores both branches without
/// memoising: one letter throughout, so every choice matches and the recursion
/// fans out over every path through the grid.
#[test]
fn the_all_same_letter_worst_case() {
    let s1 = "a".repeat(100);
    let s2 = "a".repeat(100);

    assert!(interleaves(&s1, &s2, &"a".repeat(200)));
    assert!(!interleaves(&s1, &s2, &"a".repeat(199)));
    assert!(!interleaves(&s1, &s2, &"a".repeat(201)));

    // One letter out of place, at the very end, so every path must be explored
    // before the answer is known.
    let mut spoiled = "a".repeat(199);
    spoiled.push('b');
    assert!(!interleaves(&s1, &s2, &spoiled));

    // And at the very start.
    let mut spoiled = String::from("b");
    spoiled.push_str(&"a".repeat(199));
    assert!(!interleaves(&s1, &s2, &spoiled));
}

/// A long genuine interleaving with a single character replaced by one that
/// appears in neither source, so it is certainly not an interleaving.
#[test]
fn a_single_wrong_character_is_rejected() {
    let mut rng = Rng(0xFEED_FACE_CAFE_BEEF);

    for _ in 0..20 {
        let s1 = rng.word(80, 3);
        let s2 = rng.word(80, 3);
        let s3 = rng.interleave(&s1, &s2);
        assert!(interleaves(&s1, &s2, &s3), "the original must be accepted");

        let position = rng.below(s3.len());
        let mut spoiled = s3.clone().into_bytes();
        // 'z' occurs in neither source, which uses only a, b, c.
        spoiled[position] = b'z';
        let spoiled = String::from_utf8(spoiled).unwrap();

        assert!(
            !interleaves(&s1, &s2, &spoiled),
            "a foreign character was accepted at {position} in {spoiled:?}"
        );
    }
}

/// Swapping the two sources cannot change the answer.
#[test]
fn the_two_sources_are_symmetric() {
    let mut rng = Rng(0x00C0_FFEE_0000_0001);

    for _ in 0..200 {
        let len1 = rng.below(8);
        let len2 = rng.below(8);
        let s1 = rng.word(len1, 3);
        let s2 = rng.word(len2, 3);
        let s3 = rng.word(s1.len() + s2.len(), 3);

        assert_eq!(
            interleaves(&s1, &s2, &s3),
            interleaves(&s2, &s1, &s3),
            "asymmetric for {s1:?} + {s2:?} -> {s3:?}"
        );
    }
}

/// The inputs are taken by value; calling again with equal strings must give
/// the same answer, so nothing may depend on state left over from a call.
#[test]
fn repeated_calls_agree() {
    for _ in 0..3 {
        assert!(interleaves("aabcc", "dbbca", "aadbbcbcac"));
        assert!(!interleaves("aabcc", "dbbca", "aadbbbaccc"));
    }
}
