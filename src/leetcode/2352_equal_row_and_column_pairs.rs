use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        /*
        let mut d = grid.iter().fold(HashMap::new(), |mut acc, val| {
            *acc.entry(val).or_insert(0) += 1;
            acc
        });
        // dbg!(&d);

        for i in 0..grid.len() {
            let mut c = vec![0; grid.len()];
            for j in 0..grid.len() {
                c[j] = grid[j][i];
            }
            if let Some(v) = d.get_mut(&c) {
                *v += 1;
            }
        }

        // dbg!(&d);

        d.values().filter(|&x| *x != 0).sum()
         */

        let rows = grid.iter().fold(HashMap::new(), |mut acc, val| {
            *acc.entry(val).or_insert(0) += 1;
            acc
        });

        let mut columns: HashMap<Vec<i32>, i32> = HashMap::new();
        for i in 0..grid.len() {
            let mut c = vec![0; grid.len()];
            for j in 0..grid.len() {
                c[j] = grid[j][i];
            }
            *columns.entry(c).or_insert(0) += 1;
        }

        rows.iter().fold(0, |mut acc, (&k, v)| {
            acc += columns.get(k).unwrap_or(&0) * v;
            // acc += columns.entry(k.into()).or_(0) * v;
            acc
        })
    }
}

pub fn te() {
    println!("{:?}", Solution::equal_pairs([vec![3,2,1], vec![1,7,6], vec![2,7,7]].into()));
    println!("{:?}", Solution::equal_pairs([vec![13, 13], vec![13, 13]].into()));
}