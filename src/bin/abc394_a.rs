use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut cnt = 0;
    for c in s.chars() {
        if c == '2' {
            cnt += 1;
        }
    }

    println!("{}", "2".repeat(cnt));
}
