use crate::Solution;

fn correct_sub_segment(s: &str, p: usize, q: usize) -> bool {
    if q > s.len() {
        return false;
    }

    correct_segment(&s[p..q])
}

fn correct_segment(s: &str) -> bool {
    if s == "0" {
        return true;
    }
    if s.starts_with('0') || s.len() == 0 || s.len() > 3 {
        return false;
    }

    let v: i32 = s.parse().unwrap();

    0 <= v && v < 256
}

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut all_ips: Vec<String> = Vec::new();

        for p in 1..=3 {
            if correct_sub_segment(&s, 0, p) {
                for q in 1..=3 {
                    if correct_sub_segment(&s, p, p + q) {
                        for r in 1..=3 {
                            if correct_sub_segment(&s, p + q, p + q + r)
                                && correct_sub_segment(&s, p + q + r, s.len())
                            {
                                all_ips.push(
                                    [
                                        &s[0..p],
                                        &s[p..p + q],
                                        &s[p + q..p + q + r],
                                        &s[p + q + r..],
                                    ]
                                    .join("."),
                                );
                            }
                        }
                    }
                }
            }
        }

        all_ips
    }
}
