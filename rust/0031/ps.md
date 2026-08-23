A permutation of an array of integers is an arrangement of its members into a sequence or linear order.

For example, for `arr = [1,2,3]`, the following are permutations of `arr`: `[1,2,3]`, `[1,3,2]`, `[3,1,2]`, and `[2,3,1]`.

The next permutation of an array of integers is the next lexicographically greater permutation of its integers. More formally, if all the permutations of the array are sorted in lexicographical order, the next permutation is the permutation that immediately follows the given array. If such an arrangement is not possible, the array must be rearranged into the lowest possible order (that is, sorted in ascending order).

For example, the next permutation of `arr = [1,2,3]` is `[1,3,2]`. The next permutation of `arr = [2,3,1]` is `[3,1,2]`. The next permutation of `arr = [3,2,1]` is `[1,2,3]` because `[3,2,1]` has no lexicographically greater rearrangement.

Given an array of integers `nums`, find its next permutation.

The replacement must be in place and use only constant extra memory.

Example 1:

Input: nums = [1,2,3]
Output: [1,3,2]

Example 2:

Input: nums = [3,2,1]
Output: [1,2,3]

Example 3:

Input: nums = [1,1,5]
Output: [1,5,1]

Constraints:

- `1 <= nums.length <= 100`
- `0 <= nums[i] <= 100`
