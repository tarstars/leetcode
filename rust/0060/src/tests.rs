use super::*;

#[test]
fn example_1() {
    assert_eq!(Solution::get_permutation(3, 3), "213");
}

#[test]
fn example_2() {
    assert_eq!(Solution::get_permutation(4, 9), "2314");
}

#[test]
fn example_3() {
    assert_eq!(Solution::get_permutation(3, 1), "123");
}

#[test]
fn handles_the_smallest_input() {
    assert_eq!(Solution::get_permutation(1, 1), "1");
}

#[test]
fn returns_every_three_digit_permutation_in_order() {
    let expected = ["123", "132", "213", "231", "312", "321"];
    for (index, permutation) in expected.into_iter().enumerate() {
        assert_eq!(Solution::get_permutation(3, index as i32 + 1), permutation);
    }
}

#[test]
fn handles_the_first_and_last_permutations() {
    assert_eq!(Solution::get_permutation(4, 1), "1234");
    assert_eq!(Solution::get_permutation(4, 24), "4321");
}

#[test]
fn handles_the_maximum_input() {
    assert_eq!(Solution::get_permutation(9, 362_880), "987654321");
}
