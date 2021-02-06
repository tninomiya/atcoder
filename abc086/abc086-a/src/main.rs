// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let even = a * b % 2 == 0;
    println!("{}", if even { "Even" } else { "Odd" });
}
