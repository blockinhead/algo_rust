struct Solution;

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let _colors = colors.chars().collect::<Vec<char>>();
        let mut alice_moves = 0;
        let mut bob_moves = 0;

        for i in 1.._colors.len() - 1 {
            if _colors[i - 1] == _colors[i] && _colors[i] == _colors[i + 1] {
                match _colors[i] {
                    'A' => { alice_moves += 1; }
                    'B' => { bob_moves += 1; }
                    _ => {}
                }
            }
        }


        return alice_moves > bob_moves;
    }
}

pub fn te() {
    println!("{:?}", Solution::winner_of_game("AAABABB".to_string()));
    println!("{:?}", Solution::winner_of_game("AA".to_string()));
}