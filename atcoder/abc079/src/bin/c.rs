use proconio::input;

pub struct Solution {}

impl Solution {
    pub fn solve(s: String) -> String {
        let mut ans = String::new();
        let s = digits_vec(s);
        // 部分集合の位数はn
        let n = s.len() - 1;
        for bit in 0..(1 << n) {
            // bitを下から見て、1が立ってる場合は足し算
            let mut sum = s[0];
            let mut opt: Vec<char> = Vec::new();
            for i in 0..n {
                if (bit & (1 << i)) != 0 {
                    sum += s[i + 1];
                    opt.push('+');
                } else {
                    sum -= s[i + 1];
                    opt.push('-');
                }
            }
            if sum == 7 {
                ans += &s[0].to_string();
                for i in 0..n {
                    ans += &opt[i].to_string();
                    ans += &s[i + 1].to_string();
                }
                ans += "=7";
                break;
            }
        }

        return ans;
    }
}

pub fn digits_vec(s: String) -> Vec<isize> {
    let mut vec: Vec<isize> = Vec::new();
    for c in s.chars().into_iter() {
        vec.push(c.to_string().parse::<isize>().unwrap())
    }
    return vec;
}

fn main() {
    input! {
        s: String,
    }

    let ans = Solution::solve(s);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let case_s = "1222".to_string();
        assert_eq!(Solution::solve(case_s), "1+2+2+2=7".to_string());
    }

    #[test]
    fn test_digits() {
        let case_s = "1222".to_string();
        assert_eq!(digits_vec(case_s), vec![1, 2, 2, 2]);
    }
}
