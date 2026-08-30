use crate::Solution;

use std::collections::HashSet;

fn fill_combinations(
    candidates: &mut Vec<i32>,
    combinations: &mut HashSet<Vec<i32>>,
    path: &mut Vec<i32>,
    target: i32
) {
    if target == 0 {
        let mut sorted_path = path.clone();
        sorted_path.sort();
        combinations.insert(sorted_path);
        return;
    }

    if let Some(v) = candidates.pop() {
        fill_combinations(candidates, combinations, path, target);

        if target >= v {
            path.push(v);

            fill_combinations(candidates, combinations, path, target - v);

            path.pop();
        }
        candidates.push(v);
    }
}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combinations: HashSet<Vec<i32>> = HashSet::new();
        let mut path: Vec<i32> = Vec::new();

        fill_combinations(&mut candidates, &mut combinations, &mut path, target);

        combinations.into_iter().collect()
    }
}
