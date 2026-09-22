Given the `root` of a binary tree, return *the inorder traversal of its nodes' values*.

Inorder means: everything in the left subtree, then the node itself, then everything in the right subtree.

Example 1:

```text
Input:  root = [1,null,2,3]
Output: [1,3,2]
```

Example 2:

```text
Input:  root = []
Output: []
```

Example 3:

```text
Input:  root = [1]
Output: [1]
```

Constraints:

- The number of nodes in the tree is in the range `[0, 100]`.
- `-100 <= Node.val <= 100`

Follow up: recursive solution is trivial, could you do it iteratively?

Notes:

- This is the first tree problem here, so the node type is new:

  ```rust
  Option<Rc<RefCell<TreeNode>>>
  ```

  Read it outside in. `Option` — the child may be absent. `Rc` — *reference
  counted* shared ownership, because a tree node can in principle be pointed at
  from more than one place, and unlike `Box` an `Rc` can be cloned cheaply
  (cloning bumps a counter, it does not copy the node). `RefCell` — interior
  mutability, letting you mutate through a shared reference, with the
  borrow rules checked at *run time* instead of compile time.
- The two calls you need: `node.borrow()` gives a shared view, `node.borrow_mut()`
  gives an exclusive one. They panic if the rules are violated — two live
  borrows where one is mutable — so keep each borrow short. A borrow lives
  until the end of its enclosing statement, which is why
  `let next = node.borrow().left.clone();` is fine but holding the result of
  `node.borrow()` in a variable across a loop iteration usually is not.
- `Rc::clone(&node)` is the idiomatic way to take another handle; `node.clone()`
  does the same thing but reads like a deep copy, which it is not.
- Recursion is the easy route and 100 nodes will not overflow the stack. The
  follow-up wants the iterative version: push left spines onto an explicit
  stack, pop a node, emit it, then move to its right child.
- The tests lean on an invariant rather than a second traversal: inserting
  values into a binary search tree and reading them back inorder must give them
  in sorted order, and BST insertion never performs an inorder walk.
