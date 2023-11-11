struct Solution;
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();

        let mut acc = 0;
        for (i, v) in nums.iter().enumerate() {
            if acc == sum - acc - v {
                return i as i32
            }
            acc += v;
        }

        -1
    }
}

pub fn te() {
    // println!("{:?}", Solution::pivot_index([1,7,3,6,5,6].into()));
    println!("{:?}", Solution::pivot_index([2, 1, -1].into()));
}
