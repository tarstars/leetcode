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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result_head: Option<Box<ListNode>> = None;
        let mut result_tail = &mut result_head;

        loop {
            match (list1.take(), list2.take()) {
                (Some(mut node1), Some(mut node2)) => {
                    if node1.val < node2.val {
                        list1 = node1.next.take();
                        list2 = Some(node2);
                        *result_tail = Some(node1);
                    } else {
                        list2 = node2.next.take();
                        list1 = Some(node1);
                        *result_tail = Some(node2);
                    }
                    result_tail = &mut result_tail.as_mut().unwrap().next;
                }   
                (None, Some(node)) | (Some(node), None) => {
                    *result_tail = Some(node);
                    break;                    
                }   
                _ => {
                    break;
                }       
            }
        }

        return result_head;
    }
}
