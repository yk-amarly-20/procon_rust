use proconio::input;
use std::cmp::min;

pub struct Solution {}

impl Solution {
  pub fn solve(c1: isize, c5: isize, c10: isize, c50: isize, c100: isize, c500: isize, mut a: isize) -> isize {
    let mut ans = 0;
    let price = vec![1, 5, 10, 50, 100, 500];
    let coins: Vec<isize> = vec![c1, c5, c10, c50, c100, c500];
    for i in (0..6).rev() {
      let p = min(coins[i], a / price[i]);
      ans += p;
      a -= p * price[i];
    }

    return ans;
  }
}

fn main() {
  input! {
    c1: isize,
    c5: isize,
    c10: isize,
    c50: isize,
    c100: isize,
    c500: isize,
    mut a: isize
  }

  let ans = Solution::solve(c1, c5, c10, c50, c100, c500, a);
  println!("{}", ans);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_case() {
    let case_c1 = 3;
    let case_c5 = 2;
    let case_c10 = 1;
    let case_c50 = 3;
    let case_c100 = 0;
    let case_c500 = 2;
    let case_a = 620;
    assert_eq!(Solution::solve(case_c1, case_c5, case_c10, case_c50, case_c100, case_c500, case_a), 6);
  }
}
