struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        // всё существенно проще, если знать что у вектора есть трейт window
        let mut ord: Vec<bool> = Vec::new();
        for i in  1..nums.len() {
            let d = nums[i - 1] - nums[i];
            if d != 0 { ord.push(d > 0)};
        }

        return ord.iter().all(|x|{ x == &ord[0]})
    }
}

pub fn te() {
    println!("{:?}", Solution::is_monotonic(vec![1, 2, 4, 3]));
    println!("{:?}", Solution::is_monotonic(vec![1, 2, 2, 3]));
    println!("{:?}", Solution::is_monotonic(vec![4, 4, 2, 1]));
}