fn beeramid (bonus: i32, price: f32) -> usize {
    let mut n = 0.0;
    for i in 1.. {
        n += (i * i) as f32 * price;
        if n > bonus as f32 {
            return (i - 1) as usize;
        }
    }

    0
}


pub fn te_beeramid() {
    assert_eq!(beeramid(454, 5.0), 5);
    assert_eq!(beeramid(455, 5.0), 6);
}