use std::f32;

use proconio::{input};

fn main() {
  input! {
    a:f32,
    b: f32,
  }

  let ans: f32 = ((b - 1.) / (a - 1.)).ceil();
  println!("{}", ans);

}
