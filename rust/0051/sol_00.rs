use crate::Solution;

fn vec2board(a: &Vec<i32>) -> Vec<String> {
    let n = a.len();
    let mut board: Vec<Vec<u8>> = Vec::new();
    for p in 0..n {
        board.push(vec![b'.'; n])
    }


    for (p, q) in a.iter().enumerate() {
        board[p][*q as usize] = b'Q';
    }

    board
        .into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .collect()
}

fn last_compatible(a: &Vec<i32>) -> bool {
    if let Some(v) = a.last() {
        let ind = a.len();
        for p in 0..ind - 1 {
            if a[p] == *v || ind - 1 - p == (*v - a[p]).abs() as usize {
                return false;
            }
        }
    }
    return true;
}

fn find_all_vectors(a: &mut Vec<Vec<i32>>, n: usize, path: &mut Vec<i32>) {
    if path.len() == n {
        a.push(path.clone());
        return;
    }

    for d in 0..(n as i32) {
        path.push(d);

        if last_compatible(path) {
            find_all_vectors(a, n, path);
        }

        path.pop();
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut qs: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();

        find_all_vectors(&mut qs, n as usize, &mut path);

        qs.iter().map(vec2board).collect()
    }
}
