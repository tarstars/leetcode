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
    for (values, k) in [
        (vec![1, 2, 3, 4, 5], 2),
        (vec![1, 2, 3, 4, 5], 3),
        (vec![1], 1),
        (vec![1, 2, 3, 4], 4),
    ] {
        let reversed = Solution::reverse_k_group(from_values(&values), k);
        println!("{values:?}, k = {k} -> {:?}", to_values(&reversed));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reverse(values: &[i32], k: i32) -> Vec<i32> {
        to_values(&Solution::reverse_k_group(from_values(values), k))
    }

    /// The reference implementation the property tests compare against.
    fn expected(values: &[i32], k: i32) -> Vec<i32> {
        let k = k as usize;
        values
            .chunks(k)
            .flat_map(|chunk| {
                let mut chunk = chunk.to_vec();
                if chunk.len() == k {
                    chunk.reverse();
                }
                chunk
            })
            .collect()
    }

    #[test]
    fn example_1_pairs() {
        assert_eq!(reverse(&[1, 2, 3, 4, 5], 2), vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn example_2_triples() {
        assert_eq!(reverse(&[1, 2, 3, 4, 5], 3), vec![3, 2, 1, 4, 5]);
    }

    /// k = 1 must leave the list untouched.
    #[test]
    fn k_of_one_is_the_identity() {
        assert_eq!(reverse(&[1, 2, 3, 4, 5], 1), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn single_node() {
        assert_eq!(reverse(&[7], 1), vec![7]);
    }

    /// k equal to the length reverses the whole list.
    #[test]
    fn k_equals_length() {
        assert_eq!(reverse(&[1, 2, 3, 4], 4), vec![4, 3, 2, 1]);
    }

    /// The final short group keeps its original order.
    #[test]
    fn trailing_remainder_is_untouched() {
        assert_eq!(reverse(&[1, 2, 3, 4, 5, 6, 7], 3), vec![3, 2, 1, 6, 5, 4, 7]);
        assert_eq!(reverse(&[1, 2, 3, 4, 5], 4), vec![4, 3, 2, 1, 5]);
    }

    /// Equal values can't reveal the reordering, but the length must survive:
    /// dropped or duplicated nodes show up here.
    #[test]
    fn repeated_values() {
        assert_eq!(reverse(&[9, 9, 9, 9, 9], 2), vec![9; 5]);
    }

    #[test]
    fn zero_values() {
        assert_eq!(reverse(&[0, 0, 0], 2), vec![0, 0, 0]);
    }

    /// Reversing k-groups twice restores the original list.
    #[test]
    fn reversing_twice_is_the_identity() {
        for len in 1..20usize {
            let values: Vec<i32> = (0..len as i32).collect();
            for k in 1..=len as i32 {
                let once = Solution::reverse_k_group(from_values(&values), k);
                let twice = Solution::reverse_k_group(once, k);
                assert_eq!(to_values(&twice), values, "len = {len}, k = {k}");
            }
        }
    }

    /// Every (len, k) pair up to a modest size, against the reference.
    #[test]
    fn matches_reference_for_all_small_inputs() {
        for len in 1..25usize {
            let values: Vec<i32> = (0..len as i32).collect();
            for k in 1..=len as i32 {
                assert_eq!(
                    reverse(&values, k),
                    expected(&values, k),
                    "len = {len}, k = {k}"
                );
            }
        }
    }

    /// 5000 nodes — the constraint's maximum, and deep enough that a
    /// recursive solution risks blowing the stack.
    #[test]
    fn longest_allowed_list() {
        let values: Vec<i32> = (0..5000).map(|i| i % 1001).collect();
        for k in [1, 2, 3, 7, 4999, 5000] {
            assert_eq!(reverse(&values, k), expected(&values, k), "k = {k}");
        }
    }
}
