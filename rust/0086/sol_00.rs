use crate::{ListNode, Solution};

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut it = &mut head;
        let mut head_ge: Option<Box<ListNode>> = None;
        let mut tail: &mut Option<Box<ListNode>> = &mut head_ge;

        while it.is_some() {
            if it.as_ref().unwrap().val >= x {
                *tail = it.take();
                tail = &mut tail.as_mut().unwrap().next;
                *it = tail.take();
            } else {
                it = &mut it.as_mut().unwrap().next;
            }
        }

        *it = head_ge;

        head
    }
}
