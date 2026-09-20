use crate::Solution;

use std::collections::HashMap;

fn is_scramble_helper(hm: &mut HashMap<(String, String), bool>, s1: &str, s2: &str) -> bool {
    if s1.len() == 0 {
        return true;
    }

    if s1 == s2 {
        return true;
    }

    if hm.contains_key(&(s1.to_string(), s2.to_string())) {
        return *hm.get(&(s1.to_string(), s2.to_string())).unwrap();
    }

    let mut can_split = false;

    for p in 1..s1.len() {
        let ls1 = s1[0..p].to_string();
        let rs1 = s1[p..].to_string();
        let ls2 = s2[0..p].to_string();
        let rs2 = s2[p..].to_string();
        let lsm2 = s2[0..s2.len() - p].to_string();
        let rsm2 = s2[s2.len() - p..].to_string();

        if is_scramble_helper(hm, &ls1, &ls2) && is_scramble_helper(hm, &rs1, &rs2)
            || is_scramble_helper(hm, &ls1, &rsm2) && is_scramble_helper(hm, &rs1, &lsm2)
        {
            can_split = true;
        }
    }

    hm.insert((s1.to_string(), s2.to_string()), can_split);

    can_split
}

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut hm: HashMap<(String, String), bool> = HashMap::new();

        is_scramble_helper(&mut hm, &s1, &s2)
    }
}
