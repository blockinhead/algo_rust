struct Solution;
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut current = vec![1; row_index as usize + 1];
        let mut prev = vec![1; row_index as usize + 1];

        for i in 1..=row_index as usize{
            for j in 1..i {
                current[j] = prev[j] + prev[j - 1]
            }
            (prev, current) = (current, prev);
        }

        prev
    }
}

pub fn te() {
    println!("{:?}", Solution::get_row(0));
}