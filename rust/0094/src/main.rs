#[path = "../sol_00.rs"]
mod sol_00;

use std::cell::RefCell;
use std::collections::VecDeque;
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

/// Builds a tree from LeetCode's level-order array notation, where `None`
/// marks an absent node and the children of absent nodes are simply not
/// listed. `[1, None, 2, 3]` is a root with only a right child, which in turn
/// has only a left child.
fn from_level_order(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut remaining = values.iter();

    let root = match remaining.next() {
        Some(Some(value)) => Rc::new(RefCell::new(TreeNode::new(*value))),
        _ => return None,
    };

    let mut pending = VecDeque::new();
    pending.push_back(Rc::clone(&root));

    while let Some(node) = pending.pop_front() {
        if let Some(Some(value)) = remaining.next() {
            let child = Rc::new(RefCell::new(TreeNode::new(*value)));
            node.borrow_mut().left = Some(Rc::clone(&child));
            pending.push_back(child);
        }
        if let Some(Some(value)) = remaining.next() {
            let child = Rc::new(RefCell::new(TreeNode::new(*value)));
            node.borrow_mut().right = Some(Rc::clone(&child));
            pending.push_back(child);
        }
    }

    Some(root)
}

fn main() {
    let trees: [&[Option<i32>]; 4] = [
        &[Some(1), None, Some(2), Some(3)],
        &[],
        &[Some(1)],
        &[Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7)],
    ];

    for values in trees {
        let root = from_level_order(values);
        println!(
            "{values:?} -> {:?}",
            Solution::inorder_traversal(root)
        );
    }
}

#[cfg(test)]
mod tests;
