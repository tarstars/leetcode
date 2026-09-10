use crate::Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut st: Vec<&str> = Vec::new();

        for v in path.split('/') {
            match v {
                "" => {}
                "." => {}
                ".." => {
                    st.pop();
                }
                _ => {
                    st.push(v);
                }
            }
        }

        "/".to_owned() + &st.join("/")
    }
}
