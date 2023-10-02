// use itertools::Itertools;

struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s
            .split_whitespace()
            .map(|s| {s.chars().rev().collect::<String>()})
            .collect::<Vec<String>>()
            .join(" ")
    }
}

pub fn te() {
    println!("{:?}", Solution::reverse_words("Let's take LeetCode contest".to_string()));
}