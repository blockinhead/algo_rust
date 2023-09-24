use std::collections::{HashMap, HashSet};

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut res = String::new();

    let mut prev_chars = HashMap::new();

    for triplet in triplets.iter() {
        prev_chars.entry(triplet[0]).or_insert(HashSet::new());
        prev_chars.entry(triplet[1]).and_modify(|x: &mut HashSet<char>| { x.insert(triplet[0]); }).or_insert(HashSet::from([triplet[0]]));
        prev_chars.entry(triplet[2]).and_modify(|x: &mut HashSet<char>| { x.insert(triplet[1]); }).or_insert(HashSet::from([triplet[1]]));
    }

    // dbg!(&prev_chars);

    while !&prev_chars.is_empty() {
        // dbg!(&prev_chars);
        let _prev_chars = prev_chars.clone();
        let next_first = _prev_chars.iter().find_map(|(key, val)| return if val.is_empty() { Some(key) } else { None }).unwrap();
        // dbg!(&next_first);

        prev_chars.remove(&next_first);

        for (_, val) in prev_chars.iter_mut() {
            val.remove(&next_first);
        }

        res.push(*next_first);

    }

    res
}


pub fn te_recover_secret() {
    assert_eq!(recover_secret(vec![
        ['t','u','p'],
        ['w','h','i'],
        ['t','s','u'],
        ['a','t','s'],
        ['h','a','p'],
        ['t','i','s'],
        ['w','h','s']])
               , "whatisup");
}