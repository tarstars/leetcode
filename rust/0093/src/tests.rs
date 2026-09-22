use super::*;

/// Is this one of the four parts of a valid address?
fn valid_part(part: &str) -> bool {
    !part.is_empty()
        && part.len() <= 3
        && (part.len() == 1 || !part.starts_with('0'))
        && part.parse::<u32>().is_ok_and(|value| value <= 255)
}

/// Every choice of three cut positions, validated. A flat triple loop rather
/// than a recursion, so it shares nothing with a backtracking solution.
fn reference(s: &str) -> Vec<String> {
    let n = s.len();
    let mut out = Vec::new();

    for first in 1..n {
        for second in first + 1..n {
            for third in second + 1..n {
                let parts = [
                    &s[..first],
                    &s[first..second],
                    &s[second..third],
                    &s[third..],
                ];
                if parts.iter().all(|part| valid_part(part)) {
                    out.push(parts.join("."));
                }
            }
        }
    }

    out.sort();
    out
}

/// The output order is unspecified, so both sides are sorted before comparing.
fn addresses(s: &str) -> Vec<String> {
    let mut got = Solution::restore_ip_addresses(s.to_string());
    got.sort();
    got
}

fn check(s: &str) {
    assert_eq!(addresses(s), reference(s), "for {s:?}");
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

    fn digits(&mut self, len: usize, alphabet: &[u8]) -> String {
        (0..len)
            .map(|_| alphabet[self.below(alphabet.len() as u64)] as char)
            .collect()
    }
}

#[test]
fn example_1() {
    assert_eq!(
        addresses("25525511135"),
        vec!["255.255.11.135", "255.255.111.35"]
    );
}

#[test]
fn example_2() {
    assert_eq!(addresses("0000"), vec!["0.0.0.0"]);
}

#[test]
fn example_3() {
    assert_eq!(
        addresses("101023"),
        vec![
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3"
        ]
    );
}

/// Fewer than four digits cannot be cut into four parts, and more than twelve
/// cannot fit — but the input may be up to twenty characters long.
#[test]
fn lengths_outside_four_to_twelve_have_no_answers() {
    for len in 1..4usize {
        assert!(addresses(&"1".repeat(len)).is_empty(), "length {len}");
    }
    for len in 13..=20usize {
        assert!(addresses(&"1".repeat(len)).is_empty(), "length {len}");
    }
    assert!(addresses(&"9".repeat(20)).is_empty());
}

/// The two extreme addresses: all zeros and all 255s.
#[test]
fn the_boundary_addresses() {
    assert_eq!(addresses("0000"), vec!["0.0.0.0"]);
    assert_eq!(addresses("255255255255"), vec!["255.255.255.255"]);
    assert!(addresses("256256256256").is_empty());
}

/// A part may be `"0"` but never `"00"` or `"01"`.
#[test]
fn leading_zeros_are_rejected() {
    check("010010");
    check("00000");
    check("0100");
    check("1010");
    assert!(addresses("00000000").is_empty());
}

/// 255 is in, 256 is out, at each of the four positions.
#[test]
fn the_value_boundary() {
    check("2552552552");
    check("2562552552");
    check("1111111111");
    check("2555255525");
}

/// Every string of length 4 to 6 over a digit set covering zero, small values
/// and the 255/256 boundary — 19,375 strings.
#[test]
fn every_short_string() {
    const ALPHABET: [u8; 5] = *b"01259";

    let mut checked = 0;
    for len in 4..=6usize {
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
    assert_eq!(checked, 19_375, "the sweep changed size");
}

/// Random strings across every permitted length and several digit sets.
#[test]
fn matches_reference_on_random_strings() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for len in 1..=20usize {
        for alphabet in [
            &b"0123456789"[..],
            &b"012"[..],
            &b"0"[..],
            &b"25"[..],
            &b"6789"[..],
        ] {
            for _ in 0..8 {
                check(&rng.digits(len, alphabet));
            }
        }
    }
}

/// Every returned address must be well formed and must rebuild the input
/// exactly — a check that does not consult the oracle at all.
#[test]
fn every_answer_is_a_valid_address_of_the_input() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);

    for _ in 0..300 {
        let len = 4 + rng.below(9);
        let s = rng.digits(len, b"01259");

        for address in addresses(&s) {
            let parts: Vec<&str> = address.split('.').collect();
            assert_eq!(parts.len(), 4, "{address:?} does not have four parts");
            for part in &parts {
                assert!(valid_part(part), "{address:?} has a bad part {part:?}");
            }
            assert_eq!(parts.concat(), s, "{address:?} does not rebuild {s:?}");
        }
    }
}

/// No address may be produced twice.
#[test]
fn the_answers_are_distinct() {
    let mut rng = Rng(0xF00D_FACE_8BAD_F00D);

    for _ in 0..300 {
        let len = 4 + rng.below(9);
        let s = rng.digits(len, b"0125");
        let got = addresses(&s);

        let mut sorted = got.clone();
        sorted.dedup();
        assert_eq!(sorted.len(), got.len(), "{s:?} produced a duplicate");
    }
}

/// Length 12 is the only length where all four parts must be three digits, so
/// every cut position is forced.
#[test]
fn the_longest_solvable_length() {
    check("255255255255");
    check("111111111111");
    check("100200300400");
    check("999999999999");
    assert!(addresses("999999999999").is_empty());
}
