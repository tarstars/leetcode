#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for s in ["(()", ")()())", ""] {
        let answer = Solution::longest_valid_parentheses(s.to_string());
        println!("{s:?} -> {answer}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(s: &str) -> i32 {
        let bytes = s.as_bytes();
        let mut longest = 0;

        for start in 0..bytes.len() {
            let mut balance = 0;

            for (end, &parenthesis) in bytes.iter().enumerate().skip(start) {
                balance += if parenthesis == b'(' { 1 } else { -1 };

                if balance < 0 {
                    break;
                }
                if balance == 0 {
                    longest = longest.max(end + 1 - start);
                }
            }
        }

        longest as i32
    }

    #[test]
    fn example_1() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::longest_valid_parentheses(String::new()), 0);
    }

    #[test]
    fn handles_fully_valid_strings() {
        assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("(())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("()(())".to_string()), 6);
    }

    #[test]
    fn ignores_unmatched_parentheses() {
        assert_eq!(Solution::longest_valid_parentheses("(((".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses(")))".to_string()), 0);
        assert_eq!(
            Solution::longest_valid_parentheses("())(())(".to_string()),
            4
        );
    }

    #[test]
    fn joins_adjacent_valid_groups() {
        assert_eq!(
            Solution::longest_valid_parentheses("()(()())".to_string()),
            8
        );
        assert_eq!(
            Solution::longest_valid_parentheses(")()()(()".to_string()),
            4
        );
    }

    #[test]
    fn matches_reference_for_all_small_inputs() {
        for length in 0..=12 {
            for mask in 0..1_usize << length {
                let s: String = (0..length)
                    .map(|index| if mask & (1 << index) == 0 { '(' } else { ')' })
                    .collect();

                assert_eq!(
                    Solution::longest_valid_parentheses(s.clone()),
                    reference(&s),
                    "s: {s:?}"
                );
            }
        }
    }

    #[test]
    fn handles_the_maximum_input_length() {
        let s = format!("{}{}", "(".repeat(15_000), ")".repeat(15_000));
        assert_eq!(Solution::longest_valid_parentheses(s), 30_000);
    }
}
