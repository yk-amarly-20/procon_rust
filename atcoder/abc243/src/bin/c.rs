use proconio::{input, marker::Chars};
use std::{collections::HashMap, hash::Hash, cmp::{max, min}};

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
        s: Chars
    }

    // y座標でグループ分け、右向き、左向きの最大値、最小値を管理しておく
    let mut rmin = HashMap::new();
    let mut lmax = HashMap::new();

    for i in 0..n {
        let x = xy[i].0;
        let y = xy[i].1;

        if s[i] == 'L' {
           lmax.insert(y, *max(lmax.get(&y).unwrap_or(&-1), &x));
        }
        else {
           rmin.insert(y, *min(rmin.get(&y).unwrap_or(&1001001001), &x));
        }
    }

    let mut flag = false;
    for i in lmax.keys() {
        if lmax.get(i).unwrap_or(&1001001001) >= rmin.get(i).unwrap_or(&-1) {
            flag = true;
            break;
        }
    }

    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
