use std::{collections::HashMap, fs::read_to_string};

fn recur(curr_total: usize, mut left: Vec<usize>, res_vec: &mut Vec<usize>) {
    if left.is_empty() {
        res_vec.push(curr_total);
        return;
    }

    let next_elem = left.pop().unwrap();

    let mut tmp = curr_total;
    tmp += next_elem;
    recur(tmp, left.clone(), res_vec);

    let mut tmp = curr_total;
    tmp *= next_elem;
    recur(tmp, left.clone(), res_vec);

    // comment the rest of this function out if you want p1
    let mut tmp = curr_total;
    tmp = [tmp.to_string(), next_elem.to_string()]
        .join("")
        .parse()
        .unwrap();
    recur(tmp, left.clone(), res_vec);
}

fn validate_eqn(prod: usize, expr: Vec<usize>) -> bool {
    let curr_total = 0;
    let mut res_vec = vec![];

    recur(curr_total, expr.clone(), &mut res_vec);
    res_vec.contains(&prod)
}

fn main() {
    let input = read_to_string("input").expect("should have been there ");
    let mut eqns: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(':').collect();
        let prod: usize = split
            .get(0)
            .expect("should be normal")
            .parse()
            .expect("should be a num");
        let mut expr: Vec<usize> = split
            .get(1)
            .expect("should exist")
            .split_whitespace()
            .map(|e| e.parse().expect("should be a number"))
            .collect();

        expr.reverse();
        eqns.insert(prod, expr);
    }

    let mut total_calib_res = 0;
    for (prod, expr) in eqns {
        if validate_eqn(prod, expr) {
            total_calib_res += prod;
        }
    }

    println!("Calib Result : {}", total_calib_res);
}
