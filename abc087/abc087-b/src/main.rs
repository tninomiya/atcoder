// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    let mut res = 0;
    for i in (0..=a).rev() {
        if i * 500 > x {
            continue;
        }
        for j in (0..=b).rev() {
            if i * 500 + j * 100 > x {
                continue;
            }
            for k in (0..=c).rev() {
                if i * 500 + j * 100 + k * 50 == x {
                    res += 1;
                }
            }
        }
    }
    println!("{}", res);
}
