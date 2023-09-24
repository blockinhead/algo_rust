pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().reduce(|a, b| a ^ b).unwrap()
}

pub fn single_number_te() {
    println!("{:?}", single_number(vec![1, 1, 23]));
}