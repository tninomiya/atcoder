use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }
    let mut count = 0;
    if n / 100 == 1 {
        count += 1;
        n -= 100;
    }
    if n / 10 == 1 {
        count += 1;
        n -= 10;
    }
    if n == 1 {
        count += 1;
    }

    println!("{}", count);
}
