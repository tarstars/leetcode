#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (intervals, new_interval) in [
        (vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        (
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16],
            ],
            vec![4, 8],
        ),
    ] {
        let inserted = Solution::insert(intervals.clone(), new_interval.clone());
        println!("{intervals:?} + {new_interval:?} -> {inserted:?}");
    }
}

#[cfg(test)]
mod tests;
