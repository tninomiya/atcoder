use proconio::input;

fn main() {
    input! {
        n: usize,
        y: u64,
    }
    let y = y as usize / 1000;

    for i in 0..=n {
        if 10 * i > y {
            break;
        }
        for j in 0..=(n - i) {
            if 10 * i + 5 * j > y {
                break;
            }

            let k = n - i - j;
            if 10 * i + 5 * j + k == y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
