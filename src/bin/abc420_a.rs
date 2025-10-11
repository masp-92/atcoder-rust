use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    println!("{}", ((x + y - 1) % 12) + 1)
}