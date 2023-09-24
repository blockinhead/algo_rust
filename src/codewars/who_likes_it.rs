// https://www.codewars.com/kata/5266876b8f4bf2da9b000362/train/rust

fn likes(names: &[&str]) -> String {
    match names {
        [] => String::from("no one likes this"),
        [name] => format!("{name} likes this"),
        [name1, name2] => format!("{name1} and {name2} like this"),
        [name1, name2, name3] => format!("{name1}, {name2} and {name3} like this"),
        // [name1, name2, ..] => format!("{name1}, {name2} and {} others like this", names.len() - 2)
        [n1, n2, rest@ ..] => format!("{n1}, {n2} and {} others like this", rest.len())
    }
}

fn _main() {
    assert_eq!(likes(&[]), "no one likes this");
    assert_eq!(likes(&["Peter"]), "Peter likes this");
    assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
    assert_eq!(
        likes(&["Max", "John", "Mark"]),
        "Max, John and Mark like this"
    );
    assert_eq!(
        likes(&["Alex", "Jacob", "Mark", "Max"]),
        "Alex, Jacob and 2 others like this"
    );
}
