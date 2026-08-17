#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for n in 1..=3 {
        println!("{n} -> {:?}", Solution::generate_parenthesis(n));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Any order is allowed.
    fn generate(n: i32) -> Vec<String> {
        let mut got = Solution::generate_parenthesis(n);
        got.sort();
        got
    }

    fn is_balanced(s: &str) -> bool {
        let mut depth = 0i32;
        for c in s.chars() {
            depth += if c == '(' { 1 } else { -1 };
            if depth < 0 {
                return false;
            }
        }
        depth == 0
    }

    #[test]
    fn example_1() {
        assert_eq!(
            generate(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn example_2_single_pair() {
        assert_eq!(generate(1), vec!["()"]);
    }

    #[test]
    fn two_pairs() {
        assert_eq!(generate(2), vec!["(())", "()()"]);
    }

    /// The count is the nth Catalan number.
    #[test]
    fn catalan_counts() {
        let want = [1, 2, 5, 14, 42, 132, 429, 1430];
        for (i, &count) in want.iter().enumerate() {
            let n = i as i32 + 1;
            assert_eq!(generate(n).len(), count, "n = {n}");
        }
    }

    /// Every string must be well-formed and exactly 2n characters long.
    #[test]
    fn every_result_is_well_formed() {
        for n in 1..=8 {
            for s in generate(n) {
                assert_eq!(s.len(), 2 * n as usize, "{s} has the wrong length");
                assert!(is_balanced(&s), "{s} is not balanced");
            }
        }
    }

    #[test]
    fn no_duplicates() {
        for n in 1..=8 {
            let got = generate(n);
            let unique: HashSet<&String> = got.iter().collect();
            assert_eq!(unique.len(), got.len(), "duplicates at n = {n}");
        }
    }

    /// The extremes are always present: fully nested and fully sequential.
    #[test]
    fn extremes_are_present() {
        for n in 1..=8 {
            let got = generate(n);
            let nested = format!("{}{}", "(".repeat(n as usize), ")".repeat(n as usize));
            let flat = "()".repeat(n as usize);
            assert!(got.contains(&nested), "missing {nested}");
            assert!(got.contains(&flat), "missing {flat}");
        }
    }

    /// The largest allowed input.
    #[test]
    fn maximum_n() {
        let got = generate(8);
        assert_eq!(got.len(), 1430);
        assert_eq!(got[0], "(((((((())))))))");
        assert_eq!(got[got.len() - 1], "()()()()()()()()");
    }
}
