use crate::Solution;
use std::collections::HashMap;

impl Solution {
    /// Same recursion as `sol_00`, with the allocation taken out of it.
    ///
    /// Three changes:
    ///
    /// * **Never build a substring.** Work on `&[u8]` and recurse on
    ///   `(i, j, len)` — a window into `s1`, a window into `s2`, and their
    ///   shared length. Slicing a `&[u8]` is a pointer and a length, not a
    ///   copy, so no `to_string()` appears anywhere.
    /// * **A memo key of three `usize`s.** `(String, String)` forced two fresh
    ///   allocations on every probe *and* every insert. There are at most
    ///   30*30*30 = 27,000 states, and this key names them directly.
    /// * **An anagram early exit.** If the two windows do not use the same
    ///   letters, no split can rescue them. This prunes hardest of all — see
    ///   the table in `ps.md`.
    ///
    /// One ordering detail worth noticing: the equality and anagram checks come
    /// *before* the memo lookup. Both are cheap and settle most calls outright,
    /// so paying for a hash first would be backwards.
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let (a, b) = (s1.as_bytes(), s2.as_bytes());
        a.len() == b.len() && solve(a, b, 0, 0, a.len(), &mut HashMap::new())
    }
}

/// Do the two windows use exactly the same letters? One pass, counting up for
/// `x` and down for `y`; they match when every tally lands back on zero.
fn same_letters(x: &[u8], y: &[u8]) -> bool {
    let mut tally = [0i8; 26];
    for (&left, &right) in x.iter().zip(y) {
        tally[(left - b'a') as usize] += 1;
        tally[(right - b'a') as usize] -= 1;
    }
    tally.iter().all(|&count| count == 0)
}

/// Is `b[j..j + len]` a scramble of `a[i..i + len]`?
fn solve(
    a: &[u8],
    b: &[u8],
    i: usize,
    j: usize,
    len: usize,
    memo: &mut HashMap<(usize, usize, usize), bool>,
) -> bool {
    let (x, y) = (&a[i..i + len], &b[j..j + len]);

    if x == y {
        return true;
    }
    if !same_letters(x, y) {
        return false;
    }
    if let Some(&known) = memo.get(&(i, j, len)) {
        return known;
    }

    // Split `x` after `k` characters. Either the halves line up in order, or
    // they are swapped — and when swapped, `x`'s leading `k` characters sit at
    // the *end* of `y`, which is where the mirrored index `j + len - k` comes
    // from.
    let answer = (1..len).any(|k| {
        (solve(a, b, i, j, k, memo) && solve(a, b, i + k, j + k, len - k, memo))
            || (solve(a, b, i, j + len - k, k, memo) && solve(a, b, i + k, j, len - k, memo))
    });

    memo.insert((i, j, len), answer);
    answer
}
