struct Solution;
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut highest_altitude = 0;
        let mut altitude = 0;
        for v in &gain {
            altitude += v;
            highest_altitude = highest_altitude.max(altitude);
        }

        highest_altitude
    }
}

pub fn te() {
    println!("{:?}", Solution::largest_altitude([-5,1,5,0,-7].into()));
}