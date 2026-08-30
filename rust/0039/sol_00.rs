use crate::Solution;

fn combination_helper(
        all_options: &mut Vec<Vec<i32>>,
        candidates: &mut Vec<i32>,
        mut target: i32,
        path: &mut Vec<i32>) {
    if target == 0 {
        all_options.push(path.clone());
        return;
    }
    let initial_length = path.len();
    if let Some(current_candidate) = candidates.pop() {
        while target >= 0 {
            combination_helper(all_options, candidates, target, path);
            target -= current_candidate;
            path.push(current_candidate);
        }
        candidates.push(current_candidate);
    }
    path.truncate(initial_length);
}

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut all_options = Vec::new();
        let mut path = Vec::new();

        candidates.sort_unstable();
        candidates.reverse();

        combination_helper(&mut all_options, &mut candidates, target, &mut path);

        all_options
    }
}
