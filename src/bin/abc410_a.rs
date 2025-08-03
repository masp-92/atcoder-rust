use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        k: usize,
    }

    let mut cnt = 0;
    for ai in a {
        if ai >= k {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}