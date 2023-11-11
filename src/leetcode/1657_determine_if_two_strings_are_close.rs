use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let c1 = Solution::count(word1);
        let c2 = Solution::count(word2);

        if c1.len() != c2.len() || !c1.keys().all(|k| c2.contains_key(k)) {
            return false
        }

        let mut t1 = c1.values().collect::<Vec<_>>();
        t1.sort();
        let mut t2 = c2.values().collect::<Vec<_>>();
        t2.sort();

        for (&v1, v2) in t1.iter().zip(t2) {
            if v1 != v2 {
                return false
            }
        }

        true
    }

    fn count(word: String) -> HashMap<char, i32> {
        word.chars().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
    }
}

pub fn te() {
    println!("{:?}", Solution::close_strings("cabbba".into(), "abbccc".into()));
}