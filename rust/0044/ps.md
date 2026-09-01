Given an input string `s` and a pattern `p`, implement wildcard matching with these rules:

- `'?'` matches exactly one character.
- `'*'` matches any sequence of characters, including an empty sequence.

The pattern must match the entire input string, not merely a substring.

Example 1:

```text
Input: s = "aa", p = "a"
Output: false
```

Example 2:

```text
Input: s = "aa", p = "*"
Output: true
```

Example 3:

```text
Input: s = "cb", p = "?a"
Output: false
```

Constraints:

- `0 <= s.length, p.length <= 2000`
- `s` contains only lowercase English letters.
- `p` contains only lowercase English letters, `'?'`, and `'*'`.
