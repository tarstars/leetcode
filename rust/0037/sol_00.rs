use crate::Solution;

use std::collections::HashSet;

fn all_unique(it: impl IntoIterator<Item=char> ) -> bool {
    let mut visited: HashSet<char> = HashSet::new();

    it.into_iter().filter(|c| *c != '.').all(|c| visited.insert(c))
}

fn good_board(b: &[Vec<char>]) -> bool {
   (0..9).all(
        |index| {
            let ind_col = index % 3;
            let ind_row = index / 3;

            all_unique(b[index].iter().copied()) &&
            all_unique(b.iter().map(|r| r[index])) &&
            all_unique(
                b[ind_col*3..(ind_col * 3 + 3)]
                .iter()
                .flat_map(|v| v[ind_row*3..(ind_row*3 + 3)]
                .iter()
                .copied())
            )
        }
    )
}

fn solver(board: &mut Vec<Vec<char>>, offset: usize) -> bool {
    let mut next_offset: Option<usize> = None;

    for current_offset in offset..81 {
        let p = current_offset / 9;
        let q = current_offset % 9;

        if board[p][q] == '.' {
            next_offset = Some(current_offset);
            break;
        }
    }

    if next_offset.is_none() {
        return true;
    }

    let p = next_offset.unwrap() / 9;
    let q = next_offset.unwrap() % 9;

    for c in ['1', '2', '3', '4', '5', '6', '7', '8', '9'] {

        board[p][q] = c;

        if !good_board(board) {
            continue;
        }

        if solver(board, next_offset.unwrap()) {
            return true;
        }
    }

    board[p][q] = '.';

    false
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        solver(board, 0);
    }
}
