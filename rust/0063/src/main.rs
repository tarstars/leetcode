#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

/// Builds a grid from a picture: `.` is open ground, `#` is an obstacle.
fn grid(rows: &[&str]) -> Vec<Vec<i32>> {
    rows.iter()
        .map(|row| {
            row.bytes()
                .map(|cell| if cell == b'#' { 1 } else { 0 })
                .collect()
        })
        .collect()
}

fn main() {
    for rows in [
        vec!["...", ".#.", "..."],
        vec![".#", ".."],
        vec!["#"],
        vec![".....", "####.", "....."],
    ] {
        let paths = Solution::unique_paths_with_obstacles(grid(&rows));
        println!("{rows:?} -> {paths}");
    }
}

#[cfg(test)]
mod tests;
