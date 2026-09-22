A **valid IP address** consists of exactly four integers separated by single dots. Each integer is between `0` and `255` (**inclusive**) and **cannot have leading zeros**.

- For example, `"0.1.2.201"` and `"192.168.1.1"` are valid IP addresses, but `"0.011.255.245"`, `"192.168.1.312"` and `"192.168@1.1"` are **not**.

Given a string `s` containing only digits, return *all possible valid IP addresses that can be formed by inserting dots into `s`*. You are **not** allowed to reorder or remove any digits in `s`.

You may return the valid IP addresses in **any** order.

Example 1:

```text
Input:  s = "25525511135"
Output: ["255.255.11.135","255.255.111.35"]
```

Example 2:

```text
Input:  s = "0000"
Output: ["0.0.0.0"]
```

Example 3:

```text
Input:  s = "101023"
Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
```

Constraints:

- `1 <= s.length <= 20`
- `s` consists of digits only.

Notes:

- Every digit must be used and none may be reordered, so the whole problem is
  choosing three cut positions. A string shorter than 4 or longer than 12 has
  no answers at all — worth checking first, since the input can be up to 20.
- The leading-zero rule is the one people get wrong: `"0"` is a valid part,
  `"00"` and `"01"` are not. So a part starting with `'0'` must be exactly one
  character long.
- A part is at most 3 digits, and `255` is the boundary — `"256"` is out.
- Two routes: **backtracking**, placing one part at a time and recursing on the
  rest, or a **triple loop** over the three cut positions, validating the four
  slices. The second is what the tests use as an oracle, so writing that as
  your solution proves nothing.
- The output order is unspecified, so the tests sort both sides before
  comparing.
- `"0000"` and `"255255255255"` are the two edge cases worth having in mind
  while you write the validity check.
