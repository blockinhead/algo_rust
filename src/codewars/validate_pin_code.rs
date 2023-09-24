// https://www.codewars.com/kata/55f8a9c06c018a0d6e000132/train/rust

fn validate_pin(pin: &str) -> bool {
    // if (pin.len() == 4 || pin.len() == 6) && pin.chars().all(|c| c.is_numeric()) {
    if (pin.len() == 4 || pin.len() == 6) && pin.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }
    false
}

fn main() {
    println!("Hello, world!");
    println!("{}", validate_pin("    "));
    assert_eq!(validate_pin("1234"), true);
    assert_eq!(validate_pin("123"),  false);
    assert_eq!(validate_pin("a123"),  false);
}
