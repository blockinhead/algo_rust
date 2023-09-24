fn to_digits(v: u64) -> Vec<u64> {
    let mut v_copy = v;
    let mut buf: Vec<u64> = Vec::new();

    while v_copy > 0 {
        let n = v_copy % 10;
        v_copy = v_copy / 10;
        buf.push(n);
    }

    buf
}

fn sum_powers(n: u64) -> u64 {
    to_digits(n).iter().rev().enumerate().map(|(i, v)| { v.pow(i as u32 + 1)} ).sum()
}

fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();

    for i in a..=b {
        dbg!(i);
        dbg!(sum_powers(i));
        if sum_powers(i) == i {
            res.push(i);
        }
    }

    res
}

pub fn te_sum_powers() {
    println!("{:?}", 89);
    println!("{:?}", sum_dig_pow(1, 100));
    // println!("{:?}", sum_powers(89));
}