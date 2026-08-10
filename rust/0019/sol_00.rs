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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut it = head.as_ref();

        let mut s = 0;
        while let Some(node) = it {
            s += 1;
            it = node.next.as_ref();
        }

        let index_remove = s - n;

        if index_remove == 0 {
            return head.unwrap().next;
        }

        let mut cur = &mut head;

        for _ in 0..index_remove {
            cur = &mut cur.as_mut().unwrap().next;
        }

        *cur = cur.take().unwrap().next;

        head
    }
}
