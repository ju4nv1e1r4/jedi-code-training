use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut mapping: HashMap<String, String> = HashMap::new();
        mapping.insert(String::from(")"), String::from("("));
        mapping.insert(String::from("}"), String::from("{"));
        mapping.insert(String::from("]"), String::from("["));

        let mut stack:Vec<String>= vec![];

        for c in s.chars() {
            if c == '(' || c == '{' || c == '[' {
                stack.push(c.to_string());
            } else {
                if stack.len() == 0 || stack[stack.len() - 1] != mapping[&c.to_string()] {
                    return false
                }

                stack = stack[0..stack.len() - 1].to_vec();
            }
        }

        if stack.len() == 0 {
            true
        } else {
            false
        }
    }   
}