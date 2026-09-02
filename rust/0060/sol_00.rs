use crate::Solution;

fn next_permutation(p: &mut Vec<i32>) {
    let n = p.len();

    let pivot = p.windows(2).rposition(|w| w[0] < w[1]).unwrap();
    let gt = (pivot + 1..n).rfind(|ind| p[*ind] > p[pivot as usize]).unwrap();
    p.swap(pivot as usize, gt as usize);
    p[pivot+1..n].reverse();
}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut perm: Vec<i32> = (1..=n).collect();

        for _ in 0..(k - 1) {
            next_permutation(&mut perm);
        }

        perm.iter().map(|x| x.to_string()).collect()
    }
}
