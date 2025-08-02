use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {s: String, t: String}

    let s_cs: Vec<char> = s.chars().collect();
    let tset: HashSet<char> = t.chars().collect();

    for i in 1..s_cs.len() {
        if s_cs[i].is_uppercase() && !tset.contains(&s_cs[i-1]) {
            println!("No");
            return
        }
    }
    println!("Yes");
}
