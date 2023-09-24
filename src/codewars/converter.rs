fn converter(n: f64, decimals: u8, base: f64) -> String {
    if n == 0.0 {
        let mut res = "0".to_string();
        if decimals != 0 {
            res.push('.');
            for _ in 0..decimals {
                res.push('0');
            }
        }

        return res;
    }

    let mut res = String::new();
    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();

    if n < 0.0 { res.push('-') }
    let mut _n = n.abs();

    for i in (- (decimals as i32) ..= _n.log(base) as i32).rev() {
        if i == -1 { res.push('.') }
        let current_digit = base.powi(i);
        res.push(digits[(_n / current_digit) as usize] as char);
        _n = _n % (base.powi(i));
    }
    res
}

pub fn te_converter() {
    assert_eq!(converter(13.0, 0, 8.0), "15", "It should convert 13.0 into 15 base 8.0");
    assert_eq!(converter(10.0, 0, 16.0), "A", "It should convert 10.0 into A base 16.0");
    assert_eq!(converter(std::f64::consts::PI, 0, 2.0), "11", "It should convert pi into 11 base 2.0");
    assert_eq!(converter(7.0, 0, 19.0), "7", "It should convert 7.0 into 7 base 19.0");
    assert_eq!(converter(1.0, 0, 2.0), "1", "It should convert 1.0 into 1 base 2.0");
    assert_eq!(converter(-10.0, 0, 23.0), "-A", "It should convert -10.0 into -A base 23.0");
    assert_eq!(converter(0.0, 4, 26.0), "0.0000", "It should convert 0.0 into 0.0000 base 26.0");
    assert_eq!(converter(-15.5, 2, 23.0), "-F.BB", "It should convert -15.5 into -F.BB base 23.0");
}