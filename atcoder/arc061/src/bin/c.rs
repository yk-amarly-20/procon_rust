use proconio::input;

pub struct Solution {}

impl Solution {
    pub fn solve(s: usize) -> usize {
        let mut sum = 0;
        let s = digits_vec(s);
        // 全部でn通り
        let n = s.len() - 1;
        for bit in 0..(1 << n) {
            let mut ratio = 1;
            // bitは2進数で3桁
            for i in 0..s.len() {
                sum += ratio * s[i];
                ratio = if (bit & (1 << i)) != 0 { 1 } else { ratio * 10 };
            }
        }

        return sum;
    }
}

// 一桁ずつ格納したベクターを返す
pub fn digits_vec(n: usize) -> Vec<usize> {
    let mut n = n;
    if n == 0 {
        return vec![0];
    }

    let mut vec: Vec<usize> = Vec::new();
    while n > 0 {
        vec.push(n % 10);
        n /= 10;
    }

    return vec;
}

fn main() {
    input! {
        s: usize,
    }

    let ans = Solution::solve(s);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let case_s = 125;
        assert_eq!(Solution::solve(case_s), 176);
    }
}
