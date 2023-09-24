use itertools::{izip, put_back};

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {

    let mut res = Vec::new();
    let mut m = matrix.to_vec();

    if matrix.len() == 0 {
        return Vec::new()
    }



    while m.len() > 0 {
        res.append(&mut m.remove(0));
        if m.len() == 0 {
            break
        }


        // dbg!(&res);
        // dbg!(&m);

        let mut new_m = Vec::new();
        let num_columns = m[0].len();
        for _ in 0..num_columns {
            let mut t = Vec::new();
            let index_to_remove = m[0].len() - 1;
            for i in 0..m.len() {

                t.push(m[i].remove(index_to_remove))
            }
            new_m.push(t);
        }
        // dbg!(&new_m);
        m = new_m;


    }


    res
}


pub fn te_snail() {
    let square = &[
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9],
    ];
    let expected = vec![1,2,3,6,9,8,7,4,5];
    assert_eq!(snail(square), expected);
}