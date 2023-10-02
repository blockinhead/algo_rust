use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let counter_s = Solution::string_to_hash(&s);
        let counter_t = Solution::string_to_hash(&t);

        for (ch, i) in &counter_t {
            if counter_s.get(ch).unwrap_or(&0) != i {
                return *ch;
            }
        }

        unreachable!()
    }

    fn string_to_hash(s: &String) -> HashMap<char, i32> {
        s
            .chars()
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            })
    }
}

pub fn te() {
    println!("{:?}", Solution::find_the_difference("abcd".to_string(), "abcde".to_string()));
}