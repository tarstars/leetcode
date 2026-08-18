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

fn list_len(mut head: &Option<Box<ListNode>>) -> i32 {
    let mut s = 0;

    while let Some(node) = head {
        s += 1;
        head = &node.next;
    }

    s
}

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut res_head: Option<Box<ListNode>> = None;
        let mut res_tail = &mut res_head;

        let mut s = list_len(&head);

        while s >= k {
            let mut chunk_head: Option<Box<ListNode>> = None;

            for _i in 0..k {
                let mut first = head.take().unwrap();
                head = first.next.take();

                first.next = chunk_head;
                chunk_head = Some(first);                    
            }
            *res_tail = chunk_head;
            s -= k;
            while res_tail.is_some() {
                res_tail = &mut res_tail.as_mut().unwrap().next;
            }
        }

        *res_tail = head;

        return res_head
    }
}
