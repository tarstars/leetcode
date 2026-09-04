use super::*;

/// A direct transcription of the grammar in the statement, used to check the
/// solution over every short string. It splits on `e`/`E` and validates the two
/// halves separately, which is a different shape from the single left-to-right
/// scan (or state machine) a solution usually takes.
mod grammar {
    fn strip_sign(b: &[u8]) -> &[u8] {
        match b.first() {
            Some(b'+') | Some(b'-') => &b[1..],
            _ => b,
        }
    }

    /// `[sign] digits`
    fn is_integer(b: &[u8]) -> bool {
        let b = strip_sign(b);
        !b.is_empty() && b.iter().all(u8::is_ascii_digit)
    }

    /// `[sign] ( digits "." | digits "." digits | "." digits )`
    fn is_decimal(b: &[u8]) -> bool {
        let b = strip_sign(b);

        match b.iter().position(|&c| c == b'.') {
            None => false,
            Some(dot) => {
                let (head, tail) = (&b[..dot], &b[dot + 1..]);

                head.iter().all(u8::is_ascii_digit)
                    && tail.iter().all(u8::is_ascii_digit)
                    && !(head.is_empty() && tail.is_empty())
            }
        }
    }

    /// `( decimal | integer ) [ ("e" | "E") integer ]`
    pub fn is_number(s: &str) -> bool {
        let b = s.as_bytes();

        match b.iter().position(|&c| c == b'e' || c == b'E') {
            Some(i) => (is_decimal(&b[..i]) || is_integer(&b[..i])) && is_integer(&b[i + 1..]),
            None => is_decimal(b) || is_integer(b),
        }
    }
}

fn is_number(s: &str) -> bool {
    Solution::is_number(s.to_owned())
}

fn assert_valid(cases: &[&str]) {
    for s in cases {
        assert!(is_number(s), "{s:?} should be a valid number");
    }
}

fn assert_invalid(cases: &[&str]) {
    for s in cases {
        assert!(!is_number(s), "{s:?} should not be a valid number");
    }
}

/// Every string of the given length over the alphabet, in order.
fn all_strings(alphabet: &[u8], len: usize) -> impl Iterator<Item = String> + '_ {
    let base = alphabet.len() as u64;

    (0..base.pow(len as u32)).map(move |mut code| {
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            bytes.push(alphabet[(code % base) as usize]);
            code /= base;
        }
        String::from_utf8(bytes).expect("the alphabet is ASCII")
    })
}

#[test]
fn example_1() {
    assert!(is_number("0"));
}

#[test]
fn example_2() {
    assert!(!is_number("e"));
}

#[test]
fn example_3() {
    assert!(!is_number("."));
}

/// The valid list quoted in the statement.
#[test]
fn valid_examples_from_the_statement() {
    assert_valid(&[
        "2",
        "0089",
        "-0.1",
        "+3.14",
        "4.",
        "-.9",
        "2e10",
        "-90E3",
        "3e+7",
        "+6e-1",
        "53.5e93",
        "-123.456e789",
    ]);
}

