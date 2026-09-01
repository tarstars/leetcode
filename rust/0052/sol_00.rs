use crate::Solution;

fn last_match(path: &Vec<i32>) -> bool {
    let n = path.len();

    for p in 0..(n - 1) {
        if path[p] == path[n - 1] || (n - 1 - p) == (path[p] - path[n - 1]).abs() as usize {
            return false;
        }
    }

    true
}

fn search(path: &mut Vec<i32>, s: &mut i32, n: i32) {
    if path.len() == n as usize {
        *s += 1;
        return
    }

    for d in 0..n {
        path.push(d);

        if last_match(path) {
            search(path, s, n);
        }

        path.pop();
    }

}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut s = 0;
        let mut path: Vec<i32> = Vec::new();

        search(&mut path, &mut s, n);

        s
    }
}
