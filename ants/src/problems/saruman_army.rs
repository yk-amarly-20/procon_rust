use proconio::input;

pub struct Solution {}

impl Solution {
  pub fn saruman_army(n: usize, r: isize, x: Vec<isize>) -> isize {
    let mut count = 0;
    let mut i = 0;
    while i < n {
      // s: カバーされていない最も左にある点
      let s = x[i];
      while i < n && x[i] <= s + r {
        i += 1;
      }
      // p: 新しく印をつける点
      let p = x[i - 1];
      count += 1;
      while i < n && x[i] <= p + r {
        i += 1;
      }
    }
    return count;
  }
}

fn main() {
  input! {
    n: usize,
    r: isize,
    x: [isize; n],
  }

  let ans = Solution::saruman_army(n, r, x);
  println!("{}", ans);
}

#[cfg(test)]
mod tests {
  use std::vec;

use super::*;

  #[test]
  fn test_case() {
    let case_n = 6;
    let case_r = 10;
    let case_x = vec![1, 7, 15, 20, 30, 50];
    assert_eq!(Solution::saruman_army(case_n, case_r, case_x), 3);
  }
}
