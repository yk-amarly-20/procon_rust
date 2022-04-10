use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = n + 1;
    let mut flag = false;

    for i in (0..k).rev() {
        let tmp = a[i];
        for j in k..min(i + ans, n) {
            if tmp < a[j] {
                // 入れ替える
                // ans = min(ans, j - i);
                ans = j - i;
                flag = true;
                break;
            }
        }
    }

    if flag {
        println!("{}", ans);
    }
    else {
        println!("{}", -1);
    }

}
