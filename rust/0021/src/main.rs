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
    for (a, b) in [(vec![1, 2, 4], vec![1, 3, 4]), (vec![], vec![]), (vec![], vec![0])] {
        let merged = Solution::merge_two_lists(from_values(&a), from_values(&b));
        println!("{a:?} + {b:?} -> {:?}", to_values(&merged));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
        to_values(&Solution::merge_two_lists(from_values(a), from_values(b)))
    }

    #[test]
    fn example_1() {
        assert_eq!(merge(&[1, 2, 4], &[1, 3, 4]), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn example_2_both_empty() {
        assert_eq!(merge(&[], &[]), Vec::<i32>::new());
    }

    #[test]
    fn example_3_one_empty() {
        assert_eq!(merge(&[], &[0]), vec![0]);
        assert_eq!(merge(&[0], &[]), vec![0]);
    }

    /// Every element of one list precedes every element of the other.
    #[test]
    fn disjoint_ranges() {
        assert_eq!(merge(&[1, 2, 3], &[4, 5, 6]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(merge(&[4, 5, 6], &[1, 2, 3]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn strictly_interleaved() {
        assert_eq!(merge(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    /// One list runs out long before the other.
    #[test]
    fn very_uneven_lengths() {
        assert_eq!(merge(&[0], &[1, 2, 3, 4, 5]), vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(merge(&[1, 2, 3, 4, 5], &[6]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(merge(&[9], &[1, 2, 3]), vec![1, 2, 3, 9]);
    }

    #[test]
    fn all_values_equal() {
        assert_eq!(merge(&[2, 2, 2], &[2, 2]), vec![2, 2, 2, 2, 2]);
    }

    #[test]
    fn negative_values() {
        assert_eq!(merge(&[-100, -3, 0], &[-50, -1, 100]), vec![-100, -50, -3, -1, 0, 100]);
    }

    /// 50 nodes in each list — the constraint's maximum.
    #[test]
    fn maximum_lengths() {
        let evens: Vec<i32> = (0..50).map(|i| i * 2 - 50).collect();
        let odds: Vec<i32> = (0..50).map(|i| i * 2 - 49).collect();

        let mut want: Vec<i32> = evens.iter().chain(odds.iter()).copied().collect();
        want.sort();

        assert_eq!(merge(&evens, &odds), want);
        assert_eq!(merge(&evens, &evens), {
            let mut w: Vec<i32> = evens.iter().chain(evens.iter()).copied().collect();
            w.sort();
            w
        });
    }

    /// The result must reuse the nodes, so the total length is always the sum.
    #[test]
    fn length_is_always_the_sum() {
        for a_len in 0..6 {
            for b_len in 0..6 {
                let a: Vec<i32> = (0..a_len).map(|i| i * 3).collect();
                let b: Vec<i32> = (0..b_len).map(|i| i * 2).collect();
                assert_eq!(
                    merge(&a, &b).len(),
                    a_len as usize + b_len as usize,
                    "{a:?} + {b:?}"
                );
            }
        }
    }
}
