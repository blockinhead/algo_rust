struct Solution;
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = nums[0];
        let mut second = 0;

        let mut ii = nums.len();
        for i in 1..nums.len() {
            if nums[i] > first {
                second = nums[i];
                ii = i;
                break;
            } else {
                first = nums[i];
            }
        }

        for i in ii + 1..nums.len() {
            match nums[i] {
                v if v > second => { return true; }
                v if v <= second && v > first => { second = v; }
                v if v <= first => { first = v; }
                _ => { panic!() }
            }
        }

        return false;
    }
}

pub fn te() {
    println!("{:?}", Solution::increasing_triplet([1, 2, 3, 4, 5].into()));
    println!("{:?}", Solution::increasing_triplet([5, 4, 3, 2, 1].into()));
}