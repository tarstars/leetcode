Given the `head` of a singly linked list and two integers `left` and `right` where `left <= right`, reverse the nodes of the list from position `left` to position `right`, and return *the reversed list*.

Positions are **1-indexed**.

Example 1:

```text
Input:  head = [1,2,3,4,5], left = 2, right = 4
Output: [1,4,3,2,5]
```

Example 2:

```text
Input:  head = [5], left = 1, right = 1
Output: [5]
```

Constraints:

- The number of nodes in the list is `n`.
- `1 <= n <= 500`
- `-500 <= Node.val <= 500`
- `1 <= left <= right <= n`

Follow up: could you do it in one pass?

Notes:

- Positions are 1-indexed, and `left` and `right` arrive as `i32` while every
  length and index you actually use is a `usize` — decide once where that
  conversion happens rather than scattering `as` casts.
- `left == right` means reverse a single node, i.e. change nothing. That case
  should fall out of the general code rather than needing a guard.
- `left == 1` is the trap: the head itself moves. A link-slot cursor
  (`&mut Option<Box<ListNode>>`) makes this need no special case, exactly as in
  problem 82 — the slot you are standing on may well be `head`.
- The shape that usually works: walk the cursor forward `left - 1` times so it
  holds the link *into* the segment, then reverse the next `right - left + 1`
  nodes, then reattach the tail.
- Reversing a run by repeatedly moving the node after the cursor to the front
  of the segment is a neat alternative to detaching and rebuilding — it touches
  each node once and never loses the tail.
- The constraints guarantee `1 <= left <= right <= n`, so there is no empty
  list and no out-of-range case to defend against.
