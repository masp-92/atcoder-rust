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
    let mut sumb = 0;
    let mut maxw = 0;
    for i in 0..n {
        sumb = sumb + b[i];
        if i < m {
            maxw = maxw.max(maxw + w[i]);
        }
        ans = ans.max(sumb + maxw);
    }

    println!("{}", ans);
}
