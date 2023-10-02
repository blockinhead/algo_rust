struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut current_row: Vec<f64> = vec![poured as f64];

        for row_number in 1..=query_row {
            let mut next_row: Vec<f64> = vec![0.0; (row_number + 1) as usize];
            let mut v_: f64;
            // dbg!(row_number, &current_row);

            for (i, v) in current_row.iter().enumerate() {
                v_ = f64::max(0.0, (v - 1.0) / 2.0);
                next_row[i] += v_;
                next_row[i + 1] += v_;
            }

            current_row = next_row;
        }

        f64::min(1.0, current_row[query_glass as usize])
    }
}

pub fn te() {
    println!("{:?}",  Solution::champagne_tower(2, 1, 1));
    println!("{:?}",  Solution::champagne_tower(100000009, 33, 17));
    println!("{:?}",  Solution::champagne_tower(7, 3, 1));
}