use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut flag = false;
    // 横向き
    for i in 0..n {
        for j in 0..(n - 5) {
            let blocks = vec![s[i][j], s[i][j + 1], s[i][j + 2], s[i][j + 3], s[i][j + 4], s[i][j + 5]];
            flag = flag || can_black_six(&blocks);
        }
    }

    // 縦向き
    for i in 0..(n - 5) {
        for j in 0..n {
            let blocks = vec![s[i][j], s[i + 1][j], s[i + 2][j], s[i + 3][j], s[i + 4][j], s[i + 5][j]];
            flag = flag || can_black_six(&blocks);
        }
    }

    // 斜め(右肩下がり)
    for i in 0..(n - 5) {
        for j in 0..(n - 5) {
            let blocks = vec![s[i][j], s[i + 1][j + 1], s[i + 2][j + 2], s[i + 3][j + 3], s[i + 4][j + 4], s[i + 5][j + 5]];
            flag = flag || can_black_six(&blocks);
        }
    }

    // 斜め（右肩あがり）
    for i in 5..n {
        for j in 0..(n - 5) {
            let blocks = vec![s[i][j], s[i - 1][j + 1], s[i - 2][j + 2], s[i - 3][j + 3], s[i - 4][j + 4], s[i - 5][j + 5]];
            flag = flag || can_black_six(&blocks);
        }
    }

    if flag {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

fn can_black_six(blocks: &Vec<char>) -> bool {
    blocks.iter().filter(|&b| b == &'#')
        .collect::<Vec<&char>>()
        .len()
        >= 4
}
