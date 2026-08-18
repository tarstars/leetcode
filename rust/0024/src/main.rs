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
    for values in [vec![1, 2, 3, 4], vec![], vec![1], vec![1, 2, 3]] {
        let swapped = Solution::swap_pairs(from_values(&values));
        println!("{values:?} -> {:?}", to_values(&swapped));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn swap(values: &[i32]) -> Vec<i32> {
        to_values(&Solution::swap_pairs(from_values(values)))
    }

    #[test]
    fn example_1_even_length() {
        assert_eq!(swap(&[1, 2, 3, 4]), vec![2, 1, 4, 3]);
    }

    #[test]
    fn example_2_empty() {
        assert_eq!(swap(&[]), Vec::<i32>::new());
    }

    /// A lone node has no partner and stays put.
    #[test]
    fn example_3_single_node() {
        assert_eq!(swap(&[1]), vec![1]);
    }

    /// The odd one out is the last node, which must survive unswapped.
    #[test]
    fn example_4_odd_length() {
        assert_eq!(swap(&[1, 2, 3]), vec![2, 1, 3]);
    }

    #[test]
    fn exactly_two_nodes() {
        assert_eq!(swap(&[1, 2]), vec![2, 1]);
    }

    #[test]
    fn five_nodes() {
        assert_eq!(swap(&[1, 2, 3, 4, 5]), vec![2, 1, 4, 3, 5]);
    }

    /// Equal values still swap positions — the test can't see that directly,
    /// but the length and order of the distinct neighbours must hold.
    #[test]
    fn repeated_values() {
        assert_eq!(swap(&[7, 7, 7, 7]), vec![7, 7, 7, 7]);
        assert_eq!(swap(&[1, 1, 2, 2, 3]), vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn zero_values() {
        assert_eq!(swap(&[0, 0, 0]), vec![0, 0, 0]);
    }

    /// Swapping twice restores the original list.
    #[test]
    fn swapping_twice_is_the_identity() {
        for len in 0..12 {
            let values: Vec<i32> = (0..len).collect();
            let once = Solution::swap_pairs(from_values(&values));
            let twice = Solution::swap_pairs(once);
            assert_eq!(to_values(&twice), values, "len = {len}");
        }
    }

    /// 100 nodes — the constraint's maximum.
    #[test]
    fn longest_allowed_list() {
        let values: Vec<i32> = (0..100).collect();
        let want: Vec<i32> = (0..50).flat_map(|i| [i * 2 + 1, i * 2]).collect();
        assert_eq!(swap(&values), want);
    }

    /// 99 nodes: the maximum with an odd tail.
    #[test]
    fn odd_length_at_the_maximum() {
        let values: Vec<i32> = (0..99).collect();
        let mut want: Vec<i32> = (0..49).flat_map(|i| [i * 2 + 1, i * 2]).collect();
        want.push(98);
        assert_eq!(swap(&values), want);
    }
}
