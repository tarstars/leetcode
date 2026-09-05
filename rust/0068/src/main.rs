#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn words(text: &str) -> Vec<String> {
    text.split_whitespace().map(str::to_owned).collect()
}

fn main() {
    for (text, max_width) in [
        ("This is an example of text justification.", 16),
        ("What must be acknowledgment shall be", 16),
        (
            "Science is what we understand well enough to explain to a computer.",
            20,
        ),
    ] {
        println!("--- width {max_width} ---");
        for line in Solution::full_justify(words(text), max_width) {
            println!("|{line}|");
        }
    }
}

#[cfg(test)]
mod tests;
