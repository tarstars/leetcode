use crate::Solution;

impl Solution {
    /// Dijkstra's Dutch National Flag partition, named for the flag's three
    /// bands. Where sol_00 overwrites prefixes with three write cursors, this
    /// one never writes a value it did not read: it only ever swaps, growing
    /// three regions inward from both ends until nothing unexamined is left.
    ///
    /// Four regions, delimited by three indices:
    ///
    /// ```text
    ///   [0, low)      all 0s      settled
    ///   [low, mid)    all 1s      settled
    ///   [mid, high)   unexamined  <- shrinks to nothing
    ///   [high, len)   all 2s      settled
    /// ```
    ///
    /// Each step looks at `nums[mid]`, the first unexamined element:
    ///
    /// - a `0` belongs at the front, so swap it with the first `1` (at `low`)
    ///   and advance both — the displaced value is a `1`, already known, so
    ///   `mid` may safely move past it;
    /// - a `1` is already where it belongs, so just advance `mid`;
    /// - a `2` belongs at the back, so shrink `high` and swap it there.
    ///
    /// That last case is the one to be careful with: **`mid` must not advance**.
    /// The value swapped back from `high` came out of the unexamined region and
    /// has never been looked at, so the next step has to examine it in place.
    /// Advancing `mid` there is the classic bug — it leaves stray 2s near the
    /// front, and `[1, 2, 0]` sorts to `[1, 0, 2]`.
    ///
    /// One pass, O(n) time, O(1) space.
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut low, mut mid, mut high) = (0usize, 0usize, nums.len());

        while mid < high {
            match nums[mid] {
                0 => {
                    nums.swap(low, mid);
                    low += 1;
                    mid += 1;
                }
                1 => {
                    mid += 1;
                }
                _ => {
                    high -= 1;
                    nums.swap(mid, high);
                }
            }
        }
    }
}
