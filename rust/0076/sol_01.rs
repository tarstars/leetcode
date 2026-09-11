use crate::Solution;

impl Solution {
    /// The same sliding window as sol_00, with three changes of style rather
    /// than of algorithm:
    ///
    /// - **`Option` instead of a sentinel.** sol_00 signals "no window found"
    ///   with `best_length == i32::MAX`. That is a value pretending to be a
    ///   flag, and it leaves `best_left`/`best_right` sitting at a misleading 0
    ///   whenever no window exists. Here the absence is in the type, so the
    ///   compiler insists both cases are handled and the empty-string result
    ///   cannot be forgotten.
    ///
    /// - **`usize` throughout.** Lengths and indices are `usize` in Rust.
    ///   Keeping the length as `i32` forces a cast in each direction; the `i32`
    ///   belongs to the signature, not to the interior.
    ///
    /// - **Bytes instead of `Vec<char>`.** The constraints promise English
    ///   letters, so the input is ASCII and `as_bytes()` indexes in O(1) with no
    ///   allocation. A fixed array replaces the hash map for the same reason:
    ///   the key space is 128 values, so an array offset beats hashing.
    ///
    /// One pass over `s`, so O(m + n): `left` only ever moves forward, which
    /// makes the inner loop amortised O(1) per character, and the result is
    /// built once at the end rather than on every improvement.
    pub fn min_window(s: String, t: String) -> String {
        let bytes = s.as_bytes();

        // How many of each byte the window still owes, and how many distinct
        // bytes are still owed at all.
        let mut needed = [0i32; 128];
        let mut missing = 0usize;
        for &c in t.as_bytes() {
            if needed[c as usize] == 0 {
                missing += 1;
            }
            needed[c as usize] += 1;
        }

        let mut best: Option<(usize, usize)> = None;
        let mut left = 0;

        for (right, &c) in bytes.iter().enumerate() {
            // Counts go negative for a surplus, so only the crossing to zero
            // marks a byte as satisfied.
            needed[c as usize] -= 1;
            if needed[c as usize] == 0 {
                missing -= 1;
            }

            // Nothing is owed, so shrink from the left while that holds.
            while missing == 0 {
                let length = right - left + 1;
                if best.is_none_or(|(l, r)| length < r - l + 1) {
                    best = Some((left, right));
                }

                let dropped = bytes[left] as usize;
                needed[dropped] += 1;
                if needed[dropped] == 1 {
                    missing += 1;
                }
                left += 1;
            }
        }

        match best {
            None => String::new(),
            Some((l, r)) => s[l..=r].to_owned(),
        }
    }
}
