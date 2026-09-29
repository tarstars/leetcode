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

fn generate_trees_helper(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if r < l {
        vec![None]
    } else {
        let mut all_trees: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

        for p in l..=r {
            let left_side = generate_trees_helper(l, p - 1);
            let right_side = generate_trees_helper(p + 1, r);

            for left_side_tree in left_side.iter() {
                for right_side_tree in right_side.iter() {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(p)));
                    {
                        let mut new_node_borrowed = new_node.borrow_mut();
                        new_node_borrowed.left = left_side_tree.clone();
                        new_node_borrowed.right = right_side_tree.clone();
                        drop(new_node_borrowed);
                        all_trees.push(Some(new_node));
                    }
                }
            }
        }

        all_trees
    }
}

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        generate_trees_helper(1, n)
    }
}