/// The invalid list quoted in the statement.
#[test]
fn invalid_examples_from_the_statement() {
    assert_invalid(&["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]);
}

/// A bare sign is never a number, and only one sign may lead each part.
#[test]
fn signs() {
    assert_valid(&[
        "+1", "-1", "+1.5", "-1.5", "+.5", "-.5", "+5.", "1e+1", "1e-1",
    ]);
    assert_invalid(&[
        "+", "-", "++1", "--1", "+-1", "-+1", "1+", "1-", "1e++1", "1e+",
    ]);
}

/// A lone dot is not a number: at least one digit must sit on one side of it.
#[test]
fn dots() {
    assert_valid(&[".1", "1.", "1.1", "+.1", "-1."]);
    assert_invalid(&[".", "+.", "-.", "..", "1..", "1.2.3", ".1.", "..1"]);
}

/// The exponent must be a plain integer — no dot, no second exponent, and the
/// mantissa may not be empty.
#[test]
fn exponents() {
    assert_valid(&[
        "1e1", "1E1", "1e01", "0e0", "1.e1", ".1e1", "1.1e1", "-1.1E-1",
    ]);
    assert_invalid(&[
        "e", "e1", "1e", "E", ".e1", "+e1", "1e1e1", "1e1.1", "1e.", "1e.1", "1ee1", "e.1", "1e+",
        "1e-",
    ]);
}

/// A decimal is allowed before the exponent, which is the case most easily
/// missed: "46.e3" is valid even though the mantissa ends in a dot.
#[test]
fn a_decimal_mantissa_may_end_in_a_dot() {
    assert_valid(&["46.e3", "4.e3", "0.e0", "-46.E-3"]);
}

/// Letters are only ever legal as the exponent marker.
#[test]
fn letters() {
    assert_invalid(&["a", "abc", "1a", "a1", "1a1", "0x1", "1d1", "1f", "12a34"]);
}

/// `s.parse::<f64>().is_ok()` looks like a one-line solution and agrees with
/// the grammar on every string of digits and punctuation — but Rust's float
/// parser also accepts these, and the problem does not.
#[test]
fn float_parser_lookalikes_are_rejected() {
    assert_invalid(&[
        "inf",
        "Inf",
        "infinity",
        "Infinity",
        "-inf",
        "+infinity",
        "NaN",
        "nan",
        "-NaN",
    ]);
}

/// Leading zeros carry no meaning here, and long runs of digits are fine.
#[test]
fn leading_zeros_and_long_digit_runs() {
    assert_valid(&[
        "0089",
        "00000000000000000000",
        "0.00000000000000000",
        "1e00000000000000000",
    ]);
}

/// The statement's length ceiling, valid and invalid at exactly 20 characters.
#[test]
fn longest_allowed_input() {
    assert_valid(&["-123.4567890123e-456", "12345678901234567890"]);
    assert_invalid(&["-123.4567890123e-45.", "1234567890123456789a"]);
}

/// The reference must reproduce the statement's own examples before it is
/// trusted as an oracle for the exhaustive sweep below.
#[test]
fn the_reference_agrees_with_the_statement() {
    for s in [
        "2",
        "0089",
        "-0.1",
        "+3.14",
        "4.",
        "-.9",
        "2e10",
        "-90E3",
        "3e+7",
        "+6e-1",
        "53.5e93",
        "-123.456e789",
        "0",
        "46.e3",
    ] {
        assert!(grammar::is_number(s), "reference rejects valid {s:?}");
    }

    for s in [
        "abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53", "e", ".", "+.", ".e1",
    ] {
        assert!(!grammar::is_number(s), "reference accepts invalid {s:?}");
    }
}

/// Exhaustive: every string up to length 4 over a alphabet holding two digits,
/// both exponent markers, both signs, a dot and a stray letter — 4680 cases,
/// checked against the grammar reference.
#[test]
fn matches_reference_on_every_short_string() {
    let alphabet = [b'0', b'9', b'.', b'e', b'E', b'+', b'-', b'a'];

    for len in 1..=4usize {
        for s in all_strings(&alphabet, len) {
            assert_eq!(
                is_number(&s),
                grammar::is_number(&s),
                "disagreement on {s:?}"
            );
        }
    }
}

/// The same sweep at length 5 over a smaller alphabet, to reach patterns that
/// need five characters ("1.2e3", "+1e-1", ".1e+1").
#[test]
fn matches_reference_on_every_five_character_string() {
    let alphabet = [b'0', b'.', b'e', b'+', b'-', b'a'];

    for s in all_strings(&alphabet, 5) {
        assert_eq!(
            is_number(&s),
            grammar::is_number(&s),
            "disagreement on {s:?}"
        );
    }
}
