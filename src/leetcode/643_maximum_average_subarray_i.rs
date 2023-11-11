struct Solution;
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut s: i32 = nums[0..k].iter().sum();
        let mut first = nums[0];
        let mut max_s = s;

        for i in k..nums.len() {
            s = s - first + nums[i];
            if s > max_s {
                max_s = s;
            }
            first = nums[i - k + 1];
        }

        max_s as f64/ k as f64
    }
}

pub fn te() {
    println!("{:?}", Solution::find_max_average([1,12,-5,-6,50,3].into(), 4))
}
