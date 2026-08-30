The count-and-say sequence is a sequence of digit strings defined recursively:

- `countAndSay(1) = "1"`.
- `countAndSay(n)` is the run-length encoding of `countAndSay(n - 1)`.

Run-length encoding replaces each consecutive group of the same digit with the number of digits followed by that digit. For example, `"3322251"` becomes `"23321511"`.

Given a positive integer `n`, return the `n`th element of the count-and-say sequence.

Example 1:

```text
Input: n = 4
Output: "1211"
```

Explanation: `"1" -> "11" -> "21" -> "1211"`.

Example 2:

```text
Input: n = 1
Output: "1"
```

Constraints:

- `1 <= n <= 30`
