use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut b: [isize; n],
        mut w: [isize; m],
    }

    b.sort_by(|a, b| b.cmp(a));
    w.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;
    let mut cnt = 0;
    for i in 0..n {
        if b[i] < 0 {
            break;
        }
        ans += b[i];
        cnt += 1;
    }

    for i in 0..cnt.min(m) {
        if w[i] < 0 {
            break;
        }
        ans += w[i];
    }

    for i in cnt..n.min(m) {
        if (b[i] + w[i]) < 0 {
            break;
        }

        ans += b[i] + w[i];
    }

    println!("{}", ans);
}
