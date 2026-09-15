use super::*;

fn board(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|row| row.chars().collect()).collect()
}

fn exists(rows: &[&str], word: &str) -> bool {
    Solution::exist(board(rows), word.to_owned())
}

#[test]
fn example_1() {
    assert!(exists(&["ABCE", "SFCS", "ADEE"], "ABCCED"));
}

#[test]
fn example_2() {
    assert!(exists(&["ABCE", "SFCS", "ADEE"], "SEE"));
}

#[test]
fn example_3() {
    assert!(!exists(&["ABCE", "SFCS", "ADEE"], "ABCB"));
}

#[test]
fn single_cell() {
    assert!(exists(&["a"], "a"));
    assert!(!exists(&["a"], "A"));
    assert!(!exists(&["a"], "aa"));
}

#[test]
fn straight_paths_work_in_both_directions() {
    assert!(exists(&["ABCDE"], "ABCDE"));
    assert!(exists(&["ABCDE"], "EDCBA"));
    assert!(exists(&["A", "B", "C", "D", "E"], "ABCDE"));
    assert!(exists(&["A", "B", "C", "D", "E"], "EDCBA"));
}

#[test]
fn paths_may_turn() {
    let spiral = ["ABC", "HGD", "IFE"];
    assert!(exists(&spiral, "ABCDEFGHI"));
    assert!(exists(&spiral, "IHGFEDCBA"));
}

#[test]
fn paths_cannot_move_diagonally_or_wrap_between_rows() {
    let rows = ["AB", "CD"];
    assert!(!exists(&rows, "AD"));
    assert!(!exists(&rows, "BC"));
}

#[test]
fn a_cell_cannot_be_reused() {
    assert!(!exists(&["AB", "CD"], "ABA"));
    assert!(!exists(&["AB", "CD"], "ACDBA"));

    assert!(exists(&["AA", "AA"], "AAAA"));
    assert!(!exists(&["AA", "AA"], "AAAAA"));
}

#[test]
fn search_can_start_at_any_cell() {
    assert!(exists(&["XYZ", "ABC"], "ABC"));
    assert!(exists(&["XYZ", "ABC"], "ZYX"));
}

#[test]
fn matching_is_case_sensitive() {
    assert!(exists(&["aB", "Cd"], "aBdC"));
    assert!(!exists(&["aB", "Cd"], "ABdC"));
}

#[test]
fn failed_branch_must_not_poison_later_search() {
    let rows = ["ABCE", "SFES", "ADEE"];
    assert!(exists(&rows, "ABCESEEEFS"));
}

#[test]
fn maximum_board_and_word_length() {
    let rows = ["ABCDEF", "LKJIHG", "MNOPQR", "XWVUTS", "YZabcd", "jihgfe"];
    assert!(exists(&rows, "ABCDEFGHIJKLMNO"));
}
