#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for s in ["42", "   -042", "1337c0d3", "0-1", "words and 987", "-91283472332"] {
        println!("{s:?} -> {}", Solution::my_atoi(s.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn atoi(s: &str) -> i32 {
        Solution::my_atoi(s.to_string())
    }

    #[test]
    fn example_1_plain_number() {
        assert_eq!(atoi("42"), 42);
    }

    #[test]
    fn example_2_spaces_sign_and_leading_zeros() {
        assert_eq!(atoi("   -042"), -42);
    }

    #[test]
    fn example_3_stops_at_first_letter() {
        assert_eq!(atoi("1337c0d3"), 1337);
    }

    #[test]
    fn example_4_stops_at_sign_after_digit() {
        assert_eq!(atoi("0-1"), 0);
    }

    #[test]
    fn example_5_leading_word_gives_zero() {
        assert_eq!(atoi("words and 987"), 0);
    }

    #[test]
    fn empty_string() {
        assert_eq!(atoi(""), 0);
    }

    #[test]
    fn only_spaces() {
        assert_eq!(atoi("     "), 0);
    }

    #[test]
    fn only_a_sign() {
        assert_eq!(atoi("-"), 0);
    }

    /// The sign may appear once; a second one ends the number before it starts.
    #[test]
    fn two_signs_give_zero() {
        assert_eq!(atoi("+-12"), 0);
    }

    #[test]
    fn explicit_plus() {
        assert_eq!(atoi("+1"), 1);
    }

    #[test]
    fn plus_after_digits_stops_the_number() {
        assert_eq!(atoi("12+3"), 12);
    }

    #[test]
    fn doubled_plus_gives_zero() {
        assert_eq!(atoi("++9"), 0);
    }

    #[test]
    fn minus_then_plus_gives_zero() {
        assert_eq!(atoi("-+3"), 0);
    }

    #[test]
    fn space_inside_the_number_stops_it() {
        assert_eq!(atoi("  +0 123"), 0);
    }

    #[test]
    fn decimal_point_stops_the_number() {
        assert_eq!(atoi("3.14159"), 3);
    }

    #[test]
    fn many_leading_zeros() {
        assert_eq!(atoi("0000000000012345678"), 12345678);
    }

    #[test]
    fn clamps_above_i32_max() {
        assert_eq!(atoi("91283472332"), i32::MAX);
    }

    #[test]
    fn clamps_below_i32_min() {
        assert_eq!(atoi("-91283472332"), i32::MIN);
    }

    /// The boundaries themselves must survive unclamped.
    #[test]
    fn exactly_i32_max() {
        assert_eq!(atoi("2147483647"), i32::MAX);
    }

    #[test]
    fn exactly_i32_min() {
        assert_eq!(atoi("-2147483648"), i32::MIN);
    }

    #[test]
    fn one_past_i32_max_clamps() {
        assert_eq!(atoi("2147483648"), i32::MAX);
    }

    #[test]
    fn one_past_i32_min_clamps() {
        assert_eq!(atoi("-2147483649"), i32::MIN);
    }

    /// 200 digits is the constraint's upper bound — no accumulator may overflow.
    #[test]
    fn very_long_digit_run_clamps() {
        assert_eq!(atoi(&"9".repeat(200)), i32::MAX);
    }
}
