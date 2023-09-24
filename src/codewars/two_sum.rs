use std::collections::HashMap;

fn two_sum_(numbers: &[i32], target: i32) -> (usize, usize) {
    let mut cahce: HashMap<i32, usize> = HashMap::new();

    for (i, val) in numbers.iter().enumerate() {
        if cahce.contains_key(&(target - val)) {
            return (i, *cahce.get(&(target - val)).unwrap());
        } else { cahce.insert(*val, i); }
    }
    return (0, 0)
}

fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    let mut map = HashMap::with_capacity(numbers.len());

    for (i, &v) in numbers.iter().enumerate() {
        match map.get(&(target - v)) {
            Some(&idx) => return (i, idx),
            None => map.insert(v, i),
        };
    }
    unreachable!();
}

pub fn te_two_sum() {
    let r = two_sum(&[1, 2, 3], 4);
    println!("{:?}", r);
}