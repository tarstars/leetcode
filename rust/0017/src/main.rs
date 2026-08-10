#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for d in ["23", "", "2"] {
        println!("{d:?} -> {:?}", Solution::letter_combinations(d.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The problem allows any order, so both sides are sorted before comparing.
    fn combos(digits: &str) -> Vec<String> {
        let mut got = Solution::letter_combinations(digits.to_string());
        got.sort();
        got
    }

    fn expect(digits: &str, want: &[&str]) {
        let mut want: Vec<String> = want.iter().map(|s| s.to_string()).collect();
        want.sort();
        assert_eq!(combos(digits), want, "for digits {digits:?}");
    }

    #[test]
    fn example_1() {
        expect("23", &["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
    }

    /// An empty input yields no combinations at all — not one empty string.
    #[test]
    fn example_2_empty_input() {
        expect("", &[]);
    }

    #[test]
    fn example_3_single_digit() {
        expect("2", &["a", "b", "c"]);
    }

    /// 7 and 9 carry four letters each, unlike every other digit.
    #[test]
    fn four_letter_digits() {
        expect("7", &["p", "q", "r", "s"]);
        expect("9", &["w", "x", "y", "z"]);
    }

    #[test]
    fn two_four_letter_digits() {
        assert_eq!(combos("79").len(), 16);
        expect(
            "79",
            &[
                "pw", "px", "py", "pz", "qw", "qx", "qy", "qz", "rw", "rx", "ry", "rz", "sw", "sx",
                "sy", "sz",
            ],
        );
    }

    #[test]
    fn three_digits() {
        let got = combos("234");
        assert_eq!(got.len(), 27);
        assert_eq!(got[0], "adg");
        assert_eq!(got[got.len() - 1], "cfi");
    }

    /// A repeated digit must still expand independently at each position.
    #[test]
    fn repeated_digit() {
        let got = combos("2222");
        assert_eq!(got.len(), 81);
        assert_eq!(got[0], "aaaa");
        assert_eq!(got[got.len() - 1], "cccc");
    }

    /// The largest possible answer: four digits of four letters each.
    #[test]
    fn maximum_size_answer() {
        let got = combos("7979");
        assert_eq!(got.len(), 256);
        assert_eq!(got[0], "pwpw");
        assert_eq!(got[got.len() - 1], "szsz");

        for c in &got {
            assert_eq!(c.len(), 4, "{c} has the wrong length");
        }

        let unique: std::collections::HashSet<&String> = got.iter().collect();
        assert_eq!(unique.len(), got.len(), "duplicates in the answer");
    }
}
