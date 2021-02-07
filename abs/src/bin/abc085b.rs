use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }
    let mut map = HashMap::<usize, bool>::new();
    for v in d.iter() {
        map.entry(*v).or_insert(true);
    }
    println!("{}", map.keys().len());
}
