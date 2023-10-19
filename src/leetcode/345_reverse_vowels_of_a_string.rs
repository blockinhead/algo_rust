use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u',
                                         'A', 'E', 'I', 'O', 'U'].into_iter().collect();
        let mut s: Vec<char> = s.chars().collect();
        let mut left: usize = 0;
        let mut right: usize = s.len() - 1;

        while left < right {
            if vowels.contains(&s[left]) {
                while !vowels.contains(&s[right]) {
                    right -= 1;
                }
                if left == right {
                    break;
                }
                s.swap(left, right);
                right -= 1;
            }
            left += 1;
        }

        s.into_iter().collect()
    }
}

pub fn te() {
    println!("{:?}", Solution::reverse_vowels("leetcode".into()));
}