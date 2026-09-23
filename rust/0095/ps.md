Given an integer `n`, return *all the structurally unique **BST'**s (binary search trees), which has exactly* `n` *nodes of unique values from* `1` *to* `n`. Return the answer in **any order**.

Example 1:

```text
Input:  n = 3
Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
```

Example 2:

```text
Input:  n = 1
Output: [[1]]
```

Constraints:

- `1 <= n <= 8`

Notes:

- Same node type as problem 94, `Option<Rc<RefCell<TreeNode>>>`, but now you are
  *building* trees rather than reading one. The return type is a `Vec` of roots.
- The shape of the recursion: pick each value `root` in `1..=n` as the root. Every
  value below it must live in the left subtree and every value above it in the
  right subtree — that is exactly the BST property. So recurse on the ranges
  `low..=root-1` and `root+1..=high`, get back a list of possible left subtrees
  and a list of possible right subtrees, and pair every left with every right.
  The count is the Catalan number: 1, 1, 2, 5, 14, 42, 132, 429, 1430.
- The empty range must return `vec![None]`, not `vec![]`. One way to have nothing
  is still one way — return an empty list and every product collapses to nothing.
- Sharing versus copying. `Rc::clone` gives another handle to the *same* node, so
  two returned trees would share it. For this problem that is acceptable — nobody
  mutates the result — and it is what makes the naive solution cheap. But if you
  reuse a subtree in several trees and then mutate through one of them, every tree
  holding that handle sees the change. Decide deliberately which you want; the
  tests here deep-copy before touching anything, so either choice passes.
- Building a node with children in one expression is wordy. `TreeNode::new(v)`
  gives you a childless node; assigning the fields afterwards through
  `borrow_mut()` is usually clearer than constructing the struct literal, and it
  is the only option here since the fields are what you are filling in.
- Watch the borrow discipline from problem 94: a `borrow_mut()` guard held while
  you call something that borrows the same node again panics at run time.
