use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n + 1],
        mut c: [isize; n + m + 1],
    }

    let mut b = vec![0_isize; m + 1];
    for i in (0..(m + 1)).rev() {
        b[i] = c[n + i] / a[n];
        for j in 0..(n + 1) {
            c[i + j] -= b[i] * a[j];
        }
    }

    for k in 0..(m + 1) {
        print!("{} ", b[k]);
    }
}
