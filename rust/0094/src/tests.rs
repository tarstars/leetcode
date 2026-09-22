use super::*;

fn inorder(values: &[Option<i32>]) -> Vec<i32> {
    Solution::inorder_traversal(from_level_order(values))
}

/// Inserts into a binary search tree, duplicates going right. Reading such a
/// tree inorder must give the values in sorted order — an invariant, not a
/// second traversal, so it shares nothing with the solution.
fn bst_insert(root: &mut Option<Rc<RefCell<TreeNode>>>, value: i32) {
    if root.is_none() {
        *root = Some(Rc::new(RefCell::new(TreeNode::new(value))));
        return;
    }

    let mut current = Rc::clone(root.as_ref().unwrap());
    loop {
        // The borrow ends with this statement, so the reassignment below is safe.
        let next = {
            let node = current.borrow();
            if value < node.val {
                node.left.clone()
            } else {
                node.right.clone()
            }
        };

        match next {
            Some(child) => current = child,
            None => {
                let child = Rc::new(RefCell::new(TreeNode::new(value)));
                let mut node = current.borrow_mut();
                if value < node.val {
                    node.left = Some(child);
                } else {
                    node.right = Some(child);
                }
                return;
            }
        }
    }
}

fn bst_from(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = None;
    for &value in values {
        bst_insert(&mut root, value);
    }
    root
}

/// Deterministic generation, so any failure reproduces exactly.
struct Rng(u64);

impl Rng {
    fn next_value(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, bound: u64) -> i32 {
        (self.next_value() % bound) as i32
    }

    fn list(&mut self, len: usize, spread: u64) -> Vec<i32> {
        (0..len)
            .map(|_| self.below(spread) - spread as i32 / 2)
            .collect()
    }
}

#[test]
fn example_1() {
    assert_eq!(inorder(&[Some(1), None, Some(2), Some(3)]), vec![1, 3, 2]);
}

#[test]
fn example_2() {
    assert_eq!(inorder(&[]), Vec::<i32>::new());
}

#[test]
fn example_3() {
    assert_eq!(inorder(&[Some(1)]), vec![1]);
}

/// An absent tree and a lone node, spelled both ways.
#[test]
fn the_empty_and_single_cases() {
    assert_eq!(Solution::inorder_traversal(None), Vec::<i32>::new());
    assert_eq!(inorder(&[None]), Vec::<i32>::new());
    assert_eq!(inorder(&[Some(-100)]), vec![-100]);
    assert_eq!(inorder(&[Some(0)]), vec![0]);
}

/// A perfectly balanced tree, where the answer is easy to read off by hand.
#[test]
fn a_balanced_tree() {
    let values = [
        Some(4),
        Some(2),
        Some(6),
        Some(1),
        Some(3),
        Some(5),
        Some(7),
    ];
    assert_eq!(inorder(&values), vec![1, 2, 3, 4, 5, 6, 7]);
}

/// A chain leaning entirely one way. The left chain emits in reverse insertion
/// order, the right chain in order — a solution that mixes up the two visits
/// fails exactly one of them.
#[test]
fn degenerate_chains() {
    // 3 -> 2 -> 1, each a left child.
    assert_eq!(
        inorder(&[Some(3), Some(2), None, Some(1)]),
        vec![1, 2, 3]
    );
    // 1 -> 2 -> 3, each a right child.
    assert_eq!(
        inorder(&[Some(1), None, Some(2), None, Some(3)]),
        vec![1, 2, 3]
    );
}

/// The node itself must come between its two subtrees, not before or after
/// both — the difference between inorder, preorder and postorder.
#[test]
fn the_root_sits_between_its_subtrees() {
    let values = [Some(2), Some(1), Some(3)];
    assert_eq!(inorder(&values), vec![1, 2, 3]);
    assert_ne!(inorder(&values), vec![2, 1, 3], "that is preorder");
    assert_ne!(inorder(&values), vec![1, 3, 2], "that is postorder");
}

