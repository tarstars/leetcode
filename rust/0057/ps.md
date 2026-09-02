You are given an array of non-overlapping intervals `intervals`, where `intervals[i] = [start_i, end_i]` represents the start and end of the `i`th interval. The intervals are sorted in ascending order by `start_i`.

You are also given an interval `newInterval = [start, end]` that represents the start and end of another interval.

Insert `newInterval` into `intervals` so that `intervals` is still sorted in ascending order by `start_i` and contains no overlapping intervals. Merge overlapping intervals if necessary.

Return `intervals` after the insertion.

Example 1:

```text
Input:  intervals = [[1,3],[6,9]], newInterval = [2,5]
Output: [[1,5],[6,9]]
```

Example 2:

```text
Input:  intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
Output: [[1,2],[3,10],[12,16]]
```

Constraints:

- `0 <= intervals.length <= 10^4`
- `intervals[i].length == 2`
- `0 <= start_i <= end_i <= 10^5`
- `intervals` is sorted by `start_i` in ascending order.
- `newInterval.length == 2`
- `0 <= start <= end <= 10^5`
