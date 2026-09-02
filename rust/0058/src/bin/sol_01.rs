#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for s in [
        "Hello World",
        "   fly me   to   the moon  ",
        "luffy is still joyboy",
    ] {
        let length = Solution::length_of_last_word(s.to_owned());
        println!("{s:?} -> {length}");
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
