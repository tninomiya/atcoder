// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let even = a * b % 2 == 0;
    println!("{}", if even { "Even" } else { "Odd" });
}
