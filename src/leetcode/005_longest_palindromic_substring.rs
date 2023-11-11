struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut max_len = 0;
        let mut start = 0;
        let mut left = 0;
        let mut right;

        for i in 1..s.len() {
            left = i - 1;
            right = i;
            while left >= 0 && right < s.len() && s[left] == s[right] {
                if right - left > max_len {
                    start = left;
                    max_len = right - left
                }
                if left == 0 {
                    break
                }
                left -= 1;
                right += 1;
            }

            left = i - 1;
            right = i + 1;
            while left >= 0 && right < s.len() && s[left] == s[right] {
                if right - left > max_len {
                    start = left;
                    max_len = right - left
                }
                if left == 0 {
                    break
                }
                left -= 1;
                right += 1;
            }
        }

        s[start..=start + max_len].iter().collect::<String>()
    }
}

pub fn te() {
    // println!("{:?}", Solution::longest_palindrome("babad".into()))
    println!("{:?}", Solution::longest_palindrome("bb".into()))
}
