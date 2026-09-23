use super::*;
use std::collections::HashSet;

/// A preorder rendering with explicit markers for absent children, which
/// determines the tree completely — two trees agree on it exactly when they
/// have the same shape and the same values in the same places.
fn canonical(node: &Option<Rc<RefCell<TreeNode>>>) -> String {
    match node {
        None => ".".to_string(),
        Some(node) => {
            let node = node.borrow();
            format!(
                "({} {} {})",
                node.val,
                canonical(&node.left),
                canonical(&node.right)
            )
        }
    }
}

/// Reads the values out in inorder. For a search tree this must come back
/// sorted, which is how the tests check the BST property.
fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
    let Some(node) = node.as_ref() else { return };
    let node = node.borrow();
    inorder(&node.left, out);
    out.push(node.val);
    inorder(&node.right, out);
}

fn node_count(node: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    match node {
        None => 0,
        Some(node) => {
            let node = node.borrow();
            1 + node_count(&node.left) + node_count(&node.right)
        }
    }
}

fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    match node {
        None => 0,
        Some(node) => {
            let node = node.borrow();
            1 + height(&node.left).max(height(&node.right))
        }
    }
}

/// Every value in the left subtree below the node's, every value in the right
/// subtree above it — checked against the bounds inherited from the ancestors,
/// not just against the immediate parent.
fn is_search_tree(
    node: &Option<Rc<RefCell<TreeNode>>>,
    low: Option<i32>,
    high: Option<i32>,
) -> bool {
    let Some(node) = node.as_ref() else {
        return true;
    };
    let node = node.borrow();
    if low.is_some_and(|low| node.val <= low) || high.is_some_and(|high| node.val >= high) {
        return false;
    }
    is_search_tree(&node.left, low, Some(node.val))
        && is_search_tree(&node.right, Some(node.val), high)
}

/// The n-th Catalan number, computed by its product formula — an independent
/// route to the expected count, sharing nothing with any tree construction.
fn catalan(n: usize) -> usize {
    let mut value: u128 = 1;
    for k in 0..n as u128 {
        value = value * (2 * n as u128 - k) / (k + 1);
    }
    (value / (n as u128 + 1)) as usize
}

fn generated(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    Solution::generate_trees(n)
}

fn canonical_set(trees: &[Option<Rc<RefCell<TreeNode>>>]) -> HashSet<String> {
    trees.iter().map(canonical).collect()
}

