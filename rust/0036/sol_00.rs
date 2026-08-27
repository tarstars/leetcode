use crate::Solution;
use std::collections::HashMap;

fn good(a: &Vec<char>) -> bool {
    let mut f: HashMap<char, u8> = HashMap::new();

    for c in a {
        if *c != '.' {
            *f.entry(*c).or_insert(0) += 1;
        }
    }

    for &v in f.values() {
        if v > 1 {
            return false;
        }
    }

    true
}

fn get_column(b: &Vec<Vec<char>>, q: usize) -> Vec<char> {
    let mut col: Vec<char> = Vec::new();

    for p in 0..9 {
        col.push(b[p][q]);
    }

    return col
}

fn get_cluster(b: &Vec<Vec<char>>, p: usize, q: usize) -> Vec<char> {
    let mut cluster: Vec<char> = Vec::new();

    for pp in 0..3 {
        for qq in 0..3 {
            cluster.push(b[p*3 + pp][q*3 + qq]);
        }
    }

    cluster
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for p in 0..9 {
            if !good(&board[p]) {
                return false;
            }
        }

        for q in 0..9 {
            if !good(&get_column(&board, q)) {
                return false;
            }
        }

        for p in 0..3 {
            for q in 0..3 {
                if !good(&get_cluster(&board, p, q)) {
                    return false;
                }
            }
        }

        true
    }
}
