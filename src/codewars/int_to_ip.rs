fn int32_to_ip(int: u32) -> String {
    format!("{:?}.{:?}.{:?}.{:?}", int >> 24 & 0xff, int >> 16 &0xff, int >> 8 & 0xff, int & 0xff)
}


pub fn te_int32_to_ip() {
    assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
    assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
    assert_eq!(int32_to_ip(0), "0.0.0.0");
}