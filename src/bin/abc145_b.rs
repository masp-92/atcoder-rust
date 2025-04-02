use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    if n % 2 != 0 {
        println!("No");
        return;
    }

    let half = n / 2;
    if s[..half] != s[half..] {
        println!("No");
        return;
    }

    println!("Yes");
}