#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for rows in 1..=4 {
        println!(
            "{rows} rows -> {:?}",
            Solution::convert("PAYPALISHIRING".to_string(), rows)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert(s: &str, num_rows: i32) -> String {
        Solution::convert(s.to_string(), num_rows)
    }

    #[test]
    fn example_1_three_rows() {
        assert_eq!(convert("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR");
    }

    #[test]
    fn example_2_four_rows() {
        assert_eq!(convert("PAYPALISHIRING", 4), "PINALSIGYAHRPI");
    }

    #[test]
    fn example_3_single_row_single_char() {
        assert_eq!(convert("A", 1), "A");
    }

    /// One row means no zigzag at all — the usual division-by-(2*rows-2) crash.
    #[test]
    fn one_row_is_the_identity() {
        assert_eq!(convert("PAYPALISHIRING", 1), "PAYPALISHIRING");
    }

    #[test]
    fn two_rows_alternate() {
        assert_eq!(convert("ABCD", 2), "ACBD");
    }

    #[test]
    fn two_rows_odd_length() {
        assert_eq!(convert("ABC", 2), "ACB");
    }

    #[test]
    fn more_rows_than_characters() {
        assert_eq!(convert("ABC", 5), "ABC");
    }

    #[test]
    fn stops_partway_down_the_first_column() {
        assert_eq!(convert("ABCDE", 4), "ABCED");
    }

    #[test]
    fn three_rows_plain_alphabet() {
        assert_eq!(convert("abcdefghij", 3), "aeibdfhjcg");
    }

    #[test]
    fn single_character_many_rows() {
        assert_eq!(convert("A", 100), "A");
    }
}
