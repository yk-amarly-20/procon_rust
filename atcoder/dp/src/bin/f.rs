use std::cmp::max;

use proconio::{input, marker::Chars};

// dp[i][j]: sのi文字目まで、tのj文字目までのlcs
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut lcs = vec![vec![0; t.len() + 1]; s.len() + 1];

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            lcs[i][j] = if s[i - 1] == t[j - 1] {
                lcs[i - 1][j - 1] + 1
            } else {
                max(lcs[i - 1][j], lcs[i][j - 1])
            };
        }
    }

    let mut ans = vec![];
    let (mut i, mut j) = (s.len(), t.len());
    while i > 0 && j > 0 {
        if lcs[i][j] == lcs[i - 1][j] {
            // s[i - 1]が不要
            i -= 1;
        }
        else if lcs[i][j] == lcs[i][j - 1] {
            j -= 1;
        }
        else {
            ans.push(s[i - 1]);
            i -= 1;
            j -= 1;
        }
    }

    println!("{}", ans.iter().rev().collect::<String>());
}
