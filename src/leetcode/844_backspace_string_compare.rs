use std::collections::LinkedList;

struct Solution;
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_ = Self::string_to_ll(s);
        let mut t_ = Self::string_to_ll(t);

        while !s_.is_empty() || !t_.is_empty() {
            if s_.pop_back() != t_.pop_back() {
                return false
            }
        }

        if !s_.is_empty() || !t_.is_empty() {
            return false
        }

        true
    }

    fn string_to_ll(s: String) -> LinkedList<char> {
        let mut res = LinkedList::new();
        for c in s.chars() {
            if c == '#' {
                if res.len() > 0 {
                    res.pop_back();
                }
            } else {
                res.push_back(c);
            }
        }

        res
    }
}

pub fn te(){
    println!("{:?}", Solution::backspace_compare("ab##c".to_string(), "ad#c".to_string()));
}
