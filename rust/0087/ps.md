We can scramble a string `s` to get a string `t` using the following algorithm:

1. If the length of the string is 1, stop.
2. If the length of the string is greater than 1, do the following:
   - Split the string into two non-empty substrings at a random index, i.e. if
     the string is `s`, divide it to `x` and `y` where `s = x + y`.
   - **Randomly** decide to swap the two substrings or to keep them in the same
     order, so `s` may become `s = x + y` or `s = y + x`.
   - Apply step 1 recursively on each of the two substrings `x` and `y`.

Given two strings `s1` and `s2` of the same length, return `true` if `s2` is a scrambled string of `s1`, otherwise return `false`.

Example 1:

```text
Input:  s1 = "great", s2 = "rgeat"
Output: true
```

Example 2:

```text
Input:  s1 = "abcde", s2 = "caebd"
Output: false
```

Example 3:

```text
Input:  s1 = "a", s2 = "a"
Output: true
```

Constraints:

- `s1.length == s2.length`
- `1 <= s1.length <= 30`
- `s1` and `s2` consist of lowercase English letters.

Notes:

- The definition is already a recursion: `s2` is a scramble of `s1` if there is
  some split point `i` where *either* the two halves match up in order
  (`s1[..i]` with `s2[..i]`, and `s1[i..]` with `s2[i..]`) *or* they match up
  crossed over. Note that when the halves are swapped, the split points on the
  two sides are at mirrored positions — getting that index arithmetic right is
  most of the work.
- Written literally that recursion is exponential and will not finish at length
  30. There are two independent ways to tame it, and measured on a genuine
  `false` pair of length 30 over three letters, *either one alone* is enough:

  | variant                        | recursive calls    |
  | ------------------------------ | ------------------ |
  | memoisation + anagram check    | 169                |
  | anagram check only             | 223                |
  | memoisation only               | 70,228             |
  | neither                        | over 200,000,000   |

  - **Memoisation.** The state is only `(start in s1, start in s2, length)`,
    at most 30*30*30 = 27,000 combinations.
  - **An anagram early exit.** If the two substrings do not contain the same
    letters, no amount of splitting will help, so return false immediately.
    This prunes far harder than its simplicity suggests.

  Doing both is the natural solution, but it is worth knowing that the anagram
  check is carrying most of the weight.
- Watch out for `String` indexing. Rust will not let you subscript a `String` by
  character position, and `&s[a..b]` slices by *bytes*. These inputs are ASCII,
  so `as_bytes()` is the easy way out, and it makes the anagram check cheap too.
- The tests use a genuinely different oracle for short inputs: they *generate*
  the complete set of strings reachable from `s1` and ask whether `s2` is in it,
  rather than deciding the question recursively.
