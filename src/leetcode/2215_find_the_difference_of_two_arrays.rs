use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution;
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1: HashSet<i32> = HashSet::from_iter(nums1);
        let nums2: HashSet<i32> = HashSet::from_iter(nums2);

        [(&nums1 - &nums2).into_iter().collect::<Vec<i32>>(), (&nums2 - &nums1).into_iter().collect::<Vec<i32>>()].into()
    }
}

pub fn te() {
    println!("{:?}", Solution::find_difference([1, 2, 3].into(), [2, 4, 6].into()));
}