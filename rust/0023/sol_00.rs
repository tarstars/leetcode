use crate::{ListNode, Solution};

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();

        for (i, head) in lists.iter().enumerate() {
            if let Some(node) = head {
                heap.push(Reverse((node.val, i)));
            }
        }

        let mut head: Option<Box<ListNode>> = None;
        let mut tail= &mut head;

        while let Some(Reverse((_val, i))) = heap.pop() {
            let mut new_node = lists[i].take().unwrap();
            lists[i] = new_node.next.take();
            if let Some(next) = &lists[i] {
                heap.push(Reverse((next.val, i)));
            }
            *tail = Some(new_node);
            tail = &mut tail.as_mut().unwrap().next; 
        }

        head
    }
}
