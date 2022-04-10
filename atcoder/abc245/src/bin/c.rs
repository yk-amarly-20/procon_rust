use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
        b: [isize; n],
    }

    if solve(n, k, &a, &b) {
        println!("Yes");
    } else {
        println!("No");
    }

}

fn solve(
    n: usize,
    k: isize,
    a: &Vec<isize>,
    b: &Vec<isize>,
) -> bool {
    if n == 1 {
        return true;
    }
    let mut dp = vec![false; n + 1];
    let mut ep = vec![false; n + 1];

    dp[1] = true;
    ep[1] = true;

    for i in 1..n {
        dp[i + 1] = (((a[i] - a[i - 1]).abs() <= k) && dp[i]) || (((a[i] - b[i - 1]).abs() <= k) && ep[i]);
        ep[i + 1] = (((b[i] - a[i - 1]).abs() <= k) && dp[i]) || (((b[i] - b[i - 1]).abs() <= k) && ep[i]);
    }

    return dp[n] || ep[n];
}
