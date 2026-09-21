A message containing letters from `A-Z` can be encoded into numbers using the following mapping:

```text
'A' -> "1"
'B' -> "2"
...
'Z' -> "26"
```

To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above (there may be multiple ways). For example, `"11106"` can be mapped into:

- `"AAJF"` with the grouping `(1 1 10 6)`
- `"KJF"` with the grouping `(11 10 6)`

Note that the grouping `(1 11 06)` is invalid because `"06"` cannot be mapped into `'F'` since `"6"` is different from `"06"`.

Given a string `s` containing only digits, return *the number of ways to decode it*. If the entire string cannot be decoded in any valid way, return `0`.

The test cases are generated so that the answer fits in a **32-bit** integer.

Example 1:

```text
Input:  s = "12"
Output: 2
```

`"12"` could be decoded as `"AB"` (1 2) or `"L"` (12).

Example 2:

```text
Input:  s = "226"
Output: 3
```

`"226"` could be decoded as `"BZ"` (2 26), `"VF"` (22 6), or `"BBF"` (2 2 6).

Example 3:

```text
Input:  s = "06"
Output: 0
```

`"06"` cannot be mapped to `"F"` because of the leading zero.

Constraints:

- `1 <= s.length <= 100`
- `s` contains only digits and may contain leading zero(s).

Notes:

- The recurrence is Fibonacci-shaped: `ways(i)` = `ways(i+1)` if the single
  digit at `i` is decodable, plus `ways(i+2)` if the pair starting at `i` is.
  A string of `n` ones has exactly `Fib(n+1)` decodings, which is worth using
  as a self-check.
- `'0'` is where almost every bug lives. It is never a letter on its own, and
  the only two-digit codes ending in it are `"10"` and `"20"`. So a `'0'` must
  be consumed as the second half of a pair or the whole decoding fails.
- A two-digit code is valid exactly when it reads `10..=26` — note `"01"` is
  not `1`, and `"27"` is not a code at all.
- Watch the answer's size: `Fib(47)` already overflows `i32`. The problem
  guarantees the given inputs stay inside, but if you ever generate your own
  long test strings, that guarantee is yours to keep.
- O(1) space is achievable — the recurrence only ever looks two positions
  ahead, so two running variables replace the whole table.
- The tests use a constructive oracle for short inputs: they build the actual
  set of decoded letter strings and count it, rather than running the same
  recurrence a second time.
