Given an array of distinct integers `candidates` and an integer `target`, return all unique combinations whose sum is `target`.

You may choose the same candidate any number of times. Two combinations are different when at least one chosen number occurs a different number of times. The combinations may be returned in any order.

Example 1:

```text
Input: candidates = [2,3,6,7], target = 7
Output: [[2,2,3],[7]]
```

Example 2:

```text
Input: candidates = [2,3,5], target = 8
Output: [[2,2,2,2],[2,3,3],[3,5]]
```

Example 3:

```text
Input: candidates = [2], target = 1
Output: []
```

Constraints:

- `1 <= candidates.length <= 30`
- `2 <= candidates[i] <= 40`
- All values in `candidates` are distinct.
- `1 <= target <= 40`
- The number of unique valid combinations is less than `150`.
