use crate::Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut res = vec![vec![0; n as usize]; n as usize];
        let mut l = 0;
        let mut r = n;
        let mut u = 0;
        let mut d = n;
        let mut ind = 1;

        while l < r && u < d {
            for q in l..r {
                res[u][q] = ind;
                ind = ind + 1;
            }
            u = u + 1;
            if u == d {break}

            r -= 1;
            for p in u..d {
                res[p][r] = ind;
                ind = ind + 1;
            }
            if l == r {break}

            d = d - 1;
            for q in (l..r).rev() {
                res[d][q] = ind;
                ind = ind + 1;
            }
            if u == d {break}

            for p in (u..d).rev() {
                res[p][l] = ind; 
                ind += 1;
            }
            l += 1;
        }

        res
    }
}
