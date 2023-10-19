struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut prev = cost[1];
        let mut prevprev = cost[0];

        for i in 2..cost.len() {
            let t = cost[i] + std::cmp::min(prev, prevprev);
            prevprev = prev;
            prev = t;
        }

        std::cmp::min(prev, prevprev)
    }
}

pub fn te() {
    println!("{:?}", Solution::min_cost_climbing_stairs(vec![1,100,1,1,1,100,1,1,100,1]));
}
