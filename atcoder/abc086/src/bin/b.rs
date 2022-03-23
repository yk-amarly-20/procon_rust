use proconio::{input};
fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let digit_b: u32 = b.to_string().len() as u32;
    let num = (a * (10_i32.pow(digit_b) as f64) + b) as f64;
    let sqrt = num.sqrt() as usize;

    if (sqrt * sqrt) == num as usize {
        println!("Yes");
        return;
    }

    println!("No");
}
