use super::*;

/// A bitwise adder: `sum = a XOR b` and `carry = (a AND b) << 1`, repeated
/// until the carry dies. No per-digit carry variable anywhere, so it is a
/// genuinely different formulation from the usual right-to-left walk. It is
/// quadratic when a carry ripples the whole way, so it is only used on short
/// and medium inputs.
fn reference(a: &str, b: &str) -> String {
    let width = a.len().max(b.len()) + 1;

    // Least significant bit first.
    let mut x: Vec<u8> = a.bytes().rev().map(|c| c - b'0').collect();
    let mut y: Vec<u8> = b.bytes().rev().map(|c| c - b'0').collect();
    x.resize(width, 0);
    y.resize(width, 0);

    while y.iter().any(|&bit| bit == 1) {
        let sum: Vec<u8> = x.iter().zip(&y).map(|(&p, &q)| p ^ q).collect();
        let carry: Vec<u8> = std::iter::once(0)
            .chain(x.iter().zip(&y).map(|(&p, &q)| p & q))
            .take(width)
            .collect();
        x = sum;
        y = carry;
    }

    while x.len() > 1 && *x.last().expect("non-empty") == 0 {
        x.pop();
    }

    x.iter().rev().map(|&bit| (bit + b'0') as char).collect()
}

fn add(a: &str, b: &str) -> String {
    Solution::add_binary(a.to_owned(), b.to_owned())
}

/// Every binary string of the given length the constraints permit: no leading
/// zero, unless the whole string is "0".
fn every_legal_input(len: usize) -> Vec<String> {
    if len == 1 {
        return vec!["0".to_owned(), "1".to_owned()];
    }

    (0..1u32 << (len - 1))
        .map(|code| {
            let tail: String = (0..len - 1)
                .rev()
                .map(|i| if code >> i & 1 == 1 { '1' } else { '0' })
                .collect();
            format!("1{tail}")
        })
        .collect()
}

/// `"1"` repeated, i.e. 2^n - 1.
fn ones(n: usize) -> String {
    "1".repeat(n)
}

/// `"1"` followed by n zeros, i.e. 2^n.
fn power_of_two(n: usize) -> String {
    format!("1{}", "0".repeat(n))
}

#[test]
fn example_1() {
    assert_eq!(add("11", "1"), "100");
}

#[test]
fn example_2() {
    assert_eq!(add("1010", "1011"), "10101");
}

/// Zero is the one string allowed to start with a zero, on either side.
#[test]
fn zero_is_the_identity() {
    assert_eq!(add("0", "0"), "0");
    assert_eq!(add("0", "1"), "1");
    assert_eq!(add("1", "0"), "1");

    for len in 1..=8 {
        for s in every_legal_input(len) {
            assert_eq!(add(&s, "0"), s, "{s} + 0");
            assert_eq!(add("0", &s), s, "0 + {s}");
        }
    }
}

/// The operands may be different lengths, in either order.
#[test]
fn operands_of_different_lengths() {
    assert_eq!(add("1", "1111"), "10000");
    assert_eq!(add("1111", "1"), "10000");
    assert_eq!(add("1", "1000"), "1001");
    assert_eq!(add("1000", "1"), "1001");
    assert_eq!(add("101", "10000000"), "10000101");
}

/// A run of ones is 2^n - 1, so adding one carries the whole way and yields a
/// power of two — the case where the result is longer than either operand.
#[test]
fn carry_ripples_the_whole_way() {
    for n in 1..=64 {
        assert_eq!(add(&ones(n), "1"), power_of_two(n), "{n} ones + 1");
    }
}

/// Doubling a number shifts it left by one, which in string form just appends
/// a zero. Holds for every value except zero itself.
#[test]
fn adding_a_number_to_itself_appends_a_zero() {
    for len in 1..=8 {
        for s in every_legal_input(len) {
            let expected = if s == "0" {
                "0".to_owned()
            } else {
                format!("{s}0")
            };
            assert_eq!(add(&s, &s), expected, "{s} + {s}");
        }
    }
}

