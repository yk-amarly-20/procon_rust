use std::str::CharIndices;

use proconio::input;

pub struct Solution {}

impl Solution {
  pub fn solve(n: usize, s: String) -> String {
    let mut ans = String::new();
    let s_char: Vec<char> = s.chars().collect();
    let mut a = 0;
    let mut b = n - 1;
    while a <= b {
      let mut left = true;
      for i in 0..=(b - a) {
        if s_char[a + i] < s_char[b - i] {
          left = true;
          break;
        }
        else if s_char[a + i] > s_char[b - i] {
          left = false;
          break;
        }
      }
      if left {
        ans += &s_char[a].to_string();
        a += 1;
      }
      else {
        ans += &s_char[b].to_string();
        b -= 1;
      }
    }

    return ans;
  }
}

fn main() {
  input! {
    n: usize,
    s: String,
  }

  let ans = Solution::solve(n, s);
  println!("{}", ans);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_case() {
    let case_n = 6;
    let case_s = "ACDBCB".to_string();
    assert_eq!(Solution::solve(case_n, case_s), "ABCBCD".to_string());
  }
}
