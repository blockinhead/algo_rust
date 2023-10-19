struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }

        if nums.len() == 1{
            if nums[0] == target {
                return vec![0, 0];
            }
            return vec![-1, -1];
        }

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (right + left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                if mid == 0 {
                    break
                }
                right = mid - 1;
            }
        }

        if nums.get(left + 1).unwrap_or(&(target + 1)) != &target {
            if nums.get(left).unwrap_or(&(target + 1)) != &target {
                return vec![-1, -1];
            }
        }



        let start = left;

        right = nums.len() - 1;

        while left <= right {
            let mid = (right + left) / 2;
            if nums[mid] > target {
                if mid == 0 {
                    break
                }
                right = mid - 1;
            } else {
                left = mid + 1
            }
        }

        vec![start as i32, right as i32]
    }
}


pub fn te() {
    // dbg!(Solution::search_range([5, 7, 7, 8, 8, 9].into(), 8));
    // println!("{:?}", Solution::search_range([0].into(), 1));
    // println!("{:?}", Solution::search_range([1].into(), 0));
    // println!("{:?}", Solution::search_range([2, 2].into(), 1));
    // println!("{:?}", Solution::search_range([1, 3].into(), 1));
    println!("{:?}", Solution::search_range([2, 2].into(), 3));
}
