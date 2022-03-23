use proconio::{input};
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let reminder = n % k;
    let reminder_abs = k - reminder;

    println!("{}", min(reminder, reminder_abs));
}
