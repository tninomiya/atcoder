use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [(usize, (usize, usize)); n],
    }
    let mut prev_t = 0;
    let mut prev_p = (0, 0);
    for p in v.iter() {
        if !possible(p.0 - prev_t, &prev_p, &p.1) {
            println!("No");
            return;
        }
        prev_t = p.0;
        prev_p = p.1;
    }
    println!("Yes");
}

fn possible(t: usize, from: &(usize, usize), to: &(usize, usize)) -> bool {
    let diff_x = if from.0 > to.0 {
        from.0 - to.0
    } else {
        to.0 - from.0
    };
    let diff_y = if from.1 > to.1 {
        from.1 - to.1
    } else {
        to.1 - from.1
    };

    diff_x + diff_y <= t && (t - (diff_x + diff_y)) % 2 == 0
}
