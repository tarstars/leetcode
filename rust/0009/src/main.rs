#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for x in [121, -121, 10, 0, 1000000001, i32::MAX] {
        println!("{x} -> {}", Solution::is_palindrome(x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_true() {
        assert!(Solution::is_palindrome(121));
    }

    /// The minus sign has no mirror, so no negative number qualifies.
    #[test]
    fn example_2_negative_is_never_a_palindrome() {
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn example_3_trailing_zero() {
        assert!(!Solution::is_palindrome(10));
    }

    #[test]
    fn zero_is_a_palindrome() {
        assert!(Solution::is_palindrome(0));
    }

    #[test]
    fn every_single_digit_is_a_palindrome() {
        for d in 0..=9 {
            assert!(Solution::is_palindrome(d), "{d} should be a palindrome");
        }
    }

    #[test]
    fn two_equal_digits() {
        assert!(Solution::is_palindrome(11));
    }

    #[test]
    fn two_different_digits() {
        assert!(!Solution::is_palindrome(12));
    }

    /// -0 is just 0, but a sign-only check that looks at `x < 0` must not trip.
    #[test]
    fn negative_single_digit() {
        assert!(!Solution::is_palindrome(-1));
    }

    #[test]
    fn odd_length_palindrome() {
        assert!(Solution::is_palindrome(12321));
    }

    #[test]
    fn even_length_palindrome() {
        assert!(Solution::is_palindrome(1234554321));
    }

    #[test]
    fn interior_zeros() {
        assert!(Solution::is_palindrome(1000000001));
    }

    #[test]
    fn nearly_a_palindrome() {
        assert!(!Solution::is_palindrome(1000000011));
    }

    /// Ten digits, just under i32::MAX — a full-width reversal would overflow.
    #[test]
    fn largest_ten_digit_palindrome_in_range() {
        assert!(Solution::is_palindrome(2147447412));
    }

    #[test]
    fn i32_max_is_not_a_palindrome() {
        assert!(!Solution::is_palindrome(i32::MAX));
    }

    #[test]
    fn i32_min_is_not_a_palindrome() {
        assert!(!Solution::is_palindrome(i32::MIN));
    }
}
