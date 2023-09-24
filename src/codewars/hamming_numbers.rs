// def hamming(n):
//     hammings = [1] * n
//     x2, x3, x5 = 2, 3, 5
//     i = j = k = 0
//
//     for n in range(1, n):
//         hammings[n] = min(x2, x3, x5)
//         if x2 == hammings[n]:
//             i += 1
//             x2 = 2 * hammings[i]
//         if x3 == hammings[n]:
//             j += 1
//             x3 = 3 * hammings[j]
//         if x5 == hammings[n]:
//             k += 1
//             x5 = 5 * hammings[k]
//
//     return hammings[-1]
//
//
// print(hamming(30))



use itertools::min;

fn hamming(n: usize) -> u64 {
    let mut numbers = vec![1u64; n];

    let mut x2 = 2u64;
    let mut x3 = 3u64;
    let mut x5 = 5u64;

    let mut i = 0u64;
    let mut j = 0u64;
    let mut k = 0u64;

    for n in 1..n {
        numbers[n] = min(vec![x2, x3, x5]).unwrap();
        if x2 == numbers[n] {
            i += 1;
            x2 = 2u64 * numbers[i as usize];
        }
        if x3 == numbers[n] {
            j += 1;
            x3 = 3u64 * numbers[j as usize];
        }
        if x5 == numbers[n] {
            k += 1;
            x5 = 5u64 * numbers[k as usize];
        }
    }

    numbers[n-1]
}

pub fn te_hamming() {
    println!("{:?}", hamming(20))
}