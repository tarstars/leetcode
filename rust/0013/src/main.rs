#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for s in ["III", "LVIII", "MCMXCIV", "MMMCMXCIX"] {
        println!("{s} -> {}", Solution::roman_to_int(s.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(s: &str) -> i32 {
        Solution::roman_to_int(s.to_string())
    }

    /// Builds the numeral for `n` from place-value tables. This is problem 12,
    /// used here only to generate every valid input for the exhaustive test.
    fn to_roman(n: usize) -> String {
        const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];
        const HUNDREDS: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        const TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

        format!(
            "{}{}{}{}",
            THOUSANDS[n / 1000],
            HUNDREDS[n / 100 % 10],
            TENS[n / 10 % 10],
            ONES[n % 10]
        )
    }

    #[test]
    fn example_1() {
        assert_eq!(value("III"), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(value("LVIII"), 58);
    }

    #[test]
    fn example_3() {
        assert_eq!(value("MCMXCIV"), 1994);
    }

    #[test]
    fn single_symbols() {
        assert_eq!(value("I"), 1);
        assert_eq!(value("V"), 5);
        assert_eq!(value("X"), 10);
        assert_eq!(value("L"), 50);
        assert_eq!(value("C"), 100);
        assert_eq!(value("D"), 500);
        assert_eq!(value("M"), 1000);
    }

    /// All six subtractive forms.
    #[test]
    fn subtractive_forms() {
        assert_eq!(value("IV"), 4);
        assert_eq!(value("IX"), 9);
        assert_eq!(value("XL"), 40);
        assert_eq!(value("XC"), 90);
        assert_eq!(value("CD"), 400);
        assert_eq!(value("CM"), 900);
    }

    /// A repeated symbol after a subtractive pair: the IV must not swallow the X.
    #[test]
    fn subtractive_pair_followed_by_more() {
        assert_eq!(value("XIV"), 14);
        assert_eq!(value("MMXXIV"), 2024);
    }

    #[test]
    fn largest_value() {
        assert_eq!(value("MMMCMXCIX"), 3999);
    }

    #[test]
    fn longest_numeral() {
        assert_eq!(value("MMMDCCCLXXXVIII"), 3888);
    }

    #[test]
    fn descending_only_no_subtraction() {
        assert_eq!(value("MDCLXVI"), 1666);
    }

    /// Every valid numeral in range, generated from the inverse mapping.
    #[test]
    fn every_numeral_in_range() {
        for n in 1..=3999usize {
            let r = to_roman(n);
            assert_eq!(value(&r), n as i32, "{r} should be {n}");
        }
    }
}
