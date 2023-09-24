use std::str::FromStr;
use itertools::{EitherOrBoth, Itertools};

fn add_pair(c1: char, c2: char) -> (char, bool) {
    let r = (i8::from_str(c1.to_string().as_str()).unwrap() + i8::from_str(c2.to_string().as_str()).unwrap()).to_string();
    if r.len() > 1 {
        return (r.chars().nth(1).unwrap(), true)
    }

    (r.chars().next().unwrap(), false)

}

fn add_str(s1: &String, s2: &String) -> String {
    let r = s1.chars().rev().zip_longest(s2.chars().rev()).map(|x| match x {
        EitherOrBoth::Both(a, b) => {add_pair(a, b)}
        EitherOrBoth::Left(a) => {add_pair(a, '0')}
        EitherOrBoth::Right(a) => {add_pair(a, '0')}
    });
        // .collect::<Vec<_>>();
    let mut res = "".to_string();
    let mut current_flag = false;
    for (d, next_flag) in r {
        if current_flag {
            let (d_tmp, flag_tmp) = add_pair(d, '1');
            res.push(d_tmp);
            current_flag = next_flag || flag_tmp;
        } else {
            res.push(d);
            current_flag = next_flag;
        }
    }
    if current_flag {
        res.push('1');
    }

    res.chars().rev().collect()
}

fn mult_str(s: &String, by: char) -> String {
    match by {
        '0' => "0".to_string(),
        '1' => s.to_string(),
        _ => {
            let mut r = s.clone();
            for _ in 2..=by.to_digit(10).unwrap() {
                r = add_str(&r, &s);
            }
            r.to_string()
        }
    }
}

fn multiply(a: &str, b: &str) -> String {
    if a == "0" || b == "0" {
        return "0".to_string()
    }
    let mut res = "0".to_string();
    let aa = a.to_string();
    // aa.trim_start_matches()

    for (i, c) in b.to_string().chars().rev().enumerate() {
        let mut t = mult_str(&aa, c);
        if !t.replace("0", "").is_empty() {
            t.push_str(&*std::iter::repeat('0').take(i).collect::<String>());
        } else { t = "0".to_string(); }
        res = add_str(&t, &res);
    }

    res = res.trim_start_matches('0').to_string();
    if res.is_empty() {
        return "0".to_string();
    }
    res
}

pub fn te_mult_as_str() {
    // dbg!(add_pair('1', '8'));
    // dbg!(add_str(&"112".to_string(), &"1099".to_string()));
    // dbg!(mult_str(&"0114".to_string(), '0'));
    dbg!(multiply("00","030"));

}