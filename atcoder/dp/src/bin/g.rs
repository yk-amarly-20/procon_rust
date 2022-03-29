use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // g[x]: xを始点とする終点の集合
    let mut g = vec![vec![]; n];
    // memo[x]: xを始点とする最大有向パス長
    let mut memo = vec![None; n];

    for _ in 0..m {
        input!(x: usize, y: usize);
        g[x - 1].push(y - 1);
    }

    for i in 0..n {
        rec(i, &g, &mut memo);
    }

    println!("{}", memo.iter().max().unwrap().unwrap());

}

fn rec(i: usize, g: &Vec<Vec<usize>>, memo: &mut Vec<Option<usize>>) -> usize {
    // i始点の最大有効パス長を返す
    // 最大長はmemoに格納してメモ化dp
    // まだ最大長を算出できていないindexについてはmemo[idx] = None
    if let Some(val) = memo[i] {
        return val;
    }

    let mut fmax = 0;
    for &child in g[i].iter() {
        fmax = fmax.max(rec(child, g, memo) + 1);
    }

    memo[i] = Some(fmax);
    return fmax;
}
