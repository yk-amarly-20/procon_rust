use proconio::input;

pub struct Solution {
  n: usize,
  a: Vec<isize>,
  k: isize,
}

impl Solution {
  pub fn new(n: usize, a: Vec<isize>, k: isize) -> Self {
    Self {
      n: n,
      a: a,
      k: k
    }
  }

  pub fn solve(&self) -> bool {
    return self.dfs(0, 0);
  }

  // 深さ優先探索
  pub fn dfs(&self, i: usize, sum: isize) -> bool {
    if i == self.n {
      return self.k == sum;
    }

    if self.dfs(i + 1, sum) {
      return true;
    }

    if self.dfs(i + 1, sum + self.a[i]) {
      return true;
    }

    return false;
  }
}

fn main() {
  input! {
    n: usize,
    a: [isize; n],
    k: isize,
  }

  let solution = Solution::new(n, a, k);
  if solution.solve() {
    println!("Yes");
  } else {
    println!("No");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_case() {
    let n_case_1 = 4;
    let a_case_1 = vec![1, 2, 4, 7];
    let k_case_1 = 13;
    assert_eq!(Solution::new(n_case_1, a_case_1, k_case_1).solve(), true);
  }
}
