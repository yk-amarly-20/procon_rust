use std::io::*;
use std::{str::FromStr};

pub fn read<T: FromStr>() -> T {
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

fn read_card() -> (usize, Vec<Card>) {
    let n: usize = read();
    let cards: Vec<Card> = (0..n).map(|_| {
        let s: String = read();
        let mut c_itr = s.chars();
        let suit: char = c_itr.next().unwrap();
        let value: char = c_itr.next().unwrap();
        return Card {suit, value}
    }).collect();

    return (n, cards)
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Card {
    suit: char,
    value: char,
}

fn main() {
    let (n, cards) = read_card();
    let mut bubble_cards: Vec<Card> = cards.clone();
    let mut insertion_cards: Vec<Card> = cards.clone();

    bubble_sort(&mut bubble_cards);
    insertion_sort(&mut insertion_cards);
    println!("{}", bubble_cards[0].value);
}

fn bubble_sort<T: PartialOrd + Copy>(source: &mut [T]) {
    let mut flag = true;
    while flag {
        flag = false;
        for i in 1..source.len() {
            if source[i - 1] > source[i] {
                source.swap(i, i + 1);
                flag = true;
            }
        }
    }
}

fn insertion_sort<T: PartialOrd + Copy>(source: &mut [T]) {
    for i in 1..source.len() {
        let v = source[i];
        let mut j = i;
        while j > 0 && source[j - 1] > v {
            source[j] = source[j - 1];
            j -= 1;
        }
        source[j] = v;
    }
}
