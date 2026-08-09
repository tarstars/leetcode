use crate::Solution;

struct Token {
    c: u8,
    is_star: bool,
}

impl Token {
    fn matches(&self, ch: u8) -> bool {self.c == b'.' || self.c == ch}
}

struct Matrix {
    w: usize,
    h: usize,
    dat: Vec<bool>,
}

impl Matrix {
    fn new_shape(h: usize, w: usize) -> Self {Self {h, w, dat: vec![false; w*h]}}
    fn shape(&self) -> (usize, usize) {(self.h, self.w)}
    fn set(&mut self, p: usize, q: usize, v: bool) {self.dat[self.w*p + q] = v}
    fn get(&self, p: usize, q: usize) -> bool {self.dat[self.w*p + q]}
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut regex: Vec<Token> = Vec::with_capacity(p.len());

        for &ch in p {
            if ch != b'*' {
                regex.push(Token { c:ch, is_star: false });
            } else {
                let last = regex.len() - 1;
                regex[last].is_star = true;
            }
        }

        let mut mat = Matrix::new_shape(s.len() + 1, regex.len() + 1);
        mat.set(0, 0, true);
        for q in 1..mat.shape().1 {
            mat.set(0, q,regex[q - 1].is_star && mat.get(0, q - 1)
            )
        }

        for p in 1..mat.shape().0 {
            for q in 1..mat.shape().1 {
                mat.set(p, q,
                    if regex[q - 1].is_star 
                        {mat.get(p, q - 1) || mat.get(p - 1, q) && regex[q - 1].matches(s[p - 1])} 
                    else 
                        {mat.get(p - 1, q - 1) && regex[q - 1].matches(s[p - 1])}
                )
            }
        }

        mat.get(s.len(), regex.len())
    }
}
