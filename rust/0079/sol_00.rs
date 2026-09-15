use crate::Solution;

use std::collections::HashSet;

fn found(
    board: &Vec<Vec<char>>,
    h: usize,
    w: usize,
    word: &Vec<char>,
    word_offset: usize,
    visited: &mut HashSet<(usize, usize)>,
    p: i32,
    q: i32,
) -> bool {
    if word_offset + 1 as usize == word.len() {
        return true;
    }

    for (dp, dq) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
        let (np, nq) = (p + dp, q + dq);
        if 0 <= np && np < h as i32 && 0 <= nq && nq < w as i32 {
            let (np, nq) = (np as usize, nq as usize);
            if !visited.contains(&(np, nq)) && board[np][nq] == word[(word_offset + 1) as usize] {
                visited.insert((np, nq).clone());
                if found(
                    board,
                    h,
                    w,
                    word,
                    word_offset + 1,
                    visited,
                    np as i32,
                    nq as i32,
                ) {
                    return true;
                }
                visited.remove(&(np, nq));
            }
        }
    }

    false
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let h = board.len();
        let w = board[0].len();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let word: Vec<char> = word.chars().collect();

        for p in 0..h {
            for q in 0..w {
                if board[p][q] == word[0] {
                    visited.insert((p, q));
                    if found(&board, h, w, &word, 0, &mut visited, p as i32, q as i32) {
                        return true;
                    }
                    visited.remove(&(p, q));
                }
            }
        }

        false
    }
}
