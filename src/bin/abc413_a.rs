use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut sum = 0;
    for _ in 0..n {
        input!{ a: usize }
        sum += a;
    }

    if sum <= m {
        println!("Yes")
    } else {
        println!("No")
    }
}
