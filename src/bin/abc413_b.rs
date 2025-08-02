use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut set: HashSet<String> = HashSet::new();
    let mut s = Vec::new();
    for _ in 0..n {
        input! { si: String }
        s.push(si)
    }

    for i in 0..n-1 {
        for j in i+1..n {
            set.insert(format!("{}{}", s[i], s[j]));
            set.insert(format!("{}{}", s[j], s[i]));
        }
    }

    println!("{}", set.len())
}
