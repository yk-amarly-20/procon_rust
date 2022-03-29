use proconio::input;

fn main() {
    input! {
        n: usize,
        prob: [f64; n],
    }

    let mut dice = vec![vec![0.0; n + 1]; n + 1];
    dice[0][0] = 1.0;

    for i in 0..n {
        for j in 0..=i {
            // head
            dice[i + 1][j + 1] += dice[i][j] * prob[i];
            // tail
            dice[i + 1][j] += dice[i][j] * (1.0 - prob[i]);
        }
    }

    let ans = dice[n][(n + 1)/ 2..].iter().sum::<f64>();
    println!("{}", ans);
}
