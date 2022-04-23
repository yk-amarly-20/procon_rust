use proconio::input;
use std::cmp::max;

pub struct Solution {}

impl Solution {
  pub fn get_max_length(n: usize, a: Vec<isize>) -> isize {
    let mut ans: isize = 0;
    for i in 0..=(n - 3) {
      for j in (i + 1)..=(n - 2) {
        for k in (j + 1)..=(n - 1) {
          if !is_triangle(a[i], a[j], a[k]) {
            continue;
          }
          else {
            ans = max(ans, a[i] + a[j] + a[k]);
          }
        }
      }
    }

    return ans;
  }
}

pub fn is_triangle(a: isize, b: isize, c: isize) -> bool {
  (b - c).abs() <= a && a <= b + c
}

fn main() {
  input! {
    n: usize,
    a: [isize; n],
  }

  let ans = Solution::get_max_length(n, a);
  println!("{}", ans);
}

#[cfg(test)]
mod tests {
  use std::vec;

use super::*;

  #[test]
  fn test_case() {
    let n_case_1 = 5;
    let a_case_1 = vec![2, 3, 4, 5, 10];
    assert_eq!(Solution::get_max_length(n_case_1, a_case_1), 12);

    let n_case_2 = 4;
    let a_case_2 = vec![4, 5, 10, 20];
    assert_eq!(Solution::get_max_length(n_case_2, a_case_2), 0);
  }
}
