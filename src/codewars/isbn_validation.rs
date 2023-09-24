fn valid_isbn10(isbn: &str) -> bool {
    if isbn.len() != 10 {
        return false;
    }

    let mut acum = 0;
    for (i, ch) in isbn[0..9].chars().enumerate() {
        match ch.to_digit(10) {
            None => {return false}
            Some(v) => {acum += (i + 1) as u32 * v}
        }
    }
    let rem = acum % 11;
    let last = isbn.chars().last().unwrap();
    return match rem {
        10 => { last == 'X' }
        _ => { rem == last.to_digit(10).unwrap_or(11) }
    }
}

fn dotest(isbn: &str, expected: bool) {
    let actual = valid_isbn10(isbn);
    assert!(actual == expected, "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}")
}

pub fn te_valid_isbn() {
    dotest("1112223339", true);
    dotest("048665088X", true);
    dotest("1293000000", true);
    dotest("1234554321", true);
    dotest("1234512345", false);
    dotest("1293", false);
    dotest("X123456788", false);
    dotest("ABCDEFGHIJ", false);
    dotest("XXXXXXXXXX", false);
    dotest("123456789T", false);
}