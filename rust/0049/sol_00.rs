use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm : HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut k: Vec<char> = s.chars().collect();
            k.sort_unstable();
            let k: String = k.iter().collect();
            hm.entry(k).or_default().push(s);
        }

        hm.into_values().collect()
    }
}
