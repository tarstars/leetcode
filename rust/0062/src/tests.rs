use super::*;

/// The answer is the binomial coefficient C(m + n - 2, m - 1): every path is a
/// fixed sequence of m-1 downs and n-1 rights, so it's a choice of which steps
/// are downs. Computed multiplicatively in u128 — the running product is always
/// an exact integer, so the division never truncates — which keeps the
/// reference independent of the dynamic-programming approach.
fn expected(m: i32, n: i32) -> u128 {
    let downs = (m - 1) as u128;
    let rights = (n - 1) as u128;
    let mut result = 1u128;

    for i in 1..=downs {
        result = result * (rights + i) / i;
    }

    result
}

/// The problem promises every answer fits in `2 * 10^9`, so a test case is only
/// legal if the reference stays inside that bound.
fn is_legal_case(m: i32, n: i32) -> bool {
    expected(m, n) <= 2_000_000_000
}

fn paths(m: i32, n: i32) -> u128 {
    Solution::unique_paths(m, n) as u128
}

#[test]
fn example_1() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
}

#[test]
fn example_2() {
    assert_eq!(Solution::unique_paths(3, 2), 3);
}

/// A 1x1 grid: the robot is already at the target, which counts as one path.
#[test]
fn single_cell() {
    assert_eq!(Solution::unique_paths(1, 1), 1);
}

/// With only one row or one column there is no choice to make.
#[test]
fn single_row_or_column() {
    for size in 1..=100 {
        assert_eq!(Solution::unique_paths(1, size), 1, "1 x {size}");
        assert_eq!(Solution::unique_paths(size, 1), 1, "{size} x 1");
    }
}

/// Transposing the grid swaps downs and rights, so the count is unchanged.
#[test]
fn transposing_the_grid_gives_the_same_count() {
    for m in 1..=20 {
        for n in 1..=20 {
            if is_legal_case(m, n) {
                assert_eq!(paths(m, n), paths(n, m), "{m} x {n}");
            }
        }
    }
}

/// Each cell is reached from above plus from the left, so the count at the
/// corner is the sum of the counts one row up and one column left.
#[test]
fn obeys_the_pascal_recurrence() {
    for m in 2..=18 {
        for n in 2..=18 {
            if is_legal_case(m, n) {
                assert_eq!(paths(m, n), paths(m - 1, n) + paths(m, n - 1), "{m} x {n}");
            }
        }
    }
}

/// Every grid up to 15x15, against the binomial reference.
#[test]
fn matches_reference_for_all_small_grids() {
    for m in 1..=15 {
        for n in 1..=15 {
            assert!(is_legal_case(m, n), "{m} x {n} exceeds the problem's bound");
            assert_eq!(paths(m, n), expected(m, n), "{m} x {n}");
        }
    }
}

/// Long, thin grids: n reaches its maximum of 100 while the answer stays small.
#[test]
fn long_thin_grids() {
    for n in [2, 50, 99, 100] {
        for m in [2, 3, 4] {
            assert_eq!(paths(m, n), expected(m, n), "{m} x {n}");
        }
    }
}

/// The largest answers the constraints permit. 18x17 yields 1_166_803_110,
/// which fits in an i32 but overflows one if intermediate products are not
/// handled with care — computing C(33, 17) as a ratio of factorials would
/// overflow long before the division.
#[test]
fn largest_answers_within_the_bound() {
    assert_eq!(Solution::unique_paths(17, 17), 601_080_390);
    assert_eq!(Solution::unique_paths(18, 17), 1_166_803_110);
    assert_eq!(Solution::unique_paths(17, 18), 1_166_803_110);
}

/// 601 million distinct paths — enumerating them one by one, as a naive
/// recursion without memoisation does, cannot finish.
#[test]
fn large_grid_must_not_enumerate_paths() {
    assert_eq!(paths(17, 17), expected(17, 17));
}
