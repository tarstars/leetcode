use crate::{ListNode, Solution};

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

impl Solution {
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut it: &mut Option<Box<ListNode>> = &mut head;

        for _ in 0..left - 1 {
            it = &mut it.as_mut().unwrap().next;
        }

        let mut mid_head = it.take();
        it = &mut mid_head;

        for _ in 0..right - left + 1 {
            it = &mut it.as_mut().unwrap().next;
        }

        let mut last_head = it.take();

        let mut rev_head: Option<Box<ListNode>> = None;

        while let Some(mut node) = mid_head {
            mid_head = node.next.take();
            node.next = rev_head.take();
            rev_head = Some(node);
        }

        it = &mut head;
        while it.is_some() {
            it = &mut it.as_mut().unwrap().next;
        }

        *it = rev_head.take();

        while it.is_some() {
            it = &mut it.as_mut().unwrap().next;
        }

        *it = last_head;

        head
    }
}
