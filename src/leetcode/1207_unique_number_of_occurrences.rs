use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        // let mut counter: HashMap<i32, i32> = HashMap::new();
        // for x in arr {
        //     *counter.entry(x).or_insert(0) += 1;
        // }

        let counter = arr.iter().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
        counter.values().cloned().collect::<HashSet<i32>>().len() == counter.len()
    }
}

pub fn te() {
    println!("{:?}", Solution::unique_occurrences([1, 2, 2, 3, 3, 3].into()));
    println!("{:?}", Solution::unique_occurrences([1, 2, 2, 3, 3, 3, 2].into()));
}