use crate::Solution;

fn get(a: &Vec<Vec<i32>>, ind: i32) -> i32 {
    let ind = ind as usize;
    a[ind / a[0].len()][ind % a[0].len()]
}

fn binary_search(mut l: i32, mut r: i32, p: impl Fn(i32) -> bool) -> i32 {
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if p(m) {
            r = m;
        } else {
            l = m;
        }
    }
    r
}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len() * matrix[0].len();

        let p = binary_search(-1, n as i32, |ind| get(&matrix, ind) >= target);

        p < n as i32 && get(&matrix, p) == target
    }
}
