Write a program to solve a Sudoku puzzle by filling its empty cells.

A valid solution must satisfy all of these rules:

- Every row contains each digit from `1` to `9` exactly once.
- Every column contains each digit from `1` to `9` exactly once.
- Each of the nine `3 x 3` boxes contains each digit from `1` to `9` exactly once.
- A `'.'` represents an empty cell.

Modify the board in place. Every input board has exactly one solution.

Example 1:

Input:

```text
53..7....
6..195...
.98....6.
8...6...3
4..8.3..1
7...2...6
.6....28.
...419..5
....8..79
```

Output:

```text
534678912
672195348
198342567
859761423
426853791
713924856
961537284
287419635
345286179
```

Constraints:

- `board.length == 9`
- `board[i].length == 9`
- `board[i][j]` is a digit or `'.'`.
- The input board has exactly one solution.
