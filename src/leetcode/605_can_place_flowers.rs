struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {

        if n == 0 {
            return true;
        }

        let mut counter = n;

        let mut i = 0;
        while i < flowerbed.len() {
            let prev = if i == 0 { 0 } else { flowerbed[i - 1] };
            let current = flowerbed[i];
            let next = if i == flowerbed.len() - 1 { 0 } else { flowerbed[i + 1] };

            if prev == 0 && current == 0 && next == 0 {
                counter -= 1;
                i += 2
            } else {
                i += 1
            }
            if counter == 0 {
                return true;
            }
        }

        false
    }
}


pub fn te() {
    // println!("{:?}", Solution::can_place_flowers([1, 0, 0, 0, 1].into(), 1));
    // println!("{:?}", Solution::can_place_flowers([1, 0, 0, 0, 1].into(), 2));
    // println!("{:?}", Solution::can_place_flowers([0, 0, 1, 0, 1].into(), 1)); // true
    println!("{:?}", Solution::can_place_flowers([1,0,0,0,0,0,1].into(), 2)); // true
    println!("{:?}", Solution::can_place_flowers([1, 0, 0, 0, 1, 0, 0].into(), 2)); // true


}