use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    let mut res = 0;
    for i in 0..=a {
        if i * 500 > x {
            break;
        }
        for j in 0..=b {
            if i * 500 + j * 100 > x {
                break;
            }
            let remaining = x - (i * 500 + j * 100);
            if remaining % 50 == 0 && remaining / 50 <= c {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
