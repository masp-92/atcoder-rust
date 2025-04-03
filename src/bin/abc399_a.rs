use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    input! {
        s: String,
        t: String,
    }

    let mut count = 0;
    for i in 0..n {
        if s.chars().nth(i) != t.chars().nth(i) {
            count += 1;
        }
    }
    println!("{}", count);
}