use proconio::input;

const MAX_N: usize = 100_000 * 3 + 100;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt_l = 1;
    let mut hist_l = vec![0; MAX_N];
    hist_l[a[0]] = 1;

    let mut hist_r = vec![0; MAX_N];
    let mut cnt_r = 0;
    for i in 1..n {
        hist_r[a[i]] += 1;
        if hist_r[a[i]] == 1 {
            cnt_r += 1;
        }
    }

    let mut ans = cnt_l + cnt_r;
    for i in 1..n-1 {
        hist_l[a[i]] += 1;
        if hist_l[a[i]] == 1 {
            cnt_l += 1;
        }

        hist_r[a[i]] -= 1;
        if hist_r[a[i]] == 0 {
            cnt_r -= 1;
        }

        ans = ans.max(cnt_l + cnt_r);
    }

    println!("{}", ans);
    return;
}
