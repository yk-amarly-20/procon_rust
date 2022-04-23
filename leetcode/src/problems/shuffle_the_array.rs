pub struct Solution {}

impl Solution {
  pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    for i in 0..n as usize {
      ans.push(nums[i]);
      ans.push(nums[n as usize + i]);
    }

    return ans;
  }
}
