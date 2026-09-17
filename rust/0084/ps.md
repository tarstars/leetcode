Given an array of integers `heights` representing the histogram's bar height where the width of each bar is `1`, return the area of the largest rectangle in the histogram.

Example 1:

```text
Input:  heights = [2,1,5,6,2,3]
Output: 10
```

The largest rectangle is formed by the bars of height 5 and 6, giving `5 * 2 = 10`.

Example 2:

```text
Input:  heights = [2,4]
Output: 4
```

Constraints:

- `1 <= heights.length <= 10^5`
- `0 <= heights[i] <= 10^4`

Notes:

- The obvious O(n^2) — for every pair of bars, take the minimum height across
  the span — is what the tests use as an oracle. It is too slow for the real
  constraints, so the exercise is finding the O(n) version.
- A useful reframing: every maximal rectangle has some bar as its *limiting*
  height. So for each bar `i`, ask how far left and how far right it can extend
  before meeting a strictly shorter bar. The answer is
  `max over i of heights[i] * (right[i] - left[i] - 1)`.
- Computing those two boundaries for every bar in linear total time is the
  crux. Think about what you can throw away permanently once you have seen a
  shorter bar.
- The maximum possible answer is `10^5 * 10^4 = 10^9`, which fits in `i32` with
  room to spare — but only just, so keep widths and heights from multiplying in
  a wider-then-narrower order that you have not thought about.
