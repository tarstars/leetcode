Hints for problem 97. Read only as far as you need.

- The wording about splitting into `n` and `m` substrings is a distraction.
  Interleaving means exactly this: walk `s3` left to right, and take each
  character from the front of whichever of `s1` or `s2` it matches. Both must be
  fully consumed at the end. The `|n - m| <= 1` condition is automatically
  satisfied by any such walk.
- The state is two numbers: how much of `s1` and how much of `s2` you have used.
  Their sum tells you how much of `s3` you have matched, so the third index is
  not independent — that is the observation the whole solution rests on.
- So `reachable(i, j)` means "the first `i + j` characters of `s3` can be built
  from the first `i` of `s1` and the first `j` of `s2`". It holds when either the
  previous character came from `s1` — `reachable(i-1, j)` and
  `s1[i-1] == s3[i+j-1]` — or from `s2`, symmetrically.
- Greedy fails. When the next character of `s3` matches the front of both `s1`
  and `s2`, there is no local rule for which to take; you have to keep both
  possibilities alive. Plain recursion over the two indices is exponential for
  the same reason, so memoise on `(i, j)` or fill a table bottom-up.
- Check the lengths first. If `s1.len() + s2.len() != s3.len()` the answer is
  `false` and nothing else needs computing — and the rest of the code can then
  assume the sum is right.
- Rust specifics: the inputs are `String`, and you want indexed byte access, so
  `s1.as_bytes()` gives a `&[u8]` you can index with `usize`. Do not index a
  `String` directly — it is UTF-8, so `s[i]` is not defined and the compiler will
  say so. Comparing `u8` values is fine here since the alphabet is ASCII.
- A `vec![vec![false; n + 1]; m + 1]` is the straightforward table. The follow-up
  wants one row: each row only reads the row above and the cell to its left, so
  you can overwrite a single `Vec<bool>` in place — take care with the order you
  sweep it, and with initialising the first column of each new row.
- The empty cases are easy to get wrong. `reachable(0, 0)` is `true`, and the
  first row and column are prefix comparisons against `s3` — not all `false`.
