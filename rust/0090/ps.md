Given an integer array `nums` that may contain duplicates, return *all possible subsets (the power set)*.

The solution set **must not** contain duplicate subsets. Return the solution in **any order**.

Example 1:

```text
Input:  nums = [1,2,2]
Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
```

Example 2:

```text
Input:  nums = [0]
Output: [[],[0]]
```

Constraints:

- `1 <= nums.length <= 10`
- `-10 <= nums[i] <= 10`

Notes:

- The input is **not** sorted, and duplicates need not be adjacent — `[4,4,4,1,4]`
  is a legal input. Sorting first is what makes equal values adjacent, which is
  what every deduplication strategy below relies on.
- Two subsets are the same when they contain the same values with the same
  multiplicities, regardless of order. So for `[1,2,2]` the answer has six
  entries, not eight: picking "the first 2" and picking "the second 2" give the
  same subset.
- The count is a product: for multiplicities `m1..mk` the answer has
  `(m1+1) * (m2+1) * ... * (mk+1)` subsets, because each distinct value
  contributes a choice of how many copies to take. That is worth using as a
  self-check.
- Three routes, roughly in order of how much they teach:
  - **Backtracking with a skip rule.** Sort, then at each position either take
    the value or skip *every* copy of it. Generates each subset exactly once, no
    deduplication needed.
  - **Iterative growth.** Start from `[[]]` and fold in one distinct value at a
    time, appending 0, 1, ... up to `m` copies of it to every subset built so
    far.
  - **Bitmask plus a set.** Enumerate all `2^n` masks and throw the results into
    a `HashSet` to deduplicate. Only viable because `n <= 10`, and it is what
    the tests use as an oracle — so writing it as your solution proves nothing.
- The output order is unspecified, so the tests canonicalise both sides (sort
  each subset, then sort the list) rather than pinning one particular order.
