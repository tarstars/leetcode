The n-queens puzzle is the problem of placing `n` queens on an `n x n` chessboard so that no two queens attack each other.

Given an integer `n`, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.

Each solution contains a distinct board configuration, where `Q` represents a queen and `.` represents an empty square.

Example 1:

```text
Input:  n = 4
Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
Explanation: There are two distinct solutions to the 4-queens puzzle.
```

Example 2:

```text
Input:  n = 1
Output: [["Q"]]
```

Constraints:

- `1 <= n <= 9`
