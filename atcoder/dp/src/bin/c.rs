use proconio::input;
use std::cmp::max;

// dp: 3 * (n + 1)
// dp[i][j]: i + 1日目に行動jをした上で、得られる行動の最大値
fn main() {
    input! {
        n: usize,
        // z[i][j]: i + 1日目に行動jをして得られる幸福度
        z: [(usize, usize, usize); n],
    }

    // dp
    let mut best_happiness = vec![vec![0_usize; 3]; n + 1];
    best_happiness[0][0] = z[0].0;
    best_happiness[0][1] = z[0].1;
    best_happiness[0][2] = z[0].2;

    for i in 1..=(n - 1) {
        best_happiness[i][0] = max(best_happiness[i - 1][1], best_happiness[i - 1][2]) + z[i].0;
        best_happiness[i][1] = max(best_happiness[i - 1][2], best_happiness[i - 1][0]) + z[i].1;
        best_happiness[i][2] = max(best_happiness[i - 1][0], best_happiness[i - 1][1]) + z[i].2;
    }

    let max_happiness = max(best_happiness[n - 1][0],
        max(best_happiness[n - 1][1], best_happiness[n - 1][2]));

    println!("{}", max_happiness);
}
