use crate::Solution;

use std::collections::HashMap;

fn num_trees_helper(n: i32, hash: &mut HashMap<i32, i32>) -> i32 {
    if hash.contains_key(&n) {
        *hash.get(&n).unwrap()
    } else if n == 0 {
        1
    } else {
        let mut num_variants = 0;

        for p in 0..n {
            num_variants += num_trees_helper(p, hash) * num_trees_helper(n - 1 - p, hash);
        }

        hash.insert(n, num_variants);

        num_variants
    }
}

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut hash: HashMap<i32, i32> = HashMap::new();
        num_trees_helper(n, &mut hash)
    }
}