/// The reference must agree with real arithmetic before it is trusted as an
/// oracle for the sweeps below.
#[test]
fn the_reference_agrees_with_integer_arithmetic() {
    for a_len in 1..=6usize {
        for b_len in 1..=6usize {
            for a in every_legal_input(a_len) {
                for b in every_legal_input(b_len) {
                    let x = u128::from_str_radix(&a, 2).expect("binary");
                    let y = u128::from_str_radix(&b, 2).expect("binary");
                    let expected = format!("{:b}", x + y);

                    assert_eq!(reference(&a, &b), expected, "{a} + {b}");
                }
            }
        }
    }
}

/// Exhaustive against real arithmetic: every legal pair up to six bits each.
#[test]
fn matches_integer_arithmetic_on_every_short_pair() {
    for a_len in 1..=6usize {
        for b_len in 1..=6usize {
            for a in every_legal_input(a_len) {
                for b in every_legal_input(b_len) {
                    let x = u128::from_str_radix(&a, 2).expect("binary");
                    let y = u128::from_str_radix(&b, 2).expect("binary");

                    assert_eq!(add(&a, &b), format!("{:b}", x + y), "{a} + {b}");
                }
            }
        }
    }
}

/// Addition is commutative, checked on medium-length pseudo-random inputs that
/// are well past what u128 can hold.
#[test]
fn addition_is_commutative() {
    let mut state = 0x243F_6A88_85A3_08D3u64;
    let mut next_bit = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state & 1 == 1
    };
    let mut random_binary = |len: usize| -> String {
        let mut s = String::from("1");
        for _ in 1..len {
            s.push(if next_bit() { '1' } else { '0' });
        }
        s
    };

    for (a_len, b_len) in [(200, 200), (300, 17), (17, 300), (129, 128), (500, 499)] {
        let a = random_binary(a_len);
        let b = random_binary(b_len);

        let forwards = add(&a, &b);
        assert_eq!(forwards, add(&b, &a), "{a_len} + {b_len} not commutative");
        assert_eq!(forwards, reference(&a, &b), "{a_len} + {b_len}");
    }
}

/// The result never carries a leading zero, and its length is either the
/// longer operand's or one more.
#[test]
fn the_result_is_well_formed() {
    for a_len in 1..=5usize {
        for b_len in 1..=5usize {
            for a in every_legal_input(a_len) {
                for b in every_legal_input(b_len) {
                    let sum = add(&a, &b);
                    let longest = a.len().max(b.len());

                    assert!(!sum.is_empty(), "{a} + {b} produced an empty string");
                    assert!(
                        sum.bytes().all(|c| c == b'0' || c == b'1'),
                        "{a} + {b} -> {sum} is not binary"
                    );
                    assert!(
                        !sum.starts_with('0') || sum == "0",
                        "{a} + {b} -> {sum} has a leading zero"
                    );
                    assert!(
                        sum.len() == longest || sum.len() == longest + 1,
                        "{a} + {b} -> {sum} has an implausible length"
                    );
                }
            }
        }
    }
}

/// 10^4 bits is the constraint's maximum — roughly 3000 decimal digits, and far
/// beyond every built-in integer type, so parsing the strings into a number
/// cannot work. The carry also ripples the entire length here.
#[test]
fn longest_allowed_input() {
    const N: usize = 10_000;

    assert_eq!(add(&ones(N), "1"), power_of_two(N));
    assert_eq!(add("1", &ones(N)), power_of_two(N));
    assert_eq!(add(&ones(N), &ones(N)), format!("{}0", ones(N)));
    assert_eq!(
        add(&power_of_two(N - 1), &power_of_two(N - 1)),
        power_of_two(N)
    );
    assert_eq!(add(&ones(N), "0"), ones(N));
}
