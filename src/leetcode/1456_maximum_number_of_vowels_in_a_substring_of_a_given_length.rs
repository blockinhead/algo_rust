use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k= k as usize;
        let s = s.chars().collect::<Vec<char>>();

        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);

        let mut res = s.iter().take(k).filter(|c| vowels.contains(c)).count();
        let mut num_vowels = res;
        let mut first_in_window = s[0];

        for w in s.windows(k).skip(1) {
            num_vowels = num_vowels - vowels.contains(&first_in_window) as usize + vowels.contains(&w[k - 1]) as usize;
            res = res.max(num_vowels);
            first_in_window = w[0];
            println!("{:?} {:?} {:?} {:?}", res, w, vowels.contains(&w[0]) as usize, num_vowels);
        }

        res as i32
    }
}

pub fn te() {
    // println!("{:?}", Solution::max_vowels("leetcode".into(), 3));
    println!("{:?}", Solution::max_vowels("novowels".into(), 1));
}
