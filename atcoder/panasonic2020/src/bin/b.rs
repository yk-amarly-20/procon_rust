use proconio::{input};

// 偶偶 => (heigth / 2) * width
// 偶奇 => (height / 2) * width
// 奇偶 => ((height // 2) * 2 + 1) * (m / 2)
// 奇奇 => ((height // 2) * 2 + 1) * (m // 2) + (height // 2 + 1)

fn main() {
    input! {
        height: usize,
        width: usize,
    }

    if height == 1 || width == 1 {
        println!("{}", 1);
        return;
    }

    match height % 2 {
        0 => {
            // when height is even
            println!("{}", height / 2 * width);
        }

        _ => {
            match width % 2 {
                0 => {
                    println!("{}", (height / 2 * 2 + 1) * (width / 2));
                }
                _ => {
                    println!("{}", (height / 2 * 2 + 1) * (width / 2) + (height / 2 + 1));
                }
            }
        }
    }
}
