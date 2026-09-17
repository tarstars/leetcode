Given the `head` of a linked list and a value `x`, partition it such that all nodes with a value *less than* `x` come before all nodes with a value *greater than or equal to* `x`.

You should preserve the original relative order of the nodes in each of the two partitions.

Example 1:

```text
Input:  head = [1,4,3,2,5,2], x = 3
Output: [1,2,2,4,3,5]
```

Example 2:

```text
Input:  head = [2,1], x = 2
Output: [1,2]
```

Constraints:

- The number of nodes in the list is in the range `[0, 200]`.
- `-100 <= Node.val <= 100`
- `-200 <= x <= 200`

Notes:

- Note the asymmetry in the predicate: `< x` goes left, `>= x` goes right. A
  node whose value equals `x` belongs to the *right* half.
- "Preserve the original relative order" is the whole difficulty. This is a
  *stable* partition, so the swap-based trick from problem 75 is off the table —
  swapping reorders.
- The natural shape is two output lists built by appending, then joined. Each
  needs a tail you can append to in O(1), which for `&mut Option<Box<ListNode>>`
  means holding the trailing empty slot rather than the last node.
- Every node in the input is reused; nothing is cloned and nothing is dropped,
  so each node gets moved exactly once. If the borrow checker is fighting you,
  the question to ask is which of the two cursors currently owns the node you
  just detached.
- `x` can sit outside the range of every value, in which case one of the two
  halves comes out empty — the join has to survive that.
