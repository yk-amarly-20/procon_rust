use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        height: [isize; n],
    }

    let mut cost = vec![std::isize::MAX; n + 1];
    cost[0] = 0;
    cost[1] = 0;
    cost[2] = (height[1] - height[0]).abs();
    if n == 2 {
        println!("{}", cost[2]);
    }
    else {
        for i in 3..=n {
            cost[i] = min(cost[i - 2] + (height[i - 1] - height[i - 3]).abs(),
            cost[i - 1] + (height[i - 1] - height[i - 2]).abs());
        }

        println!("{}", cost[n]);
    }
}
