struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut max = 0;
        for i in 1..=std::cmp::min(str1.len(), str2.len()) {
            let substr = str1[0..i].to_string();
            dbg!(&substr);
            dbg!(substr.repeat(str1.len() / i).to_string());
            dbg!(substr.repeat(str2.len() / i));

            if substr.repeat(str1.len() / i) == str1 && substr.repeat(str2.len() / i) == str2 {
                max = i;
            }
        }
        return if max > 0 { str1[0..max].to_string() } else {"".to_string()};
    }
}

pub fn te() {
    // println!("{:?}", Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()));
    println!("{:?}", Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()));
}