#[test]
fn example_1() {
    let trees = generated(3);
    let expected: HashSet<String> = [
        // 1 -> 2 -> 3, leaning right.
        "(1 . (2 . (3 . .)))",
        // 1 with a right child 3 whose left child is 2.
        "(1 . (3 (2 . .) .))",
        // The balanced one.
        "(2 (1 . .) (3 . .))",
        // 3 with a left child 1 whose right child is 2.
        "(3 (1 . (2 . .)) .)",
        // 3 -> 2 -> 1, leaning left.
        "(3 (2 (1 . .) .) .)",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    assert_eq!(canonical_set(&trees), expected);
    assert_eq!(trees.len(), 5, "no tree may be listed twice");
}

#[test]
fn example_2() {
    let trees = generated(1);
    assert_eq!(
        canonical_set(&trees),
        HashSet::from(["(1 . .)".to_string()])
    );
    assert_eq!(trees.len(), 1);
}

/// n = 2 is small enough to write out in full: either value can be the root.
#[test]
fn the_two_node_case() {
    let expected: HashSet<String> = ["(1 . (2 . .))", "(2 (1 . .) .)"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(canonical_set(&generated(2)), expected);
}

/// The count is the Catalan number: 1, 1, 2, 5, 14, 42, 132, 429, 1430.
#[test]
fn the_count_is_catalan() {
    assert_eq!(
        (0..=8).map(catalan).collect::<Vec<usize>>(),
        vec![1, 1, 2, 5, 14, 42, 132, 429, 1430],
        "the helper itself is wrong"
    );

    for n in 1..=8i32 {
        assert_eq!(
            generated(n).len(),
            catalan(n as usize),
            "wrong number of trees for n = {n}"
        );
    }
}

/// Each returned tree holds 1..=n once each, arranged as a search tree.
#[test]
fn every_tree_is_a_search_tree_over_one_to_n() {
    for n in 1..=8i32 {
        let sorted: Vec<i32> = (1..=n).collect();
        for tree in generated(n) {
            assert!(tree.is_some(), "no tree with n = {n} may be empty");
            assert_eq!(node_count(&tree), n as usize, "wrong node count");

            let mut values = Vec::new();
            inorder(&tree, &mut values);
            assert_eq!(values, sorted, "not a search tree over 1..={n}");

            assert!(
                is_search_tree(&tree, None, None),
                "search tree property violated: {}",
                canonical(&tree)
            );
        }
    }
}

/// No two returned trees may be the same tree.
#[test]
fn the_trees_are_pairwise_distinct() {
    for n in 1..=8i32 {
        let trees = generated(n);
        let distinct = canonical_set(&trees);
        assert_eq!(
            distinct.len(),
            trees.len(),
            "duplicate trees returned for n = {n}"
        );
    }
}

/// Both extremes must be present: the chain leaning all the way left, the
/// chain leaning all the way right, and for n = 7 the perfectly balanced tree.
#[test]
fn the_extreme_shapes_are_present() {
    for n in 1..=8i32 {
        let trees = generated(n);
        let heights: Vec<usize> = trees.iter().map(height).collect();

        assert_eq!(
            *heights.iter().max().unwrap(),
            n as usize,
            "the degenerate chain is missing for n = {n}"
        );

        let balanced = (n as usize + 1).next_power_of_two().trailing_zeros() as usize;
        assert_eq!(
            *heights.iter().min().unwrap(),
            balanced,
            "the shallowest tree is missing for n = {n}"
        );
    }

    // Both chains, spelled out for n = 4.
    let set = canonical_set(&generated(4));
    assert!(set.contains("(1 . (2 . (3 . (4 . .))))"), "right chain");
    assert!(set.contains("(4 (3 (2 (1 . .) .) .) .)"), "left chain");
}

/// The BSTs over 1..=n are exactly the binary tree shapes with n nodes, each
/// labelled in inorder. Generating the shapes directly and labelling them is a
/// different construction that must land on the same set.
#[test]
fn the_result_matches_shapes_labelled_inorder() {
    /// All distinct shapes with `n` nodes, every node built fresh so no two
    /// trees share one and labelling one cannot disturb another.
    fn shapes(n: usize) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![None];
        }
        let mut out = Vec::new();
        for left_size in 0..n {
            for left in shapes(left_size) {
                for right in shapes(n - 1 - left_size) {
                    let node = Rc::new(RefCell::new(TreeNode::new(0)));
                    node.borrow_mut().left = deep_copy(&left);
                    node.borrow_mut().right = deep_copy(&right);
                    out.push(Some(node));
                }
            }
        }
        out
    }

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

    /// Assigns 1, 2, 3, ... to the nodes in inorder position, which turns any
    /// shape into the unique search tree with that shape.
    fn label(node: &Option<Rc<RefCell<TreeNode>>>, next: &mut i32) {
        let Some(node) = node.as_ref() else { return };
        let left = node.borrow().left.clone();
        label(&left, next);
        node.borrow_mut().val = *next;
        *next += 1;
        let right = node.borrow().right.clone();
        label(&right, next);
    }

    for n in 1..=7usize {
        let expected: HashSet<String> = shapes(n)
            .iter()
            .map(|tree| {
                let mut next = 1;
                label(tree, &mut next);
                canonical(tree)
            })
            .collect();

        assert_eq!(
            canonical_set(&generated(n as i32)),
            expected,
            "the tree set differs for n = {n}"
        );
    }
}

/// Returned trees may share nodes with each other, but a deep copy of one must
/// still be a faithful, independent tree — and mutating the copy must leave
/// every returned tree untouched.
#[test]
fn a_deep_copy_is_independent() {
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

    let trees = generated(5);
    let before = canonical_set(&trees);

    for tree in &trees {
        let copy = deep_copy(tree);
        assert_eq!(canonical(&copy), canonical(tree), "the copy differs");

        if let Some(root) = copy.as_ref() {
            root.borrow_mut().val = 99;
        }
        assert_ne!(canonical(&copy), canonical(tree), "the copy was not deep");
    }

    assert_eq!(before, canonical_set(&trees), "the originals were mutated");
}

/// The result must be usable after the call returns — nothing borrowed stays
/// borrowed, so `borrow_mut` on any node still succeeds rather than panicking.
#[test]
fn no_borrow_outlives_the_call() {
    for tree in generated(6) {
        let Some(root) = tree else { continue };
        let value = root.borrow().val;
        root.borrow_mut().val = value;
    }
}
