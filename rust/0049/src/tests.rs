use super::*;

fn words(words: &[&str]) -> Vec<String> {
    words.iter().map(ToString::to_string).collect()
}

fn normalized(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
    for group in &mut groups {
        group.sort_unstable();
    }
    groups.sort_unstable();
    groups
}

#[test]
fn example_1() {
    assert_eq!(
        normalized(Solution::group_anagrams(words(&[
            "eat", "tea", "tan", "ate", "nat", "bat",
        ]))),
        vec![
            words(&["ate", "eat", "tea"]),
            words(&["bat"]),
            words(&["nat", "tan"]),
        ]
    );
}

#[test]
fn example_2_handles_an_empty_string() {
    assert_eq!(Solution::group_anagrams(words(&[""])), vec![words(&[""])]);
}

#[test]
fn example_3_handles_one_word() {
    assert_eq!(Solution::group_anagrams(words(&["a"])), vec![words(&["a"])]);
}

#[test]
fn keeps_words_with_different_letter_counts_separate() {
    assert_eq!(
        normalized(Solution::group_anagrams(words(&[
            "ab", "baa", "aab", "abb", "baba", "ba",
        ]))),
        vec![
            words(&["aab", "baa"]),
            words(&["ab", "ba"]),
            words(&["abb"]),
            words(&["baba"]),
        ]
    );
}

#[test]
fn groups_repeated_letters_correctly() {
    assert_eq!(
        normalized(Solution::group_anagrams(words(&[
            "aabbcc", "abcabc", "ccabab", "xyz", "zyx",
        ]))),
        vec![
            words(&["aabbcc", "abcabc", "ccabab"]),
            words(&["xyz", "zyx"]),
        ]
    );
}

#[test]
fn preserves_duplicate_words() {
    assert_eq!(
        Solution::group_anagrams(words(&["listen", "silent", "listen"])),
        vec![words(&["listen", "silent", "listen"])]
    );
}
