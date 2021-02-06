// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }
    let mut count = 0;
    while a.iter().find(|&x| x % 2 == 1).is_none() {
        for i in a.iter_mut() {
            *i /= 2;
        }
        count += 1;
    }
    println!("{}", count);
}
