use std::collections::{HashMap, HashSet};
use itertools::Itertools;
// use std::iter::FromIterator;

fn str_to_freq_hash(s: &str) -> HashMap<char, usize> {
    let mut res = s.chars()
        .filter(|x| x.is_ascii_lowercase())
        .fold(HashMap::new(),
              |mut accum, val|
                  {
                      *accum.entry(val).or_insert(0) += 1;
                      accum
                  }
        );
    res.retain(|_, v| *v > 1);
    res
}

fn mix(s1: &str, s2: &str) -> String {
    let ss1 = str_to_freq_hash(s1);
    let ss2 = str_to_freq_hash(s2);

    let amounts1 = ss1.values().cloned().collect::<Vec<usize>>();
    let amounts2 = ss2.values().cloned().collect::<Vec<usize>>();
    let mi:HashSet<usize> = HashSet::from_iter(amounts1.iter().cloned().chain(amounts2.iter().cloned()));
    let mis = mi.into_iter().sorted().rev().collect::<Vec<usize>>();

    let l: HashSet<char> = HashSet::from_iter(ss1.keys().cloned().chain(ss2.keys().cloned()));
    let ll = l.into_iter().sorted().collect::<Vec<char>>();

    // dbg!(&mis);

    let mut res: Vec<String> = Vec::new();

    for i in mis {
        // for ch in b'a'..=b'z' {
        let mut restmp: Vec<String> = Vec::new();
        for ch in ll.iter() {
            let a1 = ss1.get(ch).unwrap_or(&1);
            let a2 = ss2.get(ch).unwrap_or(&1);

            // dbg!(ch, a1, a2);
            if *a1 != i && *a2 != i {
                continue
            }
            if *a1 > i || *a2 > i {
                continue
            }
            if a1 > a2 {
                restmp.push(format!("1:{}", str::repeat(&ch.to_string(), *a1)));
            } else if a2 > a1 {
                restmp.push(format!("2:{}", str::repeat(&ch.to_string(), *a2)));
            }
        }

        for ch in ll.iter() {
            let a1 = ss1.get(ch).unwrap_or(&1);
            let a2 = ss2.get(ch).unwrap_or(&1);
            if *a1 != i || *a2 != i {
                continue
            }
            if a2 == a1 && *a1 > 1 {
                restmp.push(format!("=:{}", str::repeat(&ch.to_string(), *a1)));
            }
        }
        restmp.sort();
        res.append(&mut restmp);
    }

    res.join("/")
}
pub fn te_string_mix() {
     assert_eq!(&mix("Are they here", "yes, they are here"), "2:eeeee/2:yy/=:hh/=:rr");
     assert_eq!(&mix("looping is fun but dangerous", "less dangerous than coding",), "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
        // mix(" In many languages", " there's a pair of functions",
        //     "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
        // mix("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
        // testing("codewars", "codewars", "");
        // testing("A generation must confront the looming ", "codewarrs",
        //     "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");
}