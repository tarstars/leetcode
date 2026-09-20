An `n`-bit Gray code sequence is a sequence of `2^n` integers where:

- every value is in the range `[0, 2^n - 1]`;
- the first value is `0`;
- every pair of consecutive values differs in exactly one binary bit; and
- the last and first values also differ in exactly one bit.

Given an integer `n`, return any valid `n`-bit Gray code sequence.

Example 1:

```text
Input:  n = 2
Output: [0,1,3,2]
```

`[0,2,3,1]` is also valid: the problem accepts any sequence satisfying the
rules.

Example 2:

```text
Input:  n = 1
Output: [0,1]
```

Constraints:

- `1 <= n <= 16`
- The answer contains exactly `2^n` values.

Notes:

- The sequence is cyclic, so the transition from the final value back to zero
  matters just as much as all internal transitions.
- A useful construction reflects the sequence for `n - 1` bits, prefixes the
  reflected half with the new high bit, and appends it. The reflected order is
  what preserves one-bit transitions at the join.
- Equivalently, the standard binary-to-Gray mapping `value ^ (value >> 1)`
  generates one valid ordering directly. Whichever construction you use, the
  tests validate the defining properties rather than requiring one particular
  ordering.
- At `n = 16` the result has 65,536 entries, so the implementation should be
  linear in the output size and use an integer type wide enough for the bit
  shift.
