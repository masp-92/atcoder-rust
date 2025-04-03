use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        _n: usize,
        m: usize,
    }

    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let mut ans = 0;
    for _ in 0..m {
        input! {u: usize, v: usize}
        if u == v {
            ans += 1;
            continue;
        }

        let edge = if u > v { (u, v) } else { (v, u) };
        if set.contains(&edge) {
            ans += 1;
            continue;
        }
        set.insert(edge);
    }

    println!("{}", ans);
}
