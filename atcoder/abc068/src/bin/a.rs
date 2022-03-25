use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let title = "ABC".to_string() + &n;
    println!("{}", title);
}
