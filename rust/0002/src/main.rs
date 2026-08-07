#[path = "../sol_00.rs"]
mod sol_00;

#[derive(Debug, PartialEq, Eq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

struct Solution;

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

fn from_digits(digits: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;

    for &digit in digits.iter().rev() {
        let mut node = Box::new(ListNode::new(digit));
        node.next = head;
        head = Some(node);
    }

    head
}

fn to_digits(list: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut current = list.as_ref();

    while let Some(node) = current {
        digits.push(node.val);
        current = node.next.as_ref();
    }

    digits
}

fn main() {
    let list = from_digits(&[2, 4, 3]);
    println!("List values: {:?}", to_digits(&list));

    let mut current = list.as_ref();
    while let Some(node) = current {
        println!("Current node: {}", node.val);
        current = node.next.as_ref();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traverses_every_node_in_order() {
        let list = from_digits(&[2, 4, 3]);
        assert_eq!(to_digits(&list), vec![2, 4, 3]);
    }

    #[test]
    fn handles_a_single_node() {
        let list = from_digits(&[0]);
        assert_eq!(to_digits(&list), vec![0]);
    }

    #[test]
    fn handles_an_empty_list() {
        let list = from_digits(&[]);
        assert_eq!(to_digits(&list), Vec::<i32>::new());
    }

    #[test]
    fn adds_342_and_465() {
        let sum = Solution::add_two_numbers(from_digits(&[2, 4, 3]), from_digits(&[5, 6, 4]));
        assert_eq!(to_digits(&sum), vec![7, 0, 8]);
    }

    #[test]
    fn adds_with_a_trailing_carry() {
        let sum = Solution::add_two_numbers(from_digits(&[9, 9]), from_digits(&[1]));
        assert_eq!(to_digits(&sum), vec![0, 0, 1]);
    }

    #[test]
    fn adds_lists_of_different_lengths() {
        let sum = Solution::add_two_numbers(from_digits(&[9, 9, 9, 9]), from_digits(&[9, 9]));
        assert_eq!(to_digits(&sum), vec![8, 9, 0, 0, 1]);
    }
}
