use proconio::input;

fn main() {
    input! {
        n: usize,
        mut cards: [u32; n],
    }
    cards.sort();
    let mut alice = 0;
    let mut bob = 0;
    for (i, v) in cards.iter().rev().enumerate() {
        if i % 2 == 0 {
            alice += v;
        } else {
            bob += v;
        }
    }
    println!("{}", alice - bob);
}
