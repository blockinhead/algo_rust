fn main() {
    let words = "Hey fellow warriors";
    let sub_len = 3;

    let mut substrings = Vec::<String>::with_capacity(words.len() / sub_len);
    let mut string_iter = words.chars();
    let mut pos = 0;

    while pos < words.len() {
        let mut len = 0;
        for ch in string_iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        // substrings.push(format!("{:_<sub_len$}", words[pos..pos + len].to_string(), sub_len=sub_len));
        substrings.push(format!("{:_<1$}", words[pos..pos + len].to_string(), sub_len));
        pos += len;
    }

    println!("{:?}", substrings);
}
