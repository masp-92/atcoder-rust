use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    }

    s.sort_by(|a, b| a.len().cmp(&b.len()));

    let mut ans = String::new();
    for si in &s {
        ans += &si;
    }

    println!("{}", ans);
}
