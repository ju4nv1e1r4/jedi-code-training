use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut my_hashmap = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            let complement = target - n;
            if my_hashmap.contains_key(&complement) {
                return vec![my_hashmap[&complement] as i32, i as i32];
            }
            my_hashmap.insert(n, i);
            }
        return vec![];
    }
}
