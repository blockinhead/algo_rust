struct Solution;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        if n == 3 {
            return 2;
        }

        let un: usize = n as usize;

        let mut max_products = vec![0; un + 1];  // to store from 1 to n
        max_products[1] = 1;

        for i in 2..=un {
            let mut current = 0;
            for j in 1..i {

                current = std::cmp::max(
                    current,
                    std::cmp::max(j * max_products[i - j], j * (i - j))
                );
            }
            max_products[i] = current;
        }

        max_products[un] as i32
    }
}

// есть способ быстрее - разложить число на сумму 3 + ... + 3 + 4 или + 2
// с одной стороны понятно, что множители должны быть как можно ближе друг к другу, 7 * 2 < 5 * 4
// но не понятно, почему раскладывать надо на тройки, а не на двойки. хотя 3 * 3 > 2 * 2 * 2

pub fn te() {
    println!("{:?}", Solution::integer_break(10));
}