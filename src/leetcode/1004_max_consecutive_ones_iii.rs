struct Solution;
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut zero_counter = k;

        for v in nums.iter() {
            right += 1;
            if *v == 0 {
                zero_counter -= 1
            }
            // когда привышено допустимое число нулей, левая граница начинает ползти за правой.
            // если левая граница встречает ноль, то возможно у правой границы будет шанс отползти чуть дальше
            if zero_counter < 0 {
                if nums[left] == 0 {
                    zero_counter += 1
                }
                left += 1
            }
        }

        (right - left) as i32
    }
}

pub fn te() {
    println!("{:?}", Solution::longest_ones([1,1,1,0,0,0,1,1,1,1,1].into(), 2));
    println!("{:?}", Solution::longest_ones([1,1,1,0,0,0,1,1,1,1,0].into(), 2));
    println!("{:?}", Solution::longest_ones([1, 1, 1].into(), 2));
    println!("{:?}", Solution::longest_ones([0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1].into(), 3));
}