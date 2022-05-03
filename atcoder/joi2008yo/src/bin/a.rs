use proconio::input;

const PRICE: [usize; 6] = [500, 100, 50, 10, 5, 1];
pub struct Solution {}

impl Solution {
    pub fn run(mut n: usize) -> usize {
        n = 1000 - n;
        let mut count_coins = 0;
        for i in 0..6 {
            let count = n / PRICE[i];
            n -= count * PRICE[i];
            count_coins += count;
        }
        return count_coins;
    }
}

fn main() {
    input! {
        mut n: usize,
    }

    let ans = Solution::run(n);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let case_n_1 = 380;
        assert_eq!(Solution::run(case_n_1), 4);
        let case_n_2 = 1;
        assert_eq!(Solution::run(case_n_2), 15)
    }
}
