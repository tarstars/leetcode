use crate::Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combinations: Vec<Vec<i32>> = Vec::new();
        let mut combination: Vec<i32> = Vec::with_capacity(k as usize);

        for p in 1..=k {
            combination.push(p);
        }

        if k > n {
            return combinations;
        }

        combinations.push(combination.clone());

        loop {
            if *combination.last().unwrap() < n {
                *combination.last_mut().unwrap() += 1;
            } else if let Some(pivot) = combination.windows(2).rposition(|w| w[0] < w[1] - 1) {
                combination[pivot] += 1;
                let base = combination[pivot];
                for p in pivot + 1..k as usize {
                    combination[p] = base + (p - pivot) as i32;
                }
            } else {
                break;
            }
            combinations.push(combination.clone());
        }

        combinations
    }
}
