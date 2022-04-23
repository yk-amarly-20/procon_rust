use proconio::input;

pub struct Solution {}

impl Solution {
  pub fn scheduling(n: usize, s: Vec<isize>, t: Vec<isize>) -> isize {
    let mut ans = 0;
    let jobs = cleansing(n, s, t);
    let mut present = 0;
    for i in 0..n {
      if present < jobs[i].start_time {
        ans += 1;
        present = jobs[i].end_time;
      }
    }
    return ans;
  }
}

pub fn cleansing(n: usize, s: Vec<isize>, t: Vec<isize>) -> Vec<Job> {
  let mut jobs: Vec<Job> = Vec::new();
  for i in 0..n {
    let job = Job::new(s[i], t[i]);
    jobs.push(job);
  }
  jobs.sort_by(|job1, job2| job1.end_time.partial_cmp(&job2.end_time).unwrap());

  return jobs;
}

pub struct Job {
  start_time: isize,
  end_time: isize
}

impl Job {
  pub fn new(start_time: isize, end_time: isize) -> Self {
    Self {
      start_time: start_time,
      end_time: end_time
    }
  }
}

fn main() {
  input! {
    n: usize,
    s: [isize; n],
    t: [isize; n],
  }

  let ans = Solution::scheduling(n, s, t);
  println!("{}", ans);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_case() {
    let case_n = 5;
    let case_s = vec![1, 2, 4, 6, 8];
    let case_t = vec![3, 5, 7, 9, 10];
    assert_eq!(Solution::scheduling(case_n, case_s, case_t), 3);
  }
}
