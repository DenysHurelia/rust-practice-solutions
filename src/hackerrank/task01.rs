//- https://www.hackerrank.com/challenges/simple-array-sum/problem

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};


fn simpleArraySum(ar: &[i32]) -> i32 {
    let mut total = 0;
    for &num in ar {
        total += num;
    }
    total
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let _n: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    println!("{}", simpleArraySum(&arr));
}
