use crate::Solution;
use std::collections::HashSet;

fn all_unique(cells: impl IntoIterator<Item = char>) -> bool {
    let mut seen = HashSet::new();

    cells
        .into_iter()
        .filter(|&cell| cell != '.')
        .all(|cell| seen.insert(cell))
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        (0..9).all(|index| {
            let row = board[index].iter().copied();
            let column = board.iter().map(|row| row[index]);
            let box_row = index / 3 * 3;
            let box_column = index % 3 * 3;
            let square = board[box_row..box_row + 3]
                .iter()
                .flat_map(|row| row[box_column..box_column + 3].iter().copied());

            all_unique(row) && all_unique(column) && all_unique(square)
        })
    }
}
