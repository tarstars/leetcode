use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for word in strs {
            let mut letter_counts = [0_u8; 26];
            for letter in word.bytes() {
                letter_counts[usize::from(letter - b'a')] += 1;
            }

            groups.entry(letter_counts).or_default().push(word);
        }

        groups.into_values().collect()
    }
}
