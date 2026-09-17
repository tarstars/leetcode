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
            let node = it.as_mut().unwrap();
            if node.next.is_some() && node.next.as_ref().unwrap().val == node.val {
                node.next = node.next.take().unwrap().next;
            } else {
                it = &mut it.as_mut().unwrap().next;
            }
        }

        head
    }
}
