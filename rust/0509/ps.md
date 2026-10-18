# 509. Fibonacci Number

[LeetCode problem](https://leetcode.com/problems/fibonacci-number/)

Compute the Fibonacci number at index `n`, using zero-based indexing:

```text
F(0) = 0
F(1) = 1
F(n) = F(n - 1) + F(n - 2) for n >= 2
```

## Examples

| n | F(n) |
|---|------|
| 2 | 1    |
| 3 | 2    |
| 4 | 3    |

## Constraints

- `0 <= n <= 30`
- Rust signature: `pub fn fib(n: i32) -> i32`

## Practice

The solution in `sol_00.rs` uses matrix exponentiation, taking `O(log n)` time
and `O(1)` extra space. Run `cargo run` for sample calls and `cargo test` for
the examples and boundary cases.
