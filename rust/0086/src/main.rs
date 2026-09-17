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

fn from_values(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;

    for &value in values.iter().rev() {
        let mut node = Box::new(ListNode::new(value));
        node.next = head;
        head = Some(node);
    }

    head
}

fn to_values(list: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut values = Vec::new();
    let mut current = list.as_ref();

    while let Some(node) = current {
        values.push(node.val);
        current = node.next.as_ref();
    }

    values
}

fn main() {
    for (values, x) in [
        (vec![1, 4, 3, 2, 5, 2], 3),
        (vec![2, 1], 2),
        (vec![], 0),
        (vec![3, 3, 3], 3),
    ] {
        let split = Solution::partition(from_values(&values), x);
        println!("{values:?}, x = {x} -> {:?}", to_values(&split));
    }
}

#[cfg(test)]
mod tests;
