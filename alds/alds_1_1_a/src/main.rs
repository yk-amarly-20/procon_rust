use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
  let stdin = stdin();
  let stdin = stdin.lock();
  let token: String = stdin
    .bytes()
    .map(|c| c.expect("failed to read char.") as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();

  token.parse().ok().expect("failed to parse token.")
}

fn main() {
  let n: usize = read();
  let mut a: Vec<i32> = (0..n).map(|_| read()).collect();
  println!("{}", a.clone().into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
  for i in 1..n {
    let v = a[i];
    let mut j = i;
    while j > 0 && a[j - 1] > v {
      a[j] = a[j - 1];
      j -= 1;
    }
    a[j] = v;
    println!("{}", a.clone().into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
  }
}
