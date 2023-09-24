// https://www.codewars.com/kata/5264d2b162488dc400000001/train/rust

fn main_() {
    let words = "Hey fellow warriors";
    let mut res= Vec::<String>::new();

    for word in words.split_whitespace() {
        println!("{} {}", word, word.len());
        if word.len() > 4 {
            res.push(word.chars().rev().collect::<String>());
            // println!("{}", word.chars().rev().collect::<String>());
        } else {
            res.push(word.to_string());
        }
    }
    println!("Hello, world!");
    println!("{}", res.join(" "));
}

fn main() {
    let words = "Hey fellow warriors";

    let res = words.split_whitespace().map(|word| {
        match word.len() >= 5 {
            true => word.chars().rev().collect::<String>(),
            false => word.to_string()
        }}).collect::<Vec<String>>().join(" ");
    println!("{}", res);
}
