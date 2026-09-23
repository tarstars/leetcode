#[path = "../sol_00.rs"]
mod sol_00;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution;

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

/// Renders a tree in LeetCode's level-order array notation, trailing absent
/// nodes trimmed. Only for printing — the tests compare structure instead.
fn to_level_order(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut out = Vec::new();
    let mut pending = std::collections::VecDeque::new();
    pending.push_back(root.clone());

    while let Some(slot) = pending.pop_front() {
        match slot {
            None => out.push(None),
            Some(node) => {
                let node = node.borrow();
                out.push(Some(node.val));
                pending.push_back(node.left.clone());
                pending.push_back(node.right.clone());
            }
        }
    }

    while matches!(out.last(), Some(None)) {
        out.pop();
    }
    out
}

fn main() {
    for n in 1..=4 {
        let trees = Solution::generate_trees(n);
        println!("n = {n} -> {} trees", trees.len());
        for tree in &trees {
            println!("  {:?}", to_level_order(tree));
        }
    }
}

#[cfg(test)]
mod tests;