/// Reading a search tree inorder yields its values in sorted order.
#[test]
fn a_search_tree_reads_back_sorted() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);

    for size in [1usize, 2, 3, 7, 20, 50, 100] {
        for spread in [4u64, 20, 201] {
            for _ in 0..10 {
                let values = rng.list(size, spread);
                let mut expected = values.clone();
                expected.sort_unstable();

                assert_eq!(
                    Solution::inorder_traversal(bst_from(&values)),
                    expected,
                    "for {values:?}"
                );
            }
        }
    }
}

/// Inserting already-sorted values builds a 100-deep chain — the shape that
/// punishes a recursive solution hardest and the one an iterative stack has to
/// grow all the way for.
#[test]
fn a_hundred_deep_chain() {
    let ascending: Vec<i32> = (-50..50).collect();
    assert_eq!(ascending.len(), 100);
    assert_eq!(
        Solution::inorder_traversal(bst_from(&ascending)),
        ascending
    );

    let descending: Vec<i32> = (-50..50).rev().collect();
    let mut expected = descending.clone();
    expected.sort_unstable();
    assert_eq!(
        Solution::inorder_traversal(bst_from(&descending)),
        expected
    );
}

/// Duplicate values are ordinary: every node is visited, none collapsed.
#[test]
fn duplicate_values_are_all_visited() {
    assert_eq!(inorder(&[Some(7), Some(7), Some(7)]), vec![7, 7, 7]);
    assert_eq!(Solution::inorder_traversal(bst_from(&[5; 20])), vec![5; 20]);

    let mut rng = Rng(0x1357_9BDF_2468_ACE0);
    for _ in 0..100 {
        let values = rng.list(40, 3);
        let mut expected = values.clone();
        expected.sort_unstable();
        assert_eq!(Solution::inorder_traversal(bst_from(&values)), expected);
    }
}

/// Every tree shape of up to 4 nodes, labelled so that the correct answer is
/// `0..n` — built by assigning labels in inorder, which is the construction
/// direction rather than the reading one.
#[test]
fn every_small_shape() {
    /// Builds all distinct shapes with `n` nodes, as nested Option structures.
    fn shapes(n: usize) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![None];
        }
        let mut out = Vec::new();
        for left_size in 0..n {
            for left in shapes(left_size) {
                for right in shapes(n - 1 - left_size) {
                    // Deep copies, so no two generated trees share a node —
                    // labelling one must not mutate another.
                    let node = Rc::new(RefCell::new(TreeNode::new(0)));
                    node.borrow_mut().left = deep_copy(&left);
                    node.borrow_mut().right = deep_copy(&right);
                    out.push(Some(node));
                }
            }
        }
        out
    }

    /// A structurally identical tree sharing none of the original's nodes.
    fn deep_copy(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = node.as_ref()?;
        let (val, left, right) = {
            let borrowed = node.borrow();
            (borrowed.val, borrowed.left.clone(), borrowed.right.clone())
        };
        let copy = Rc::new(RefCell::new(TreeNode::new(val)));
        copy.borrow_mut().left = deep_copy(&left);
        copy.borrow_mut().right = deep_copy(&right);
        Some(copy)
    }

    /// Assigns 0, 1, 2, ... to the nodes in inorder position.
    fn label(node: &Option<Rc<RefCell<TreeNode>>>, next: &mut i32) {
        let Some(node) = node.as_ref() else { return };
        let left = node.borrow().left.clone();
        label(&left, next);
        node.borrow_mut().val = *next;
        *next += 1;
        let right = node.borrow().right.clone();
        label(&right, next);
    }

    let mut checked = 0;
    for n in 0..=4usize {
        for tree in shapes(n) {
            let mut next = 0;
            label(&tree, &mut next);
            assert_eq!(
                Solution::inorder_traversal(tree),
                (0..n as i32).collect::<Vec<i32>>(),
                "for a {n}-node shape"
            );
            checked += 1;
        }
    }
    // Catalan numbers: 1, 1, 2, 5, 14.
    assert_eq!(checked, 23, "the shape count changed");
}
