use crate::Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let c2n = vec![
            ("I", 1),
            ("IV", 4),
            ("V", 5),
            ("IX", 9),
            ("X", 10),
            ("XL", 40),
            ("L", 50),
            ("XC", 90),
            ("C", 100),
            ("CD", 400),
            ("D", 500),
            ("CM", 900),
            ("M", 1000),
            ];
        
        let mut res= String::new();
        while num != 0 {
            let mut max_ind = 0;
            for p in 0..c2n.len() {
                if c2n[p].1 > num {
                    break;
                }
                max_ind = p;
            }
            res.push_str(c2n[max_ind].0);
            num -= c2n[max_ind].1;
        }

        res
    }
}
