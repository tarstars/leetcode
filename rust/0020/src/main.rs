#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for s in ["()", "()[]{}", "(]", "([])", "([)]"] {
        println!("{s:?} -> {}", Solution::is_valid(s.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid(s: &str) -> bool {
        Solution::is_valid(s.to_string())
    }

    #[test]
    fn example_1() {
        assert!(valid("()"));
    }

    #[test]
    fn example_2_all_three_kinds() {
        assert!(valid("()[]{}"));
    }

    #[test]
    fn example_3_mismatched_kinds() {
        assert!(!valid("(]"));
    }

    #[test]
    fn example_4_nested() {
        assert!(valid("([])"));
    }

    /// Correctly balanced counts, but interleaved rather than nested.
    #[test]
    fn example_5_crossed() {
        assert!(!valid("([)]"));
    }

    #[test]
    fn single_bracket_either_way() {
        assert!(!valid("("));
        assert!(!valid(")"));
        assert!(!valid("]"));
    }

    /// A closer arriving with nothing open — the empty-stack pop.
    #[test]
    fn closes_before_opening() {
        assert!(!valid(")("));
        assert!(!valid("(){}}{"));
    }

    /// Openers left over at the end must fail, even though nothing mismatched.
    #[test]
    fn unclosed_openers() {
        assert!(!valid("(("));
        assert!(!valid("([]"));
        assert!(!valid("{[()]"));
    }

    #[test]
    fn deeply_nested() {
        assert!(valid("{[()()]}"));
        assert!(valid("(((((((((())))))))))"));
        assert!(!valid("{[(])}"));
    }

    #[test]
    fn adjacent_pairs_repeated() {
        assert!(valid("()()()()"));
        assert!(valid("{}{}[]()"));
        assert!(!valid("()()(()"));
    }

    /// 10^4 characters, the constraint's upper bound: 5000 nested pairs.
    #[test]
    fn maximum_length_nested() {
        let s = format!("{}{}", "(".repeat(5000), ")".repeat(5000));
        assert!(valid(&s));

        // Same length, one bracket at the very centre swapped for another kind.
        let mut bad: Vec<char> = s.chars().collect();
        bad[4999] = '[';
        assert!(!valid(&bad.into_iter().collect::<String>()));
    }

    /// 10^4 characters that are all openers, then all closers of the wrong kind.
    #[test]
    fn maximum_length_mismatched() {
        let s = format!("{}{}", "{".repeat(5000), ")".repeat(5000));
        assert!(!valid(&s));
    }
}
