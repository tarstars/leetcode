You are given two integer arrays `nums1` and `nums2`, sorted in non-decreasing
order, and two integers `m` and `n` describing how many meaningful values each
array contains.

`nums1` has length `m + n`: its first `m` entries are the values to keep, and
the final `n` entries are empty capacity represented by zeroes. Merge `nums2`
into `nums1` so that `nums1` becomes one sorted array. The merge must happen
in-place and the function returns nothing.

Example 1:

```text
Input:  nums1 = [1,2,3,0,0,0], m = 3,
        nums2 = [2,5,6],       n = 3
Output: [1,2,2,3,5,6]
```

Example 2:

```text
Input:  nums1 = [1], m = 1,
        nums2 = [],  n = 0
Output: [1]
```

Example 3:

```text
Input:  nums1 = [0], m = 0,
        nums2 = [1], n = 1
Output: [1]
```

Constraints:

- `nums1.length == m + n`
- `nums2.length == n`
- `0 <= m, n <= 200`
- `-10^9 <= nums1[i], nums2[j] <= 10^9`
- `nums1` and `nums2` are sorted in non-decreasing order over their meaningful
  ranges.

Notes:

- The free cells are at the *end* of `nums1`, so writing from left to right
  would overwrite values that have not been compared yet.
- The natural in-place strategy uses three cursors at the ends of the two
  meaningful prefixes and of the destination. Put the larger remaining value
  at the destination cursor, then move that cursor and whichever source cursor
  supplied the value.
- When `nums2` is exhausted, the remaining prefix of `nums1` is already in the
  correct place. If `nums1`'s meaningful prefix is exhausted first, copy the
  remaining values from `nums2`.
- The tests compare against a separate sorted concatenation, and include
  duplicates, negative values, empty sides, already ordered input, and cases
  where every value from one array must move around the other.
