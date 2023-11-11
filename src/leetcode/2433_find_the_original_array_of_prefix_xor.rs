struct Solution;
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; pref.len()];
        res[0] = pref[0];
        for i in 1..pref.len() {
            res[i] = pref[i] ^ pref[i-1]
        }

        res
    }
}

pub fn te() {
    println!("{:?}", Solution::find_array([5,2,0,3,1].into()));
}