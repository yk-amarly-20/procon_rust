use proconio::{input};

// 全部同じなら無限ループ？

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut c: usize,
    }

    if a == b && b == c && a % 2 == 0 {
        println!("-1");
        return;
    }

    let mut count = 0;

    while is_all_even(a, b, c) {
        let tmp_a = a;
        let tmp_b = b;
        let tmp_c = c;

        a = tmp_b / 2 + tmp_c / 2;
        b = tmp_c / 2 + tmp_a / 2;
        c = tmp_a / 2 + tmp_b / 2;
        count += 1;
    }

    println!("{}", count);
}

fn is_all_even(x1: usize, x2: usize, x3: usize) -> bool {
    if x1 % 2 == 0 && x2 % 2 == 0 && x3 % 2 == 0 {
        return true;
    }

    return false;
}
