#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for path in [
        "/home/",
        "/home//foo/",
        "/home/user/Documents/../Pictures",
        "/../",
        "/.../a/../b/c/../d/./",
    ] {
        let simplified = Solution::simplify_path(path.to_owned());
        println!("{path:40} -> {simplified}");
    }
}

#[cfg(test)]
mod tests;
