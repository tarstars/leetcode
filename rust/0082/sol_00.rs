use crate::{ListNode, Solution};

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut it = &mut head;

        while it.is_some() {
            if it.as_ref().unwrap().next.is_some() && it.as_ref().unwrap().val == it.as_ref().unwrap().next.as_ref().unwrap().val {
                let v = it.as_ref().unwrap().val;
                while it.is_some() && it.as_ref().unwrap().val == v {
                    *it = it.take().unwrap().next;
                }
            } else {
                it = &mut it.as_mut().unwrap().next;
            }
        }

        head
    }
}
