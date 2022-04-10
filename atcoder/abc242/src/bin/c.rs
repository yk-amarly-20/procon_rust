use std::vec;

use proconio::input;
const MOD: isize = 998244353;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![vec![0_isize; 10]; n + 1];
    dp[1] = vec![1; 10];
    for i in 2..(n + 1) {
        for j in 1..10 {
            match j {
                1 => dp[i][1] = (dp[i - 1][1] + dp[i - 1][2]) % MOD,
                9 => dp[i][9] = (dp[i - 1][9] + dp[i - 1][8]) % MOD,
                _ => dp[i][j] = (dp[i - 1][j] + dp[i - 1][j - 1] + dp[i - 1][j + 1]) % MOD,
            }
        }
    }

    let mut sum = 0;
    for i in 1..10 {
        sum += dp[n][i];
        sum %= MOD;
    }
    println!("{}", sum);
}
