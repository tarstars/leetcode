use std::collections::HashMap;

use crate::Solution;

fn gp_helper(n: i32, hash: &mut HashMap<i32, Vec<String>>) -> Vec<String> {
        if n == 0 {
            return vec![String::new()];
        }

        if let Some(v) = hash.get(&n) {
            return v.clone();
        }

        let mut ret: Vec<String> = Vec::new();

        for disp in 0..n {
            for inner in gp_helper(disp, hash) {
                for outer in gp_helper(n - 1 - disp, hash) {
                    ret.push(format!("({inner}){outer}"));
                }
            }
        }

        hash.insert(n, ret.clone());

        ret

}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut hash: HashMap<i32, Vec<String>> = HashMap::new();
        gp_helper(n, &mut hash)
    }
}
