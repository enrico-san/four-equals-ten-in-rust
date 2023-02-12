#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;
use rand::thread_rng;
use rand::seq::SliceRandom;

static PARENTHESIS: [[[&str; 3]; 2]; 6] = [
    [["", "", ""], ["", "", ""]],
    [["(", "", ""], [")", "", ""]],
    [["(", "", ""], ["", ")", ""]],
    [["", "(", ""], ["", ")", ""]],
    [["", "(", ""], ["", "", ")"]],
    [["", "", "("], ["", "", ")"]],
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let op = &args[1];
    let n = &args[2];

    let result = run(n, op, 100000);
}

fn run(n: &String, op: &String, cycles: i32) -> i32 {
    let mut n_vec: Vec<char> = n.chars().collect();
    let mut op_vec: Vec<char> = op.chars().collect();
    let mut count = 0;
    loop {
        count += 1;
        if count >= cycles {
            println!("not found in {} cycles", cycles);
            return count;
        }

        n_vec.shuffle(&mut thread_rng());
        op_vec.shuffle(&mut thread_rng());
        let exps = make_expression(&n_vec, &op_vec);
        for exp in exps {
            let r = meval::eval_str(&exp).unwrap();
            if r == 10.0 {
                println!("found in {}: {} = {}", count, exp, r);
                return count;
            }
        }
    }
}

fn make_expression(n_vec: &Vec<char>, op_vec: &Vec<char>) -> Vec<String> {
    let mut exps = Vec::new();
    for row in 0..PARENTHESIS.len() {
        let [p_ope, p_clo] = PARENTHESIS[row];
        let x = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}",
            p_ope[0], n_vec[0], op_vec[0], p_ope[1], n_vec[1], p_clo[0], op_vec[1], p_ope[2], n_vec[2], p_clo[1], op_vec[2], n_vec[3], p_clo[2]
        );
        exps.push(x);
    }
    return exps;
}
