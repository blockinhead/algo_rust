fn prime_factors(n: i64) -> String {
    // your code
    let mut nn = n;
    let mut c = 0;
    while  nn % 2 == 0 {
        c += 1;
        nn = nn / 2;
    }

    let mut res = fact_to_str(2, c);

    for d in (3..=(f64::sqrt(n as f64) as i64 + 1)).step_by(2) {
        c = 0;
        while nn % d == 0 {
            c += 1;
            nn = nn / d;
        }
        res += &fact_to_str(d, c);
    }

    if nn != 1 {
        res += &fact_to_str(nn, 1);
    }

    res
}

fn fact_to_str(n: i64, p: i32) -> String {
    match p {
        s if s == 0 => {"".to_string()}
        s if s == 1 => {format!("({:?})", n).to_string()}
        _ => {format!("({:?}**{:?})", n, p)}
    }
}

pub fn te_prime_factors() {
    println!("{:?}", prime_factors(7775460)); // "(2**2)(3**3)(5)(7)(11**2)(17)"
    println!("{:?}", prime_factors(17*17*93*677)); // "(3)(17**2)(31)(677)"
}