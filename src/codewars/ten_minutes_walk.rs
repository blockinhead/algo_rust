fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false
    }

    let mut route_x = 0;
    let mut route_y = 0;
    // let _ = walk.iter().map(|ch| {
    for ch in walk {
        match ch {
            'n' => route_y += 1,
            's' => route_y -= 1,
            'w' => route_x += 1,
            'e' => route_x -= 1,
            _ => {}
        }
    }
    // });

    if route_x == 0 && route_y == 0 {
        return true
    }

    false
}

pub fn te_ten_minutes_walk() {
    eprintln!("{:?}", is_valid_walk(&['n','s','n','s','n','s','n','s','n','e']))
}