You are given an integer array `nums`. You are initially positioned at the first index, and each element represents your maximum jump length from that position.

Return `true` if you can reach the last index, or `false` otherwise.

Example 1:

```text
Input:  nums = [2,3,1,1,4]
Output: true
Explanation: Jump one step from index 0 to index 1, then three steps to the last index.
```

Example 2:

```text
Input:  nums = [3,2,1,0,4]
Output: false
Explanation: Every possible path reaches index 3, whose maximum jump length is 0.
```

Constraints:

- `1 <= nums.length <= 10^4`
- `0 <= nums[i] <= 10^5`
