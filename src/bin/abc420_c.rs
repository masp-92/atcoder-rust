use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [isize; n],
        mut b: [isize; n],
    }

    let mut ans = 0isize;
    let mut mins = vec![0isize; n];
    for i in 0..n {
        mins[i] = a[i].min(b[i]);
        ans += mins[i];
    }

    for _ in 0..q {
        input! { c: char, mut x: usize, y: isize}
        x -= 1;
        if c == 'A' {
            a[x] = y
        } else {
            b[x] = y
        }
        let min = a[x].min(b[x]);
        ans += min - mins[x];
        mins[x] = min;

        println!("{}", ans);
    }
}