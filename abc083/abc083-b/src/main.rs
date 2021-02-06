// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut result = 0;
    for i in 1..=n {
        let total: usize = i.to_string().chars().map(|n| n as usize - 48).sum();
        if total >= a && total <= b {
            result += i;
        }
    }
    println!("{}", result);
}

//fn main() {
//    input! {
//        n: usize,
//        a: usize,
//        b: usize,
//    }
//    let mut result = 0;
//    for i in 1..=n {
//        let total = hold_sum(i);
//        if total >= a && total <= b {
//            result += i;
//        }
//    }
//    println!("{}", result);
//}
//
//fn hold_sum(n: usize) -> usize {
//    if n < 10 {
//        return n;
//    }
//    n % 10 + hold_sum(n / 10)
//}
