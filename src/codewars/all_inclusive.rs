// https://www.codewars.com/kata/5700c9acc1555755be00027e/train/rust


fn contain_all_rots_my(strng: &str, arr: Vec<&str>) -> bool {
    let mut str_vec: Vec<char> = strng.chars().collect();
    for _ in 0..strng.len() {
        str_vec.rotate_left(1);
        if !arr.iter().any(|&x| x == str_vec.iter().collect::<String>()) { return false }
    }
    true
}

fn contain_all_rots(strng: &str, arr: Vec<&str>) -> bool {
    let mut str_vec: Vec<char> = strng.chars().collect();
    for _ in 0..strng.len() {
        str_vec.rotate_left(1);
        if !arr.contains(&str_vec.iter().collect::<String>().as_str()) {return false}
    }
    true
}

fn main() {
    // let a = HashSet::from([1, 2, 3]);
    // let b = HashSet::from([4, 2, 3, 4]);
    // let c:HashSet<&i32> = a.union(&b).collect();
    // println!("{:?}", c);

    let v = contain_all_rots("bsjq", vec!["bsjq", "qbsj", "sjqb", "twZNsslC", "jqbs"]); // -> true
    println!("{}", v);
    let v = contain_all_rots("Ajylvpy", vec!["Ajylvpy", "ylvpyAj", "jylvpyA", "lvpyAjy", "pyAjylv", "vpyAjyl", "ipywee"]); // -> false)
    println!("{}", v);
    let v = contain_all_rots("", vec![]);
    println!("{}", v);
}
