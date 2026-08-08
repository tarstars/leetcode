use crate::Solution;

impl Solution {
    fn quick_select(a: &Vec<i32>, b: &Vec<i32>, mut p: usize) -> f64 {
        let na = a.len();
        let nb = b.len();

        let mut la = 0;
        let mut ra = na;
        let mut lb = 0;
        let mut rb = nb;

        while ra > la && rb > lb {
            let ma = (la + ra) / 2;
            let mb = (lb + rb) / 2;

            if a[ma] < b[mb] {
                if p > ma - la + mb - lb {
                    p -= ma - la + 1;
                    la = ma + 1;
                } else {
                    rb = mb;
                }
            } else {
                if p > mb - lb + ma - la {
                    p -= mb - lb + 1;
                    lb = mb + 1;
                } else {
                    ra = ma; 
                }
            }
        }

        if ra > la {
            return a[la + p] as f64
        }
        b[lb + p] as f64
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n1 = nums1.len();
        let n2 = nums2.len();

        (Self::quick_select(&nums1, &nums2, (n1 + n2) / 2) + Self::quick_select(&nums1, &nums2, (n1 + n2 - 1) / 2))/2.0
    }
}
