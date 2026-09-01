use crate::Solution;

fn build_board(queen_columns: &[usize], size: usize) -> Vec<String> {
    queen_columns
        .iter()
        .map(|&queen_column| {
            let mut row = vec![b'.'; size];
            row[queen_column] = b'Q';
            String::from_utf8(row).expect("a board contains only ASCII characters")
        })
        .collect()
}

fn search(
    row: usize,
    size: usize,
    occupied_columns: &mut [bool],
    occupied_descending_diagonals: &mut [bool],
    occupied_ascending_diagonals: &mut [bool],
    queen_columns: &mut Vec<usize>,
    solutions: &mut Vec<Vec<String>>,
) {
    if row == size {
        solutions.push(build_board(queen_columns, size));
        return;
    }

    for column in 0..size {
        let descending_diagonal = row + column;
        let ascending_diagonal = row + size - 1 - column;

        if occupied_columns[column]
            || occupied_descending_diagonals[descending_diagonal]
            || occupied_ascending_diagonals[ascending_diagonal]
        {
            continue;
        }

        occupied_columns[column] = true;
        occupied_descending_diagonals[descending_diagonal] = true;
        occupied_ascending_diagonals[ascending_diagonal] = true;
        queen_columns.push(column);

        search(
            row + 1,
            size,
            occupied_columns,
            occupied_descending_diagonals,
            occupied_ascending_diagonals,
            queen_columns,
            solutions,
        );

        queen_columns.pop();
        occupied_ascending_diagonals[ascending_diagonal] = false;
        occupied_descending_diagonals[descending_diagonal] = false;
        occupied_columns[column] = false;
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let size = usize::try_from(n).expect("n must be non-negative");
        let diagonal_count = 2 * size - 1;
        let mut solutions = Vec::new();

        search(
            0,
            size,
            &mut vec![false; size],
            &mut vec![false; diagonal_count],
            &mut vec![false; diagonal_count],
            &mut Vec::with_capacity(size),
            &mut solutions,
        );

        solutions
    }
}
