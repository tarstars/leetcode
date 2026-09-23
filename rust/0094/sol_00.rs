use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

fn helper_inorder(root: &Option<Rc<RefCell<TreeNode>>>, inorder_values: &mut Vec<i32>) {
    if let Some(node) = root {
        let bnode = &node.borrow();
        helper_inorder(&bnode.left, inorder_values);
        inorder_values.push(bnode.val);
        helper_inorder(&bnode.right, inorder_values);
    }
}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut inorder_values: Vec<i32> = Vec::new();

        helper_inorder(&root, &mut inorder_values);

        inorder_values
    }
}
