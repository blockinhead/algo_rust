struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = candies.iter().max().unwrap_or(&0);
        candies.iter().map(|x| {x + extra_candies >= *max }).collect()
    }
}

pub fn te() {
    println!("{:?}", Solution::kids_with_candies([4, 2, 1, 1, 2].into(), 1));
}
