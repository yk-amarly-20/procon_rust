use std::vec;
use std::cmp::max;

use proconio::input;

// dp[i][j]: iまでの荷物を選んで、容量をj以下とした時の価値の最大値
// i in [1, n]
// j in [1, w]5 5
fn main() {
    input! {
        n: usize,
        w: usize,
        // i番目の重さ、価値はz[i - 1]
        z: [[usize; 2]; n],
    }

    let mut best_value = vec![vec![0_usize; w + 1]; n + 1];
    for j in 1..=w {
        if j >= z[0][0] {
            best_value[1][j] = z[0][1];
        }
    }

    if n == 1 {
        println!("{}", best_value[1][z[0][0]]);
    }
    else {
        for i in 2..=n {
            for j in 1..=w {
                best_value[i][j] =
                    //if j as isize - z[i - 1][j - z[i - 1][0]] as isize >= 0 {
                    if j >= z[i - 1][0] {
                        max(best_value[i - 1][j],best_value[i - 1][j - z[i - 1][0]] + z[i - 1][1])
                    }
                    else {
                        best_value[i - 1][j]
                    }
            }
        }
        println!("{}", best_value[n][w]);
    }
}
