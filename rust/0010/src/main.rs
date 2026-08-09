#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (s, p) in [("aa", "a"), ("aa", "a*"), ("ab", ".*"), ("mississippi", "mis*is*p*.")] {
        println!("{s:?} ~ {p:?} -> {}", Solution::is_match(s.to_string(), p.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_match(s: &str, p: &str) -> bool {
        Solution::is_match(s.to_string(), p.to_string())
    }

    #[test]
    fn example_1_pattern_too_short() {
        assert!(!is_match("aa", "a"));
    }

    #[test]
    fn example_2_star_repeats() {
        assert!(is_match("aa", "a*"));
    }

    #[test]
    fn example_3_dot_star_matches_anything() {
        assert!(is_match("ab", ".*"));
    }

    #[test]
    fn star_can_match_zero_occurrences() {
        assert!(is_match("aab", "c*a*b"));
    }

    #[test]
    fn mississippi_no_match() {
        assert!(!is_match("mississippi", "mis*is*p*."));
    }

    #[test]
    fn mississippi_match() {
        assert!(is_match("mississippi", "mis*is*ip*."));
    }

    #[test]
    fn empty_pattern_matches_only_empty_string() {
        assert!(is_match("", ""));
        assert!(!is_match("a", ""));
    }

    #[test]
    fn star_matches_empty_string() {
        assert!(is_match("", "a*"));
    }

    #[test]
    fn trailing_literal_must_be_consumed() {
        assert!(!is_match("ab", ".*c"));
    }

    /// The star must give back one 'a' so the literal 'a' can match.
    #[test]
    fn star_must_backtrack() {
        assert!(is_match("aaa", "a*a"));
        assert!(is_match("a", "a*a"));
    }

    #[test]
    fn several_stars_some_matching_nothing() {
        assert!(is_match("aaa", "ab*a*c*a"));
        assert!(!is_match("aaba", "ab*a*c*a"));
    }

    #[test]
    fn star_after_the_last_matched_char() {
        assert!(is_match("a", "ab*"));
    }

    #[test]
    fn star_over_a_char_not_present() {
        assert!(!is_match("abcd", "d*"));
    }

    #[test]
    fn dot_star_then_literals() {
        assert!(is_match("ab", ".*.."));
        assert!(!is_match("a", ".*..a*"));
        assert!(is_match("bbbba", ".*a*a"));
    }

    /// Worst case for a naive exponential backtracker: 20 chars, 20 pattern chars.
    #[test]
    fn pathological_backtracking_case() {
        assert!(!is_match(
            "aaaaaaaaaaaaaaaaaaaa",
            "a*a*a*a*a*a*a*a*a*b"
        ));
    }
}
