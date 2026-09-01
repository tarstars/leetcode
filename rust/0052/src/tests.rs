use super::*;

#[test]
fn example_1() {
    assert_eq!(Solution::total_n_queens(4), 2);
}

#[test]
fn example_2() {
    assert_eq!(Solution::total_n_queens(1), 1);
}

#[test]
fn two_and_three_queens_have_no_solution() {
    assert_eq!(Solution::total_n_queens(2), 0);
    assert_eq!(Solution::total_n_queens(3), 0);
}

#[test]
fn returns_the_known_counts_for_larger_boards() {
    for (n, expected) in [(5, 10), (6, 4), (7, 40), (8, 92), (9, 352)] {
        assert_eq!(
            Solution::total_n_queens(n),
            expected,
            "incorrect count for n = {n}"
        );
    }
}
