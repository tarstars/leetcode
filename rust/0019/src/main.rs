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
    for (values, n) in [(vec![1, 2, 3, 4, 5], 2), (vec![1], 1), (vec![1, 2], 1)] {
        let got = Solution::remove_nth_from_end(from_values(&values), n);
        println!("{values:?} remove {n}th from end -> {:?}", to_values(&got));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn remove(values: &[i32], n: i32) -> Vec<i32> {
        to_values(&Solution::remove_nth_from_end(from_values(values), n))
    }

    #[test]
    fn example_1_middle_node() {
        assert_eq!(remove(&[1, 2, 3, 4, 5], 2), vec![1, 2, 3, 5]);
    }

    /// Removing the only node leaves an empty list.
    #[test]
    fn example_2_single_node() {
        assert_eq!(remove(&[1], 1), Vec::<i32>::new());
    }

    #[test]
    fn example_3_last_node() {
        assert_eq!(remove(&[1, 2], 1), vec![1]);
    }

    /// n == length means the head goes, which is the case that needs a dummy
    /// node or a separate branch.
    #[test]
    fn removes_the_head() {
        assert_eq!(remove(&[1, 2], 2), vec![2]);
        assert_eq!(remove(&[1, 2, 3, 4, 5], 5), vec![2, 3, 4, 5]);
    }

    #[test]
    fn removes_the_tail() {
        assert_eq!(remove(&[1, 2, 3, 4, 5], 1), vec![1, 2, 3, 4]);
    }

    #[test]
    fn two_nodes_each_way() {
        assert_eq!(remove(&[7, 9], 1), vec![7]);
        assert_eq!(remove(&[7, 9], 2), vec![9]);
    }

    #[test]
    fn duplicate_values_are_positional_not_by_value() {
        assert_eq!(remove(&[5, 5, 5, 5], 3), vec![5, 5, 5]);
        assert_eq!(remove(&[1, 2, 1, 2, 1], 4), vec![1, 1, 2, 1]);
    }

    #[test]
    fn zero_values_are_allowed() {
        assert_eq!(remove(&[0, 0, 0], 2), vec![0, 0]);
    }

    /// 30 nodes — the constraint's maximum — removed from each end.
    #[test]
    fn longest_allowed_list() {
        let values: Vec<i32> = (1..=30).collect();

        let mut want_first: Vec<i32> = (2..=30).collect();
        assert_eq!(remove(&values, 30), want_first);

        want_first = (1..=29).collect();
        assert_eq!(remove(&values, 1), want_first);

        let want_middle: Vec<i32> = (1..=30).filter(|&v| v != 15).collect();
        assert_eq!(remove(&values, 16), want_middle);
    }

    /// Every position in a 6-node list, checked against a Vec-based reference.
    #[test]
    fn every_position() {
        let values = [10, 20, 30, 40, 50, 60];
        for n in 1..=6 {
            let mut want = values.to_vec();
            want.remove(values.len() - n as usize);
            assert_eq!(remove(&values, n), want, "n = {n}");
        }
    }
}
