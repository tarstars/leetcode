use crate::Solution;

fn binary_search(mut l: i32, mut r: i32, p: impl Fn(i32) -> bool) -> i32 {
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if p(m) {
            r = m;
        } else {
            l = m;
        }
    }
    r
}

impl Solution {
    /// Binary search on the answer, instead of sol_00's Newton iteration.
    ///
    /// The predicate "r * r <= x" is monotone in r — true for every r up to the
    /// answer, false above it — so the search is for the last r satisfying it.
    /// The answer never exceeds 46340, since 46341^2 already passes i32::MAX,
    /// which bounds the upper end of the range without any guesswork.
    ///
    /// The trap is the same one Newton had, in a different place: squaring a
    /// candidate near the top of the range overflows i32. Compare in i64, or
    /// test `mid <= x / mid` instead of `mid * mid <= x`.
    pub fn my_sqrt(a: i32) -> i32 {
        binary_search(-1, 46341, |x| x * x > a) - 1
    }
}
