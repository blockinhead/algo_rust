use std::cmp;
use itertools::Itertools;


fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    let mut max_d = -1;

    for v in ls.iter().combinations(usize::try_from(k).unwrap()) {
        let v_sum = v.iter().map(|x| **x).sum();
        if v_sum == t {
            return t;
        }
        if v_sum < t {
            max_d = cmp::max(max_d, v_sum);
        }
    }



    max_d
}

pub fn te_best_travel() {
    let ts = &vec![91, 74, 73, 85, 73, 81, 87];
    println!("{:?}", choose_best_sum(230, 3, ts));  // , 228);
    println!("{:?}", choose_best_sum(331, 2, ts));  // , 178);
    println!("{:?}", choose_best_sum(163, 3, &vec![50]));
    println!("{:?}", choose_best_sum(163, 3, &vec![50, 55, 56, 57, 58]));

}