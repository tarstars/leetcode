There is an integer array `nums` sorted in non-decreasing order, whose values
are not necessarily distinct.

Before being passed to your function, `nums` is possibly rotated at an unknown
pivot index `k` (`0 <= k < nums.length`) so that the resulting array is
`[nums[k], nums[k + 1], ..., nums[n - 1], nums[0], ..., nums[k - 1]]`.

Given the rotated array `nums` and an integer `target`, return `true` if
`target` is in `nums`, or `false` otherwise.

You must decrease the overall number of operation steps as much as possible.

Example 1:

```text
Input:  nums = [2,5,6,0,0,1,2], target = 0
Output: true
```

Example 2:

```text
Input:  nums = [2,5,6,0,0,1,2], target = 3
Output: false
```

Constraints:

- `1 <= nums.length <= 5000`
- `-10^4 <= nums[i] <= 10^4`
- `nums` is guaranteed to be rotated at some pivot.
- `-10^4 <= target <= 10^4`

Follow-up: This problem is similar to Search in Rotated Sorted Array, but
`nums` may contain duplicates. Does this affect the runtime complexity? How
and why?
