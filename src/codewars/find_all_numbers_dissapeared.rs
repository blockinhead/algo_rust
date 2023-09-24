#[cfg_attr(debug_assertions, allow(unused_imports, unused_variables))]

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut flags = vec![false; nums.len() + 1];
    for i in nums {
        flags[i as usize] = true;
    }

    flags
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(i, &v)| v != true)
        .map(|(i, v)| i as i32)
        .collect()
}

pub fn te() {
    println!("{:?}", find_disappeared_numbers(vec![1, 1, 1]))
}