Given an integer array `nums` sorted in non-decreasing order, remove some
duplicates in-place so that each unique element appears at most twice. The
relative order of the elements must remain the same.

Place the final result in the first `k` positions of `nums` and return `k`.
The values after the first `k` positions do not matter.

Do not allocate another array. Modify `nums` in-place using `O(1)` extra space.

Example 1:

```text
Input:  nums = [1,1,1,2,2,3]
Output: 5, nums = [1,1,2,2,3,_]
Explanation: Return k = 5, with the first five elements equal to
             [1,1,2,2,3]. The value after k does not matter.
```

Example 2:

```text
Input:  nums = [0,0,1,1,1,1,2,3,3]
Output: 7, nums = [0,0,1,1,2,3,3,_,_]
Explanation: Return k = 7, with the first seven elements equal to
             [0,0,1,1,2,3,3]. The values after k do not matter.
```

Constraints:

- `1 <= nums.length <= 3 * 10^4`
- `-10^4 <= nums[i] <= 10^4`
- `nums` is sorted in non-decreasing order.
