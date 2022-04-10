use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [char; 3],
        t: [char; 3],
    }

    match is_even_substitution(&s, &t) {
        true => println!("Yes"),
        false => println!("No"),
    }
}

fn is_even_substitution(
    s: &Vec<char>,
    t: &Vec<char>
) -> bool {
    // 全部同じ
    if s[0] == t[0] && s[1] == t[1] {
        return true;
    }
    // 一つだけ同じ
    else if s[0] == t[0] || s[1] == t[1] || s[2] == t[2] {
        return false;
    }
    // 全部違う
    else {
        return true;
    }
}
