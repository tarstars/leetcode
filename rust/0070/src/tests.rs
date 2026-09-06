use super::*;

/// Counts the climbs by walking every one of them. Exponential, so it is only
/// used for small n, but it follows the problem statement literally and shares
/// nothing with a recurrence.
fn brute_force(n: i32) -> u128 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 1;
    }
    brute_force(n - 1) + brute_force(n - 2)
}

/// A closed form instead of a recurrence: a climb using k double steps has
/// n - k steps in total, and choosing which of them are doubles gives
/// C(n - k, k). Summing over every possible k counts them all.
fn binomial_sum(n: i32) -> u128 {
    let n = n as u128;
    let mut total = 0u128;

    for k in 0..=n / 2 {
        let mut choose = 1u128;
        for i in 1..=k {
            choose = choose * (n - k - k + i) / i;
        }
        total += choose;
    }

    total
}

fn climb(n: i32) -> u128 {
    Solution::climb_stairs(n) as u128
}

#[test]
fn example_1() {
    assert_eq!(Solution::climb_stairs(2), 2);
}

#[test]
fn example_2() {
    assert_eq!(Solution::climb_stairs(3), 3);
}

/// One step admits exactly one climb — the smallest legal input.
#[test]
fn a_single_stair() {
    assert_eq!(Solution::climb_stairs(1), 1);
}

/// The sequence runs 1, 2, 3, 5, 8, ... — it starts at 1, 2, so an
/// implementation seeded with the textbook 1, 1 comes out shifted by one.
#[test]
fn the_first_ten_values() {
    let expected = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89];

    for (i, want) in expected.iter().enumerate() {
        assert_eq!(Solution::climb_stairs(i as i32 + 1), *want, "n = {}", i + 1);
    }
}

/// The two references must agree with each other before either is trusted.
#[test]
fn the_references_agree() {
    for n in 1..=24 {
        assert_eq!(brute_force(n), binomial_sum(n), "n = {n}");
    }
}

/// Against literal enumeration of every climb, for the n where that is
/// affordable.
#[test]
fn matches_brute_force_for_small_inputs() {
    for n in 1..=24 {
        assert_eq!(climb(n), brute_force(n), "n = {n}");
    }
}

/// Against the closed form, over the entire legal input range.
#[test]
fn matches_the_closed_form_over_the_whole_range() {
    for n in 1..=45 {
        assert_eq!(climb(n), binomial_sum(n), "n = {n}");
    }
}

/// The result must satisfy its own recurrence: reaching stair n means arriving
/// from n - 1 or from n - 2.
#[test]
fn obeys_the_recurrence() {
    for n in 3..=45 {
        assert_eq!(
            climb(n),
            climb(n - 1) + climb(n - 2),
            "n = {n} breaks f(n) = f(n-1) + f(n-2)"
        );
    }
}

/// 45 is the constraint's ceiling, and it is chosen so the answer still fits in
/// an i32: f(45) is 1_836_311_903 against a maximum of 2_147_483_647, while
/// f(46) would be 2_971_215_073 and overflow. A solution that computes one term
/// too many, or that recurses without memoising, fails here.
#[test]
fn largest_input() {
    assert_eq!(Solution::climb_stairs(45), 1_836_311_903);
    assert_eq!(Solution::climb_stairs(44), 1_134_903_170);
    assert_eq!(Solution::climb_stairs(43), 701_408_733);

    assert!(
        Solution::climb_stairs(45) > 0,
        "the answer overflowed into a negative"
    );
}

/// Every result is positive and strictly increasing.
#[test]
fn the_sequence_grows() {
    let mut previous = 0;

    for n in 1..=45 {
        let ways = Solution::climb_stairs(n);
        assert!(ways > 0, "n = {n} gave {ways}");
        assert!(ways > previous, "n = {n} gave {ways}, not above {previous}");
        previous = ways;
    }
}
