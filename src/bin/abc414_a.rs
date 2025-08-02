use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    }

    let mut cnt = 0;
    for _ in 0..n {
        input! {
            x: usize,
            y: usize,
        }

        if x <= l && r <= y {
            cnt += 1;
        }
    }

    println!("{}", cnt)
}
