#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for s in ["babad", "cbbd", "forgeeksskeegfor"] {
        println!("{s:?} -> {:?}", Solution::longest_palindrome(s.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// "babad" admits both "bab" and "aba", so the tests check the three
    /// properties an answer must have rather than one hardcoded string.
    fn assert_longest(s: &str, expected_len: usize) {
        let got = Solution::longest_palindrome(s.to_string());

        assert!(
            s.contains(&got),
            "{got:?} is not a substring of {s:?}"
        );
        assert!(
            got.chars().eq(got.chars().rev()),
            "{got:?} is not a palindrome"
        );
        assert_eq!(
            got.chars().count(),
            expected_len,
            "wrong length for {s:?}: got {got:?}"
        );
    }

    #[test]
    fn example_1_two_valid_answers() {
        assert_longest("babad", 3);
    }

    #[test]
    fn example_2_even_length_center() {
        assert_longest("cbbd", 2);
    }

    #[test]
    fn single_character() {
        assert_longest("a", 1);
    }

    #[test]
    fn no_palindrome_longer_than_one() {
        assert_longest("abcde", 1);
    }

    #[test]
    fn whole_string_is_a_palindrome() {
        assert_longest("racecar", 7);
    }

    #[test]
    fn all_identical_characters() {
        assert_longest("aaaa", 4);
    }

    #[test]
    fn palindrome_buried_in_the_middle() {
        assert_longest("forgeeksskeegfor", 10);
    }

    #[test]
    fn palindrome_at_the_very_end() {
        assert_longest("xyzabccba", 6);
    }

    #[test]
    fn digits_are_allowed_too() {
        assert_longest("a12321b", 5);
    }

    #[test]
    fn long_run_defeats_naive_center_expansion() {
        assert_longest(&"a".repeat(1000), 1000);
    }
}
