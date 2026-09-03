use crate::{ListNode, Solution};

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut n = 0;

        if head.is_none() {
            return head;
        }

        let mut it = head.as_ref();
        while let Some(node) = it {
            n += 1;
            it = node.next.as_ref();
        }

        let mut break_point = head.as_mut();

        for _ in 0..(n - (k % n) - 1) {
            break_point = break_point.unwrap().next.as_mut();
        }

        let mut second_head = break_point.unwrap().next.take();
        let mut it = &mut second_head;

        while let Some(node) = it {
            it = &mut node.next;
        }

        *it = head;

        second_head
    }
}
