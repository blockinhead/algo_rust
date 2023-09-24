use std::str::FromStr;

fn ip_to_int(address: &str) -> u32 {
    let pows = vec![1, 2_u32.pow(8), 2_u32.pow(16), 2_u32.pow(24)];
    address.split(".").zip(pows.iter().rev()).map(|(i, mul)| u32::from_str(i).unwrap() * mul).sum()
}

pub fn te_count_ip_addresses() {
    println!("{:?}", ip_to_int("64.233.187.99"))
}