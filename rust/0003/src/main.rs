#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for s in ["abcabcbb", "bbbbb", "pwwkew", ""] {
        println!("{s:?} -> {}", Solution::length_of_longest_substring(s.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn longest(s: &str) -> i32 {
        Solution::length_of_longest_substring(s.to_string())
    }

    #[test]
    fn example_1() {
        assert_eq!(longest("abcabcbb"), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(longest("bbbbb"), 1);
    }

    #[test]
    fn example_3() {
        assert_eq!(longest("pwwkew"), 3);
    }

    #[test]
    fn handles_an_empty_string() {
        assert_eq!(longest(""), 0);
    }

    #[test]
    fn handles_a_single_character() {
        assert_eq!(longest("a"), 1);
    }

    #[test]
    fn handles_all_distinct_characters() {
        assert_eq!(longest("abcdef"), 6);
    }

    #[test]
    fn handles_spaces_and_symbols() {
        assert_eq!(longest(" !@ !@#"), 4);
    }

    #[test]
    fn repeat_is_outside_the_window() {
        assert_eq!(longest("abba"), 2);
    }
}
