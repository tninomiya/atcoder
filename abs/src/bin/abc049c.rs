use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut s: &str = &s;
    loop {
        let l = s.len();
        if s.ends_with("dreamer") {
            s = &s[..l - 7];
        } else if s.ends_with("eraser") {
            s = &s[..l - 6];
        } else if s.ends_with("dream") || s.ends_with("erase") {
            s = &s[..l - 5];
        } else {
            break;
        }
    }
    if s.len() == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
