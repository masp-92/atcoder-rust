use proconio::input;
use proconio::marker::Chars;


fn main() {
    input! {
        s: Chars,
    }

    let mut ans = Vec::new();
    for i in 0..s.len() {
        if s[i] == '#' {
            ans.push(i+1);
        }
    }

    for i in (0..ans.len()).step_by(2) {
        println!("{},{}", ans[i], ans[i+1])
    }
}