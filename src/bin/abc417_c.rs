use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    // let mut left = vec![0; n];
    let mut right = vec![0usize; n];
    let mut ans: usize = 0;
    for i in 0..n {
        if a[i] <= i {
            ans += right[i - a[i]];
        }

        let r = i + a[i];
        if r < n {
            right[r] += 1;
        }
    }

    println!("{}", ans);
}