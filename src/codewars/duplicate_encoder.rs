// https://www.codewars.com/kata/54b42f9314d9229fd6000d9c/train/rust

use std::collections::{HashMap, HashSet};

fn duplicate_encode_(word:&str) ->String {
    let words_lower = word.to_lowercase();
    let mut letter_count: HashMap<char, i32> = HashMap::new();
    for c in words_lower.chars() {
        *letter_count.entry(c).or_insert(0) += 1;
    }
    words_lower.chars().map(|c| if letter_count[&c] == 1 {"("} else {")"}).collect()
}

fn duplicate_encode(word:&str) ->String {
    let words_lower = word.to_lowercase();
    let mut single_ch: HashSet<char> = HashSet::new();
    let mut multi_ch: HashSet<char> = HashSet::new();
    for c in words_lower.chars() {
        if single_ch.contains(&c) {
            multi_ch.insert(c);
        } else {
            single_ch.insert(c);
        }
    }
    words_lower.chars().map(|c| if multi_ch.contains(&c) {")"} else { "(" }).collect()
}

fn main() {
    assert_eq!(duplicate_encode("din"),"(((");
    assert_eq!(duplicate_encode("recede"),"()()()");
    assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
    assert_eq!(duplicate_encode("(( @"),"))((");
}
