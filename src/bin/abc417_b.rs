use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut d = vec![false; n];
    for bi in b {
        for i in 0..n {
            if !d[i] && bi == a[i] {
                d[i] = true;
                break;
            }
        }
    }

    for i in 0..n {
        if !d[i] {
            println!("{}", a[i]);
        }
    }
}