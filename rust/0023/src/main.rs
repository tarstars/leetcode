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
    let lists = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
    let merged = Solution::merge_k_lists(lists.iter().map(|v| from_values(v)).collect());
    println!("{lists:?} -> {:?}", to_values(&merged));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn merge(lists: &[&[i32]]) -> Vec<i32> {
        let built: Vec<Option<Box<ListNode>>> = lists.iter().map(|v| from_values(v)).collect();
        to_values(&Solution::merge_k_lists(built))
    }

    #[test]
    fn example_1() {
        assert_eq!(
            merge(&[&[1, 4, 5], &[1, 3, 4], &[2, 6]]),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
    }

    #[test]
    fn example_2_no_lists_at_all() {
        assert_eq!(merge(&[]), Vec::<i32>::new());
    }

    /// One list, which is empty — distinct from having no lists.
    #[test]
    fn example_3_single_empty_list() {
        assert_eq!(merge(&[&[]]), Vec::<i32>::new());
    }

    #[test]
    fn several_empty_lists() {
        assert_eq!(merge(&[&[], &[], &[]]), Vec::<i32>::new());
    }

    #[test]
    fn single_non_empty_list() {
        assert_eq!(merge(&[&[1, 2, 3]]), vec![1, 2, 3]);
    }

    /// Empty lists interleaved with real ones must be skipped, not tripped over.
    #[test]
    fn empty_lists_mixed_in() {
        assert_eq!(merge(&[&[], &[1, 3], &[], &[2], &[]]), vec![1, 2, 3]);
        assert_eq!(merge(&[&[5], &[]]), vec![5]);
        assert_eq!(merge(&[&[], &[5]]), vec![5]);
    }

    #[test]
    fn disjoint_ranges() {
        assert_eq!(
            merge(&[&[7, 8, 9], &[1, 2, 3], &[4, 5, 6]]),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn all_values_equal() {
        assert_eq!(merge(&[&[2, 2], &[2], &[2, 2, 2]]), vec![2; 6]);
    }

    #[test]
    fn negative_values() {
        assert_eq!(
            merge(&[&[-10_000, 0], &[-5_000, 10_000], &[-1]]),
            vec![-10_000, -5_000, -1, 0, 10_000]
        );
    }

    /// 10^4 lists of one node each — the constraint's maximum k. Merging them
    /// pairwise from the front is O(k * n) and will crawl; a heap or a
    /// divide-and-conquer merge is O(n log k).
    #[test]
    fn ten_thousand_single_node_lists() {
        let owned: Vec<Vec<i32>> = (0..10_000).map(|i| vec![i]).collect();
        let refs: Vec<&[i32]> = owned.iter().map(|v| v.as_slice()).collect();

        let got = merge(&refs);
        assert_eq!(got.len(), 10_000);
        assert_eq!(got[0], 0);
        assert_eq!(got[9_999], 9_999);
        assert!(got.windows(2).all(|w| w[0] <= w[1]), "result is not sorted");
    }

    /// 20 lists of 500 nodes each: the same 10^4 total, shaped the other way.
    #[test]
    fn twenty_long_lists() {
        let owned: Vec<Vec<i32>> = (0..20)
            .map(|k| (0..500).map(|i| i * 20 + k).collect())
            .collect();
        let refs: Vec<&[i32]> = owned.iter().map(|v| v.as_slice()).collect();

        let got = merge(&refs);
        assert_eq!(got.len(), 10_000);
        assert_eq!(got, (0..10_000).collect::<Vec<i32>>());
    }
}
