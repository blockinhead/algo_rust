// https://www.geeksforgeeks.org/find-next-greater-number-set-digits/

fn next_bigger_number(n: u64) -> Option<u64> {
    let mut digits = n.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let mut i = digits.len() - 2;
    while i >= 0 {
        if digits[i] < digits[i + 1] {
            // digits.swap(i, i-1);
            // return Some(digits.iter().fold(0, |accum, elem| {accum * 10 + *elem as u64}))
            break
        }
        i -= 1
    }
    if i == 0 {
        return None
    }
    dbg!(&i);

    let mut j = digits.len() - 1;
    while j > i {
        if digits[j] > digits[i] {
            break
        }
        j -= 1
    }

    dbg!(&j);
    digits.swap(i, j);
    dbg!(&digits);
    digits[j+1..].reverse();
    dbg!(&digits);

    return Some(digits.iter().fold(0, |accum, elem| {accum * 10 + *elem as u64}))
}


pub fn te_next_bigger_number() {
    const ERR_MSG: &str = "\nYour result (left) did not match the expected result (right).";

    // assert_eq!(next_bigger_number(9), None, "{ERR_MSG}");
    // assert_eq!(next_bigger_number(12), Some(21), "{ERR_MSG}");
    // assert_eq!(next_bigger_number(513), Some(531), "{ERR_MSG}");
    // assert_eq!(next_bigger_number(2017), Some(2071), "{ERR_MSG}");
    // assert_eq!(next_bigger_number(414), Some(441), "{ERR_MSG}");
    // assert_eq!(next_bigger_number(144), Some(414), "{ERR_MSG}");
    next_bigger_number(1234567890);
    // assert_eq!(next_bigger_number(1234567890), Some(1234567908), "{ERR_MSG}");
    // assert_eq!(next_bigger_number(3154248282496110951), Some(3154248282496111059), "{ERR_MSG}");
}