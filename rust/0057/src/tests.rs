use super::*;

#[test]
fn example_1() {
    assert_eq!(
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        vec![vec![1, 5], vec![6, 9]]
    );
}

#[test]
fn example_2() {
    assert_eq!(
        Solution::insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16]
            ],
            vec![4, 8],
        ),
        vec![vec![1, 2], vec![3, 10], vec![12, 16]]
    );
}

#[test]
fn inserts_into_an_empty_list() {
    assert_eq!(Solution::insert(vec![], vec![4, 7]), vec![vec![4, 7]]);
}

#[test]
fn inserts_before_or_after_all_intervals() {
    assert_eq!(
        Solution::insert(vec![vec![3, 5], vec![8, 10]], vec![0, 1]),
        vec![vec![0, 1], vec![3, 5], vec![8, 10]]
    );
    assert_eq!(
        Solution::insert(vec![vec![1, 2], vec![4, 6]], vec![8, 9]),
        vec![vec![1, 2], vec![4, 6], vec![8, 9]]
    );
}

#[test]
fn handles_containment_in_either_direction() {
    assert_eq!(
        Solution::insert(vec![vec![1, 10]], vec![3, 5]),
        vec![vec![1, 10]]
    );
    assert_eq!(
        Solution::insert(vec![vec![3, 4], vec![6, 7]], vec![1, 10]),
        vec![vec![1, 10]]
    );
}

#[test]
fn merges_intervals_that_touch_at_an_endpoint() {
    assert_eq!(
        Solution::insert(vec![vec![1, 2], vec![5, 7]], vec![2, 5]),
        vec![vec![1, 7]]
    );
}

#[test]
fn preserves_intervals_on_both_sides_of_the_merge() {
    assert_eq!(
        Solution::insert(
            vec![vec![0, 1], vec![3, 5], vec![7, 8], vec![10, 12]],
            vec![4, 11],
        ),
        vec![vec![0, 1], vec![3, 12]]
    );
}

#[test]
fn handles_the_maximum_number_of_intervals() {
    let intervals = (0..10_000)
        .map(|value| vec![value * 2, value * 2])
        .collect();
    assert_eq!(
        Solution::insert(intervals, vec![0, 19_998]),
        vec![vec![0, 19_998]]
    );
}
