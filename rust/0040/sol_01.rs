use crate::Solution;

fn search(
    candidates: &[i32],
    start: usize,
    remaining: i32,
    combination: &mut Vec<i32>,
    results: &mut Vec<Vec<i32>>,
) {
    if remaining == 0 {
        results.push(combination.clone());
        return;
    }

    for index in start..candidates.len() {
        let candidate = candidates[index];

        if index > start && candidate == candidates[index - 1] {
            continue;
        }
        if candidate > remaining {
            break;
        }

        combination.push(candidate);
        search(
            candidates,
            index + 1,
            remaining - candidate,
            combination,
            results,
        );
        combination.pop();
    }
}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        let mut results = Vec::new();
        search(&candidates, 0, target, &mut Vec::new(), &mut results);
        results
    }
}
