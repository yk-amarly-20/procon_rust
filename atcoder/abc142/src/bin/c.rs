use proconio::{input};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut v = vec![0; n + 1];
    for (i, &elem) in a.iter().enumerate() {
        v[elem] = i + 1;
    }

    for i in 1..=n {
        print!("{} ", v[i]);
    }
}
