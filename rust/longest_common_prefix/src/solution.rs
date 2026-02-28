pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        for a in 0..strs[0].len() {
            for b in 1..strs.len()  {
                if a == strs[b].len() || strs[0].chars().nth(a) != strs[b].chars().nth(a) {
                    return strs[0][..a].to_string();
                }
            }
        }
        return strs[0].to_string();
    }

    pub fn longest_common_prefix_v2(strs: Vec<String>) -> String {
        let first_bytes = strs[0].as_bytes();
        for (a, &b) in first_bytes.iter().enumerate() {
            for c in &strs[1..] {
                let d = c.as_bytes();
                if a >= d.len() || d[a] != b {
                    return strs[0][..a].to_string();
                }
            }
        }
        return strs[0].to_string();
    }
}