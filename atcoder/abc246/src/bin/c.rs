use proconio::input;
use std::cmp::{Reverse, min};

fn main() {
    input! {
        n: usize,
        k: usize,
        x: isize,
        mut a: [isize; n],
    }

    let mut sum = 0;
    a.sort_by_key(|&x| Reverse(x));
    let mut i = 0;
    let mut rest = k as isize;   // 残りのクーポンの枚数
    while i < n && rest > 0 {
        let use_coupon_counts = min(a[i] / x, rest);
        rest = rest - use_coupon_counts;
        a[i] = a[i] - use_coupon_counts * x;
        i += 1;
    }

    if rest == 0 {
        for i in 0..n {
            sum = sum + a[i];
        }
        println!("{}", sum);
    }
    else {
        a.sort_by_key(|&x| Reverse(x));
        i = 0;
        while i < n && rest > 0 {
            if a[i] > 0 {
                a[i] = 0;
                rest -= 1;
            }
            i += 1;
        }
        println!("{}", sum);
    }
}
