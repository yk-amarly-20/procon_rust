use proconio::{input};

fn main() {
    input! {
        n: f32,
    }

    let x = (n / 1.08).floor();

    for i in  0..2 {
        if ((x + i as f32) * 1.08) as usize == n as usize {
            println!("{}", x as usize + i);
            return;
        }
    }

    println!(":(");
}
