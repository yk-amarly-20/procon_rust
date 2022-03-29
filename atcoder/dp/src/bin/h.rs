use proconio::{input, marker::Chars};

const MOD: u64 = 1000000007;
fn main() {
    input! {
        height: usize,
        width: usize,
        maze: [Chars; height],
    }

    // dp[i][j]: (1, 1)から(i, j)へ最短距離で行く場合の数
    let mut dp = vec![vec![0; width + 1]; height + 1];
    dp[1][1] = 1;
    for i in 1..=height {
        for j in 1..=width {
            // 上(dp[i - 1][j]) + 左(dp[i][j - 1]) % MOD
            if maze[i - 1][j - 1] == '#' {
                dp[i][j] = 0;
            }
            else {
                if i == 1 && j == 1 {
                    continue;
                }
                else {
                    dp[i][j] = (dp[i - 1][j] + dp[i][j - 1]) % MOD;
                }
            }
        }
    }
    println!("{}", dp[height][width]);
}
