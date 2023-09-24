use std::collections::HashMap;
use std::num::ParseIntError;

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = HashMap::new();

    let mut cmd_index = 0;
    while cmd_index < program.len() {
        match program[cmd_index].split(" ").collect::<Vec<_>>()[..] {
            ["mov", x, y] => {match y.parse::<i64>() {
                Ok(v) => {registers.insert(x.to_string(), v);}
                Err(_) => {registers.insert(x.to_string(), *registers.get(y).unwrap());}
            }}
            ["inc", x] => {*registers.entry(x.to_string()).or_insert(0) += 1; }
            ["dec", x] => {*registers.entry(x.to_string()).or_insert(0) -= 1; }
            ["jnz", x, y] => {
                let f: i64;
                match x.parse::<i64>() {
                    Ok(v) => {f = v;}
                    Err(_) => {f = *registers.get(x).unwrap();}
                }
                if f != 0 { cmd_index = (cmd_index as i64+ y.parse::<i64>().unwrap() - 1) as usize; }
            }
            _ => unreachable!()
        }

        cmd_index += 1;
    }

    registers
}


pub fn te_simple_assembler() {
    // dbg!(simple_assembler(vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"]));
    dbg!(simple_assembler(vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ]));
}