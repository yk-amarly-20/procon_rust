use proconio::input;

// dp[i][j] = i回目のジャンプ後、座標jにいることが可能かどうか
fn main() {
    input! {
        n: usize,
        x: usize,
        jump: [[usize; 2]; n],
    }

    let mut dp: Vec<Vec<bool>> = vec![vec![false; x + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..x {
            // a
            if dp[i][j] && j + jump[i][0] <= x {
                dp[i + 1][j + jump[i][0]] = true;
            }
            // b
            if dp[i][j] && j + jump[i][1] <= x {
                dp[i + 1][j + jump[i][1]] = true;
            }
        }
    }

    if dp[n][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
