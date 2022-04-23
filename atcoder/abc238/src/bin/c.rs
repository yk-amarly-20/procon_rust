use proconio::input;

const MOD: usize = 998244353;

pub fn calc_digitnum(n: usize) -> usize {
    return n.to_string().len();
}

pub fn calc_sum_digit(digit: usize) -> usize {
    // digit桁の自然数全ての和を計算
    // MODで割った値としておく
    let min_digit_n = (10_i32.pow(digit as u32) - 1) as usize;
    let min_digit_n_1 = (10_i32.pow(digit as u32 - 1) - 1) as usize;
    return min_digit_n * (min_digit_n + 1) / 2 % MOD - min_digit_n_1 * (min_digit_n_1 + 1) / 2 % MOD
}

fn main() {
    input! {
        n: usize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_digitnum() {
        let case1: usize = 45;
        assert_eq!(calc_digitnum(case1), 2);
        let case2: usize = 353425246;
        assert_eq!(calc_digitnum(case2), 9);
    }

    #[test]
    fn test_calc_sum_digit() {
        let case1: usize = 2;
        assert_eq!(calc_sum_digit(case1), 4905);
        let case2: usize = 1;
        assert_eq!(calc_sum_digit(case2), 45);
    }
}
