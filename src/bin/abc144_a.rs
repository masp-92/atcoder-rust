use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if a > 9 || b > 9 {
        println!("-1");
    } else {
        println!("{}", a * b);
    }
}