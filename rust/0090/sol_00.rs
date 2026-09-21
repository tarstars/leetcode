use crate::Solution;

use std::{collections::HashMap, hash::Hash};

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subs: Vec<Vec<i32>> = Vec::new();
        let mut counted_nums: HashMap<i32, i32> = HashMap::new();

        for v in nums {
            *counted_nums.entry(v).or_insert(0) += 1;
        }

        subs.push(vec![]);

        for (k, v) in counted_nums {
            let mut subs_next: Vec<Vec<i32>> = Vec::new();

            for cand in subs {
                let mut enriched_cand = cand.clone();
                for _ in 0..=v {
                    subs_next.push(enriched_cand.clone());
                    enriched_cand.push(k);
                }
            }
            subs = subs_next;
        }

        subs
    }
}
