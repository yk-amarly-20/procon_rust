use proconio::{input};

fn main() {
    input! {
        n: usize,
        m: usize,
        c: isize,
        b: [isize; m],
        a: [[isize; m]; n],
    }

    let mut solve_count = 0;
    for i in 0..n {
        let mut sum = 0;
        for j in 0..m {
            sum += a[i][j] * b[j];
        }
        sum += c;

        if sum > 0 {
            solve_count += 1;
        }
    }

    println!("{}", solve_count);
}
