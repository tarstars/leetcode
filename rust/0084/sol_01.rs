use crate::Solution;

impl Solution {
    /// Same monotonic stack as `sol_00`, written the way the standard library
    /// wants it.
    ///
    /// Three things do most of the tidying:
    ///
    /// * The stack holds bare `usize` indices. A `(index, height)` pair is
    ///   redundant — the height is `heights[index]` — and carrying it forces
    ///   `as i32` casts on something that is naturally an index.
    /// * Appending one virtual bar of height 0 past the right edge makes the
    ///   scan flush itself, so the "drain what's left" loop disappears along
    ///   with its near-duplicate width arithmetic. That duplication is exactly
    ///   where the two halves drifted into different conventions.
    /// * `while let Some(&top) = stack.last()` replaces `!stack.is_empty()`
    ///   followed by `unwrap()`.
    ///
    /// The invariant: the stack holds, left to right, the bars whose right edge
    /// is still unknown, in increasing height. A bar's rectangle is settled the
    /// moment something shorter appears, because that is where it must stop.

    // With `top` gone, whatever is on the stack now is the nearest
    // shorter bar to its left — a blocker, so the rectangle starts
    // one column past it, and ends one column before `right`.


    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut largest = 0;

        for (right, height) in heights.iter().copied().chain([0]).enumerate() {
            while let Some(&top) = stack.last() {
                if heights[top] < height {
                    break;
                }
                stack.pop();

                let left = stack.last().map_or(0, |&blocker| blocker + 1);
                largest = largest.max(heights[top] * (right - left) as i32);
            }
            stack.push(right);
        }

        largest
    }
}
