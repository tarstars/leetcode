#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    let strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    let groups = Solution::group_anagrams(strs);
    println!("{groups:?}");
}

#[cfg(test)]
mod tests;
