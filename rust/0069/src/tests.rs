use super::*;

/// The largest r with r*r <= i32::MAX. 46340^2 = 2_147_395_600 fits;
/// 46341^2 = 2_147_488_281 does not.
const MAX_ROOT: i32 = 46_340;

/// Checks the defining property rather than comparing against another square
/// root: r is the answer exactly when r*r <= x < (r+1)^2. That pins r uniquely,
/// so this is a complete oracle and it borrows nothing from any particular
/// algorithm. The arithmetic is in i64 because (r+1)^2 overflows i32 near the
/// top of the input range.
fn assert_is_floor_sqrt(x: i32, r: i32) {
    assert!(r >= 0, "sqrt({x}) returned {r}, which is negative");

    let x = x as i64;
    let r = r as i64;

    assert!(r * r <= x, "sqrt({x}) returned {r}, but {r}^2 exceeds {x}");
    assert!(
        (r + 1) * (r + 1) > x,
        "sqrt({x}) returned {r}, but {} would also fit",
        r + 1
    );
}

fn check(x: i32) {
    assert_is_floor_sqrt(x, Solution::my_sqrt(x));
}

/// A deterministic generator, so any failure reproduces.
struct Rng(u64);

impl Rng {
    fn next_value(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, bound: u32) -> i32 {
        (self.next_value() % bound as u64) as i32
    }
}

#[test]
fn example_1() {
    assert_eq!(Solution::my_sqrt(4), 2);
}

/// Rounded down, not to nearest: sqrt(8) is 2.828..., so the answer is 2.
#[test]
fn example_2() {
    assert_eq!(Solution::my_sqrt(8), 2);
}

/// Zero is a legal input and its root is zero.
#[test]
fn zero() {
    assert_eq!(Solution::my_sqrt(0), 0);
}

#[test]
fn the_smallest_inputs() {
    assert_eq!(Solution::my_sqrt(1), 1);
    assert_eq!(Solution::my_sqrt(2), 1);
    assert_eq!(Solution::my_sqrt(3), 1);
    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(5), 2);
}

/// Every perfect square in range must come back exact — no rounding down to
/// r - 1 through an off-by-one in the search.
#[test]
fn every_perfect_square_is_exact() {
    for r in 0..=MAX_ROOT {
        let square = r as i64 * r as i64;
        assert_eq!(
            Solution::my_sqrt(square as i32),
            r,
            "sqrt({square}) should be exactly {r}"
        );
    }
}

/// The two values bracketing every perfect square: one below must round down
/// to r - 1, one above must stay at r. This is where a `<` versus `<=` slip
/// shows itself.
#[test]
fn the_neighbours_of_every_perfect_square() {
    for r in 1..=MAX_ROOT {
        let square = r as i64 * r as i64;

        assert_eq!(
            Solution::my_sqrt(square as i32 - 1),
            r - 1,
            "sqrt({}) should be {}",
            square - 1,
            r - 1
        );

        if square + 1 <= i32::MAX as i64 {
            assert_eq!(
                Solution::my_sqrt(square as i32 + 1),
                r,
                "sqrt({}) should still be {r}",
                square + 1
            );
        }
    }
}

/// Exhaustive over a dense low range, against the defining property.
#[test]
fn every_small_input() {
    for x in 0..=200_000 {
        check(x);
    }
}

/// Scattered across the whole input range.
#[test]
fn values_across_the_range() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);

    for _ in 0..20_000 {
        check(rng.below(i32::MAX as u32));
    }

    for shift in 0..31 {
        let power = 1i32 << shift;
        check(power);
        check(power - 1);
        if power < i32::MAX {
            check(power + 1);
        }
    }
}

/// The top of the range. Squaring a candidate near 46341 overflows i32, so a
/// solution that tests `mid * mid <= x` in i32 panics in debug and wraps in
/// release; comparing `mid <= x / mid` or widening to i64 avoids it.
#[test]
fn largest_input() {
    assert_eq!(Solution::my_sqrt(i32::MAX), MAX_ROOT);
    assert_eq!(Solution::my_sqrt(2_147_395_600), MAX_ROOT);
    assert_eq!(Solution::my_sqrt(2_147_395_599), MAX_ROOT - 1);

    for x in (i32::MAX - 1000)..=i32::MAX {
        check(x);
    }
    for x in (2_147_395_600 - 1000)..=(2_147_395_600 + 1000) {
        check(x);
    }
}

/// The result is monotonic: a larger input never gives a smaller root.
#[test]
fn the_result_never_decreases() {
    let mut previous = 0;

    for x in 0..=100_000 {
        let root = Solution::my_sqrt(x);
        assert!(
            root >= previous,
            "sqrt({x}) = {root} is below sqrt({}) = {previous}",
            x - 1
        );
        previous = root;
    }
}
