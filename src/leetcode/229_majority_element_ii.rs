struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut elem_1: Option<i32> = None;
        let mut elem_1_counter = 0;
        let mut elem_2: Option<i32> = None;
        let mut elem_2_counter = 0;

        // нас интересуют два элемента, которых больше чем одна треть.
        // значит на каждый из таких элементов может приходится меньше чем один любой другой элемент
        // значит если будем увеличивать счётчик, когда встретили такой элемент,
        // и уменьшать счётчик, когда встретили любой другой элемент,
        // то в элементах осядут как раз те, которых две трети.
        // если счётчик = 0, то место кандидата вакантно
        // но возможно, там будет мусор (например, когда все элементы разные).
        // тогда надо проверить, действительно ли найдены годные кандидаты

        // с дефолтдиктом и дропом вакантных мест из словаря было бы намного более внятно

        for v in nums.iter() {
            let elem = Some(*v);

            if elem_1_counter > 0 && elem_1 == elem {
                elem_1_counter += 1;
                continue
            }

            if elem_2_counter > 0 && elem_2 == elem {
                elem_2_counter += 1;
                continue
            }


            if elem_1_counter == 0 {
                elem_1 = elem;
                elem_1_counter = 1;
                continue;
            } else {
                if elem_2_counter == 0 {
                    elem_2 = elem;
                    elem_2_counter = 1;
                    continue;
                }
            }

            elem_1_counter = std::cmp::max(0, elem_1_counter - 1);
            elem_2_counter = std::cmp::max(0, elem_2_counter - 1);
        }

        let mut res= vec![];
        let count = nums.len() as f32 / 3.0;
        if elem_1_counter > 0 {
            let v = elem_1.unwrap();
            if nums.iter().filter(|x| { *x==&v }).count() as f32 > count {
                res.push(v);
            }
        }

        if elem_2_counter > 0 {
            let v = elem_2.unwrap();
            if nums.iter().filter(|x| { *x==&v }).count() as f32 > count {
                res.push(v);
            }
        }

        res
    }
}

pub fn te() {
    // println!("{:?}", Solution::majority_element([3, 2, 3].to_vec()));
    // println!("{:?}", Solution::majority_element([3, 2, 3, 2, 1].to_vec()));
    // println!("{:?}", Solution::majority_element([2,1,1,3,1,4,5,6].to_vec()));
    println!("{:?}", Solution::majority_element([4,1,2,3,4,4,3,2,1,4].to_vec()));
}