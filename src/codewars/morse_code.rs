pub fn decode_morse_code(encoded: &str) -> String {
    let mut res = String::new();

    for s in encoded.trim().split("   ") {
        dbg!(s);
        if s == "ddd" {
            res.push_str("some!")
        } else {
            for l in s.split(" ") {
                dbg!(l);
                res.push_str(l);
            }
        }
        res.push(' ');
    }
    res.pop();

    dbg!(res);

    return "Ok!".to_string()
}

pub fn te_morse_code() {
    // decode_morse_code("aaa   ddd   aaa bbb   ccc dd");
    decode_morse_code("  ");
}