use super::*;

fn length(s: &str) -> i32 {
    Solution::length_of_last_word(s.to_owned())
}

#[test]
fn example_1() {
    assert_eq!(length("Hello World"), 5);
}

#[test]
fn example_2() {
    assert_eq!(length("   fly me   to   the moon  "), 4);
}

#[test]
fn example_3() {
    assert_eq!(length("luffy is still joyboy"), 6);
}

#[test]
fn handles_a_single_word() {
    assert_eq!(length("rust"), 4);
}

#[test]
fn ignores_leading_and_trailing_spaces() {
    assert_eq!(length("     answer     "), 6);
}

#[test]
fn ignores_multiple_spaces_between_words() {
    assert_eq!(length("one          three"), 5);
}

#[test]
fn handles_a_one_letter_last_word() {
    assert_eq!(length("longer words x"), 1);
}

#[test]
fn handles_the_maximum_string_length() {
    assert_eq!(Solution::length_of_last_word("a".repeat(10_000)), 10_000);
}
