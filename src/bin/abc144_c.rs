use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in (1..=sqrt_n).rev() {
        if n % i == 0 {
            println!("{}", i + (n / i) - 2);
            return;
        }
    }

    println!("{}", n - 1);
}