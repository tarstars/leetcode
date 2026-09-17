Given a `rows x cols` binary matrix filled with `'0'` and `'1'`, find the largest rectangle containing only `'1'`s and return its area.

Example 1:

```text
Input:  matrix = [["1","0","1","0","0"],
                  ["1","0","1","1","1"],
                  ["1","1","1","1","1"],
                  ["1","0","0","1","0"]]
Output: 6
```

The 2 x 3 block spanning rows 1-2 and columns 2-4 is all ones.

Example 2:

```text
Input:  matrix = [["0"]]
Output: 0
```

Example 3:

```text
Input:  matrix = [["1"]]
Output: 1
```

Constraints:

- `rows == matrix.length`
- `cols == matrix[i].length`
- `1 <= row, cols <= 200`
- `matrix[i][j]` is `'0'` or `'1'`.

Notes:

- This is problem 84 wearing a disguise. Pick any row and call it the floor.
  For each column, count how many consecutive `'1'`s sit directly on top of
  that floor — that column's run length is a bar height, and the whole row
  becomes a histogram. The largest rectangle whose *bottom* edge lies on this
  row is exactly `largest_rectangle_area` of that histogram.
- So: sweep the rows top to bottom, keep one running array of heights, and take
  the best answer over all rows. A `'0'` resets its column's height to zero; a
  `'1'` adds one to it.
- That gives O(rows * cols) overall, since each row costs one linear histogram
  pass.
- The tests use a genuinely different oracle — for every pair of rows, mark the
  columns that are solid `'1'` across that whole band and find the longest run
  of them. That is O(rows^2 * cols), fast enough to check even a 200 x 200
  matrix, and it shares no machinery with the histogram approach.
- Worth deciding up front how you carry the heights between rows, and whether
  your problem-84 routine can be reused as-is or wants to be inlined.
