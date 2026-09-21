use super::*;
use std::collections::HashSet;

/// Builds every decoded letter string, rather than counting splits a second
/// time. Constructive where the solution is a recurrence, so the two share no
/// machinery — and the set doubles as proof that distinct groupings really do
/// give distinct messages. Cost is proportional to the answer, so this is for
/// short inputs only.
fn all_decodings(digits: &[u8]) -> Vec<String> {
    if digits.is_empty() {
        return vec![String::new()];
    }
    if digits[0] == b'0' {
        return Vec::new();
    }

    let mut out = Vec::new();

    let single = (b'A' + digits[0] - b'1') as char;
    for rest in all_decodings(&digits[1..]) {
        out.push(format!("{single}{rest}"));
    }

    if digits.len() >= 2 {
        let pair = (digits[0] - b'0') * 10 + (digits[1] - b'0');
        if (10..=26).contains(&pair) {
            let letter = (b'A' + pair - 1) as char;
            for rest in all_decodings(&digits[2..]) {
                out.push(format!("{letter}{rest}"));
            }
        }
    }

    out
}

fn reference(s: &str) -> i32 {
    let decodings = all_decodings(s.as_bytes());
    let distinct: HashSet<&String> = decodings.iter().collect();
    assert_eq!(
        distinct.len(),
        decodings.len(),
        "{s:?} produced the same message twice — the oracle is wrong"
    );
    decodings.len() as i32
}

fn ways(s: &str) -> i32 {
    Solution::num_decodings(s.to_string())
}

fn check(s: &str) {
    assert_eq!(ways(s), reference(s), "for {s:?}");
}

/// Deterministic generation, so any failure reproduces exactly.
struct Rng(u64);

impl Rng {
    fn next_value(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, bound: u64) -> usize {
        (self.next_value() % bound) as usize
    }

    /// `alphabet` restricts which digits appear, so a run can be made zero-rich
    /// or zero-free on purpose.
    fn digits(&mut self, len: usize, alphabet: &[u8]) -> String {
        (0..len)
            .map(|_| alphabet[self.below(alphabet.len() as u64)] as char)
            .collect()
    }
}

#[test]
fn example_1() {
    assert_eq!(ways("12"), 2);
}

#[test]
fn example_2() {
    assert_eq!(ways("226"), 3);
}

#[test]
fn example_3() {
    assert_eq!(ways("06"), 0);
}

/// The statement's worked example, which mixes a pair, a zero and a tail.
#[test]
fn the_worked_example() {
    assert_eq!(ways("11106"), 2);
}

/// Every single digit: only '0' is undecodable.
#[test]
fn single_digits() {
    assert_eq!(ways("0"), 0);
    for digit in '1'..='9' {
        assert_eq!(ways(&digit.to_string()), 1, "for {digit:?}");
    }
}

/// All one hundred two-digit strings.
#[test]
fn every_two_digit_string() {
    for high in b'0'..=b'9' {
        for low in b'0'..=b'9' {
            let s = String::from_utf8(vec![high, low]).unwrap();
            check(&s);
        }
    }
}

/// A leading zero kills the whole string, however long it is.
#[test]
fn a_leading_zero_decodes_nothing() {
    assert_eq!(ways("0"), 0);
    assert_eq!(ways("01"), 0);
    assert_eq!(ways("012345"), 0);
    assert_eq!(ways(&"0".repeat(100)), 0);
    // The tail must stay countable: 99 ones would make every suffix count
    // overflow i32 on the way to the answer, even though the answer is 0.
    assert_eq!(ways(&format!("0{}", "3".repeat(99))), 0);
}

/// Zeros in the middle: only "10" and "20" can absorb one.
#[test]
fn zeros_must_be_absorbed_by_a_pair() {
    assert_eq!(ways("10"), 1);
    assert_eq!(ways("20"), 1);
    assert_eq!(ways("30"), 0);
    assert_eq!(ways("100"), 0);
    assert_eq!(ways("101"), 1);
    assert_eq!(ways("110"), 1);
    assert_eq!(ways("2101"), 1);
    assert_eq!(ways("1001"), 0);
}

/// The two-digit code is valid exactly on 10..=26.
#[test]
fn the_pair_boundary() {
    assert_eq!(ways("26"), 2);
    assert_eq!(ways("27"), 1);
    assert_eq!(ways("19"), 2);
    assert_eq!(ways("99"), 1);
    check("2626262626");
    check("2727272727");
}

/// `n` ones decode `Fib(n + 1)` ways. Checked up to the largest `n` whose
/// answer still fits in `i32`, which is where the problem's guarantee sits.
#[test]
fn runs_of_ones_are_fibonacci() {
    let (mut previous, mut current) = (1i64, 1i64);
    for n in 1..=45usize {
        assert!(current <= i32::MAX as i64, "n = {n} overflows i32");
        assert_eq!(
            ways(&"1".repeat(n)),
            current as i32,
            "{n} ones should decode {current} ways"
        );
        (previous, current) = (current, previous + current);
    }
}

/// Every string of length up to 6 over a digit set chosen to hit all the
/// interesting cases: a zero, both pair-leading digits, the 26/27 boundary and
/// an unpairable digit. 19,530 strings.
#[test]
fn every_short_string() {
    const ALPHABET: [u8; 5] = [b'0', b'1', b'2', b'6', b'7'];

    let mut checked = 0;
    for len in 1..=6usize {
        for mut index in 0..5u32.pow(len as u32) {
            let s: String = (0..len)
                .map(|_| {
                    let digit = ALPHABET[(index % 5) as usize] as char;
                    index /= 5;
                    digit
                })
                .collect();
            check(&s);
            checked += 1;
        }
    }
    assert_eq!(checked, 19_530, "the sweep changed size");
}

/// Random strings the constructive oracle can still afford, across digit sets
/// that make zeros common, rare or absent.
#[test]
fn matches_reference_on_random_strings() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for len in 1..=14usize {
        for alphabet in [
            &b"0123456789"[..],
            &b"012"[..],
            &b"120"[..],
            &b"123456789"[..],
            &b"3456789"[..],
        ] {
            for _ in 0..8 {
                check(&rng.digits(len, alphabet));
            }
        }
    }
}

/// Strings of length 100, in shapes whose answer is known without an oracle.
#[test]
fn the_largest_inputs() {
    // No pair is ever valid (33 > 26), so every digit stands alone: one way.
    assert_eq!(ways(&"3".repeat(100)), 1);
    assert_eq!(ways(&"7".repeat(100)), 1);

    // Each "10" must be taken whole — splitting strands a bare '0'.
    assert_eq!(ways(&"10".repeat(50)), 1);
    assert_eq!(ways(&"20".repeat(50)), 1);

    // "27" is not a code, so this is a hundred lone digits.
    assert_eq!(ways(&"27".repeat(50)), 1);

    // A zero nothing can absorb: it follows a '3', and "30" is not a code.
    assert_eq!(ways(&format!("{}0{}", "3".repeat(50), "3".repeat(49))), 0);
}
