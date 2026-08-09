// Two solutions share one test suite. `cargo test` runs sol_00 (greedy table);
// `cargo test --features sol_01` runs sol_01 (place-value tables). They can't
// both be compiled at once, since each defines Solution::int_to_roman.
#[cfg(not(feature = "sol_01"))]
#[path = "../sol_00.rs"]
mod sol_00;

#[cfg(feature = "sol_01")]
#[path = "../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for n in [3749, 58, 1994, 3999] {
        println!("{n} -> {}", Solution::int_to_roman(n));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roman(n: i32) -> String {
        Solution::int_to_roman(n)
    }

    /// Parses a Roman numeral back to an integer (this is problem 13, used here
    /// only as an independent check that the conversion round-trips).
    fn parse_roman(s: &str) -> i32 {
        let value = |c: u8| match c {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => panic!("not a Roman digit: {}", c as char),
        };
        let b = s.as_bytes();
        let mut total = 0;
        for i in 0..b.len() {
            let v = value(b[i]);
            if i + 1 < b.len() && v < value(b[i + 1]) {
                total -= v;
            } else {
                total += v;
            }
        }
        total
    }

    #[test]
    fn example_1() {
        assert_eq!(roman(3749), "MMMDCCXLIX");
    }

    #[test]
    fn example_2() {
        assert_eq!(roman(58), "LVIII");
    }

    #[test]
    fn example_3() {
        assert_eq!(roman(1994), "MCMXCIV");
    }

    #[test]
    fn smallest_value() {
        assert_eq!(roman(1), "I");
    }

    #[test]
    fn largest_value() {
        assert_eq!(roman(3999), "MMMCMXCIX");
    }

    #[test]
    fn repeated_symbols() {
        assert_eq!(roman(3), "III");
        assert_eq!(roman(2000), "MM");
    }

    /// Every subtractive form, each on its own.
    #[test]
    fn subtractive_forms() {
        assert_eq!(roman(4), "IV");
        assert_eq!(roman(9), "IX");
        assert_eq!(roman(40), "XL");
        assert_eq!(roman(90), "XC");
        assert_eq!(roman(400), "CD");
        assert_eq!(roman(900), "CM");
    }

    #[test]
    fn mixed_places() {
        assert_eq!(roman(14), "XIV");
        assert_eq!(roman(2024), "MMXXIV");
    }

    /// The longest numeral in range, at 15 characters.
    #[test]
    fn longest_numeral() {
        assert_eq!(roman(3888), "MMMDCCCLXXXVIII");
    }

    /// No symbol other than I, X, C may repeat more than three times, and
    /// V, L, D may not repeat at all.
    #[test]
    fn no_illegal_repetition() {
        for n in 1..=3999 {
            let r = roman(n);
            let b = r.as_bytes();
            for w in b.windows(4) {
                assert!(
                    w[0] != w[1] || w[1] != w[2] || w[2] != w[3],
                    "{n} -> {r} repeats a symbol four times"
                );
            }
            for c in [b'V', b'L', b'D'] {
                assert!(
                    b.iter().filter(|&&x| x == c).count() <= 1,
                    "{n} -> {r} repeats {}",
                    c as char
                );
            }
        }
    }

    /// Exhaustive round-trip over the whole input range.
    #[test]
    fn round_trips_for_every_value_in_range() {
        for n in 1..=3999 {
            let r = roman(n);
            assert_eq!(parse_roman(&r), n, "{n} -> {r} parsed back wrong");
        }
    }
}
