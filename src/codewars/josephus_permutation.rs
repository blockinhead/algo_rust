use std::collections::HashSet;

fn josephus<T:Clone+Copy>(xs:Vec<T>, k:usize) -> Vec<T> {
    let mut res: Vec<T> = Vec::new();
    let mut seen: HashSet<usize> = HashSet::new();

    let mut cur = k - 1;
    while seen.len() != xs.len() {

        // cur = ((cur + k) - 1) % xs.len();
        while seen.contains(&cur) {
            cur = (cur + 1) % xs.len();
        }
        res.push(xs[cur]);
        seen.insert(cur);
        cur += k;
        cur = cur % xs.len();
    }

    res

}

pub fn te_josephus_premutations() {
    // assert_eq!(josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2), vec![2, 4, 6, 8, 10, 3, 7, 1, 9, 5]);
}