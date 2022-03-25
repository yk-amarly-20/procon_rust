use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        height: [isize; n],
    }

    let mut best_cost = vec![std::isize::MAX; n + 1];
    best_cost[0] = 0;
    best_cost[1] = 0;
    for i in 1..=(n - 1) {
        // i: current position
        for j in 1..=k {
            // j: jump count
            if i + j <= n {
                best_cost[i + j] = best_cost[i + j].min(best_cost[i] + (height[i - 1] - height[i + j - 1]).abs());
            }
        }
    }

    println!("{}", best_cost[n]);
}
