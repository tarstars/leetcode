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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur1 = l1.as_deref();
        let mut cur2 = l2.as_deref();   
        let mut carry: i32 = 0;
        let mut head : Option<Box<ListNode>> = None;
        let mut tail : &mut Option<Box<ListNode>> = &mut head;

        while cur1.is_some() || cur2.is_some() || carry != 0 {
            let a = cur1.map_or(0, |node| node.val);
            let b = cur2.map_or(0, |node| node.val);

            let nd = (a + b + carry) % 10;
            carry = (a + b + carry) / 10;

            *tail = Some(Box::new(ListNode::new(nd)));
            tail = &mut tail.as_mut().unwrap().next;

            cur1 = cur1.and_then(|node| node.next.as_deref());
            cur2 = cur2.and_then(|node| node.next.as_deref());
        }

        head
    }
}
