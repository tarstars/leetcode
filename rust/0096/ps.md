Given an integer `n`, return *the number of structurally unique **BST'**s (binary search trees) which has exactly* `n` *nodes of unique values from* `1` *to* `n`.

Example 1:

```text
Input:  n = 3
Output: 5
```

Example 2:

```text
Input:  n = 1
Output: 1
```

Constraints:

- `1 <= n <= 19`

Notes:

- This is problem 95 with the trees thrown away — you only need how many. The
  recursion is the same: pick each value in `1..=n` as the root, and the count
  for that choice is (ways to build the left subtree) × (ways to build the right
  subtree). Sum over the choices.
- The shape of the subproblem collapses. In 95 the *values* mattered, so the
  recursion carried a range. Here only the *size* matters: the number of BSTs
  over `{4, 5, 6}` is the same as over `{1, 2, 3}`, because relabelling does not
  change the shape. So one parameter — a count — suffices, and the recurrence is

  ```text
  f(0) = 1
  f(m) = sum over k in 0..m of f(k) * f(m - 1 - k)
  ```

  with `k` the size of the left subtree, leaving `m - 1 - k` for the right.
- Written as plain recursion this recomputes the same `f(m)` exponentially often.
  A `Vec` filled from `0` upward turns it into two nested loops — the standard
  bottom-up dynamic programme. `f(0) = 1` matters for the same reason `vec![None]`
  did in 95: the empty tree is one tree, not zero.
- These are the Catalan numbers. `f(19)` is 1_767_263_190, which fits in an `i32`
  with room to spare, so no overflow handling is needed — but check that claim
  rather than taking it on faith, since a debug build panics on overflow and a
  release build wraps silently.
- `usize` for indices and `i64`/`u64` for the accumulator will spare you a pile of
  `as` casts; convert once at the return. Indexing a `Vec` with an `i32` does not
  compile — `Index` for `Vec` is implemented for `usize` only.
- There is also a closed form, `C(2n, n) / (n + 1)`. Computing it with `i32`
  overflows long before n = 19 even though the answer does not; the intermediate
  `C(38, 19)` is about 1.7e10. The DP never builds a number larger than the answer.
