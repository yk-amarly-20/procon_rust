use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }

    let mut total_count = 0;
    let mut foreigner_count = 0;

    for c in s.iter() {
        match c {
            'a' => {
                if total_count < a + b {
                    println!("Yes");
                    total_count = total_count + 1;
                }
                else {
                    println!("No");
                }
            }

            'b' => {
                if (total_count < a + b) && (foreigner_count < b) {
                    println!("Yes");
                    total_count = total_count + 1;
                    foreigner_count = foreigner_count + 1;
                }
                else {
                    println!("No");
                }
            }

            _ => {
                println!("No");
            }
        }
    }
}
