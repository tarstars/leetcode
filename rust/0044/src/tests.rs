use super::*;

fn matches(s: &str, pattern: &str) -> bool {
    Solution::is_match(s.to_owned(), pattern.to_owned())
}

#[test]
fn example_1() {
    assert!(!matches("aa", "a"));
}

#[test]
fn example_2() {
    assert!(matches("aa", "*"));
}

#[test]
fn example_3() {
    assert!(!matches("cb", "?a"));
}

#[test]
fn question_mark_matches_exactly_one_character() {
    assert!(matches("cat", "c?t"));
    assert!(!matches("ct", "c?t"));
}

#[test]
fn star_can_match_empty_or_many_characters() {
    assert!(matches("adceb", "*a*b"));
    assert!(matches("ab", "a*b"));
    assert!(matches("axxxb", "a*b"));
}

#[test]
fn wildcard_choices_must_be_globally_consistent() {
    assert!(!matches("acdcb", "a*c?b"));
    assert!(matches("abefcdgiescdfimde", "ab*cd?i*de"));
}

#[test]
fn handles_empty_inputs() {
    assert!(matches("", ""));
    assert!(matches("", "***"));
    assert!(!matches("", "?"));
    assert!(!matches("a", ""));
}

#[test]
fn consecutive_stars_are_equivalent_to_one_star() {
    assert!(matches("mississippi", "m***iss*ppi"));
}

#[test]
fn requires_a_full_string_match() {
    assert!(!matches("prefix-match-suffix", "match"));
}

#[test]
fn handles_maximum_length_inputs() {
    assert!(matches(&"a".repeat(2_000), "*"));
    assert!(matches(&"a".repeat(1_000), &"*a".repeat(1_000)));
}
