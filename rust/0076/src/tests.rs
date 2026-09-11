use super::*;

use std::collections::HashMap;

/// Does this window hold every character of `t`, counted with multiplicity?
fn covers(window: &str, t: &str) -> bool {
    let mut needed: HashMap<char, i32> = HashMap::new();
    for c in t.chars() {
        *needed.entry(c).or_insert(0) += 1;
    }
    for c in window.chars() {
        if let Some(count) = needed.get_mut(&c) {
            *count -= 1;
            if *count == 0 {
                needed.remove(&c);
            }
        }
    }
    needed.is_empty()
}

/// The length of the shortest covering window, by trying every one of them.
/// Quadratic, so it is only used on short inputs, but it follows the statement
/// literally rather than sliding anything.
fn shortest_window_length(s: &str, t: &str) -> Option<usize> {
    let chars: Vec<char> = s.chars().collect();
    let mut best: Option<usize> = None;

    for start in 0..chars.len() {
        for end in start + 1..=chars.len() {
            let window: String = chars[start..end].iter().collect();
            if covers(&window, t) {
                let len = end - start;
                best = Some(best.map_or(len, |b: usize| b.min(len)));
                break; // any longer window from this start is worse
            }
        }
    }

    best
}

fn min_window(s: &str, t: &str) -> String {
    Solution::min_window(s.to_owned(), t.to_owned())
}

/// Checks the answer against the definition rather than against one particular
/// winning window. Several windows can share the minimal length, and the
/// statement only promises uniqueness for its own test data, so requiring an
/// exact string would be checking more than the problem specifies.
fn check(s: &str, t: &str) {
    let got = min_window(s, t);
    let expected = shortest_window_length(s, t);

    match expected {
        None => assert_eq!(got, "", "no window covers {t:?} in {s:?}"),
        Some(len) => {
            assert!(
                s.contains(&got),
                "{got:?} is not a substring of {s:?} (t = {t:?})"
            );
            assert!(covers(&got, t), "{got:?} does not cover {t:?} (s = {s:?})");
            assert_eq!(
                got.chars().count(),
                len,
                "{got:?} is not the shortest window for t = {t:?} in {s:?}"
            );
        }
    }
}

/// Deterministic string generation, so any failure reproduces.
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
    assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC");
}

#[test]
fn example_2() {
    assert_eq!(min_window("a", "a"), "a");
}

/// Two 'a's are required and only one is available, so no window covers t.
#[test]
fn example_3() {
    assert_eq!(min_window("a", "aa"), "");
}

/// No window exists whenever some character is missing outright, or is present
/// too few times.
#[test]
fn no_window_exists() {
    assert_eq!(min_window("abc", "d"), "");
    assert_eq!(min_window("abc", "abcd"), "");
    assert_eq!(min_window("aab", "aaa"), "");
    assert_eq!(min_window("a", "ab"), "");
    assert_eq!(min_window("xyz", "xxy"), "");
}

/// t longer than s can never be covered.
#[test]
fn t_longer_than_s() {
    assert_eq!(min_window("a", "aa"), "");
    assert_eq!(min_window("ab", "abc"), "");
    assert_eq!(min_window("hello", "helloo"), "");
}

/// The whole of s is sometimes the only answer.
#[test]
fn the_whole_string() {
    assert_eq!(min_window("abc", "abc"), "abc");
    assert_eq!(min_window("abc", "cba"), "abc");
    assert_eq!(min_window("aa", "aa"), "aa");
}

/// Characters repeated in t must be matched that many times over.
#[test]
fn duplicates_in_t_are_counted() {
    check("aaflslflsslfa", "aaa");
    check("aaaaaaaaab", "aab");
    assert_eq!(min_window("aa", "aa"), "aa");
    assert_eq!(min_window("aab", "ab"), "ab");
    assert_eq!(min_window("bbaa", "ab"), "ba");
}

/// Case matters: 'a' and 'A' are different characters.
#[test]
fn upper_and_lower_case_are_distinct() {
    assert_eq!(min_window("a", "A"), "");
    assert_eq!(min_window("A", "a"), "");
    assert_eq!(min_window("aA", "Aa"), "aA");
    check("aAbBcC", "ABC");
    check("aAbBcC", "abc");
}

/// The answer is not always a prefix or suffix — example 1's window sits at the
/// end only after several earlier candidates are beaten.
#[test]
fn the_window_can_be_anywhere() {
    check("ADOBECODEBANC", "ABC");
    check("bbaacdbca", "abc");
    check("cabwefgewcwaefgcf", "cae");
    check("abcdefghijklmnopqrstuvwxyzabc", "abc");
}

/// Generated pairs over small alphabets, where coincidental matches are common.
#[test]
fn matches_reference_on_generated_pairs() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for alphabet in [2usize, 3, 5] {
        for s_len in 1..=14 {
            for t_len in 1..=5 {
                for _ in 0..8 {
                    let s = rng.word(s_len, alphabet);
                    let t = rng.word(t_len, alphabet);
                    check(&s, &t);
                }
            }
        }
    }
}

/// A single character alphabet reduces the problem to counting, and makes the
/// multiplicity rule the only thing that matters.
#[test]
fn a_single_character_alphabet() {
    for s_len in 1..=12 {
        for t_len in 1..=12 {
            let s = "a".repeat(s_len);
            let t = "a".repeat(t_len);

            if t_len <= s_len {
                assert_eq!(min_window(&s, &t), t, "{s_len} vs {t_len}");
            } else {
                assert_eq!(min_window(&s, &t), "", "{s_len} vs {t_len}");
            }
        }
    }
}

/// 10^5 on both sides is the constraint's maximum, so the answers here are
/// constructed rather than searched for — a quadratic scan would not finish.
#[test]
fn longest_allowed_input() {
    const N: usize = 100_000;

    // One 'b' buried in a sea of 'a's: the shortest cover is the pair at its edge.
    let s = format!("{}b{}", "a".repeat(N / 2), "a".repeat(N / 2 - 1));
    assert_eq!(s.len(), N);
    let got = min_window(&s, "ab");
    assert_eq!(got.len(), 2);
    assert!(covers(&got, "ab"));

    // Nothing to find.
    assert_eq!(min_window(&"a".repeat(N), "b"), "");

    // Everything is needed.
    let s = "a".repeat(N);
    assert_eq!(min_window(&s, &s), s);

    // t is one character, present once at the very end.
    let s = format!("{}z", "a".repeat(N - 1));
    assert_eq!(min_window(&s, "z"), "z");

    // A long t requiring many duplicates.
    let s = "ab".repeat(N / 2);
    let got = min_window(&s, &"a".repeat(100));
    assert_eq!(got.chars().filter(|&c| c == 'a').count(), 100);
    assert_eq!(got.len(), 199);
}
