use proconio::{input};

fn main() {
    input! {
        n: usize,
    }

    let mut max_count = 0;
    let mut max_i = 1;
    for i in 1..n+1 {
        let mut count = 0;
        let mut j = i;
        while j % 2 == 0 {
            j /= 2;
            count += 1;
        }

        if count > max_count {
            max_count = count;
            max_i = i;
        }
    }

    println!("{}", max_i);
}
