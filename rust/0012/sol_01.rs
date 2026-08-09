use crate::Solution;

/// Place-value tables: one entry per possible digit at each decimal position.
/// The input range is 1..=3999, so the thousands place only needs 0..=3.
const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];
const HUNDREDS: [&str; 10] = [
    "", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM",
];
const TENS: [&str; 10] = [
    "", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC",
];
const ONES: [&str; 10] = [
    "", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX",
];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let n = num as usize;
        let mut res = String::with_capacity(15);

        res.push_str(THOUSANDS[n / 1000]);
        res.push_str(HUNDREDS[n / 100 % 10]);
        res.push_str(TENS[n / 10 % 10]);
        res.push_str(ONES[n % 10]);

        res
    }
}
