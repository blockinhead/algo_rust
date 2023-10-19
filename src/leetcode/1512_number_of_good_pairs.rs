use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut d = HashMap::new();

        for v in nums.iter() {
            d.entry(v).and_modify(|x| { res += *x; *x += 1 }).or_insert(1);
        }

        res
    }
}

pub fn te() {
    println!("{:?}", Solution::num_identical_pairs([1, 2, 3, 1, 1, 3].into()));
}