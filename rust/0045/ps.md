You are given a 0-indexed array of integers `nums` with length `n`. You start at index `0`, and each value `nums[i]` is the maximum distance you may jump forward from index `i`.

Return the minimum number of jumps needed to reach index `n - 1`. Every input can reach the final index.

Example 1:

```text
Input: nums = [2,3,1,1,4]
Output: 2
```

Explanation: jump from index `0` to index `1`, then to index `4`.

Example 2:

```text
Input: nums = [2,3,0,1,4]
Output: 2
```

Constraints:

- `1 <= nums.length <= 10000`
- `0 <= nums[i] <= 1000`
- The input is generated so that the final index is reachable.
