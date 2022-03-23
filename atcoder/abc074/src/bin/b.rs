use proconio::{input};
use std::cmp::{min};


fn main() {
    input! {
        n: usize,
        k: usize,
        x: [usize; n],
    }


    let mut total_distance = 0;
    for i in 0..n {
        total_distance += min(x[i] as i32, k as i32 - x[i] as i32).abs() as usize * 2;
    }

    println!("{}", total_distance);
}
