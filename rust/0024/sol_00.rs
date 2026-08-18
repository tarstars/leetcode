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
impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut r_head = None;
        let mut r_tail = &mut r_head;

        while let Some (mut first) = head.take() {
            match first.next.take() {
                Some(mut second) => {
                    head = second.next.take();
                    *r_tail = Some(second);
                    r_tail = &mut r_tail.as_mut().unwrap().next;
                    *r_tail = Some(first);
                    r_tail = &mut r_tail.as_mut().unwrap().next;

                }
                None => {
                    *r_tail = Some(first);
                }
            }
        }

        r_head
    }
}
