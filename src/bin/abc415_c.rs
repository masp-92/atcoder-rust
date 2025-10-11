use std::collections::HashSet;
use proconio::input;
use proconio::marker::Chars;


fn make_ng(mut n: usize) -> HashSet<usize> {
    let mut cnt = 1usize;
    let mut res = HashSet::new();
    while n > 0 {
        if n % 2 == 1 {
            res.insert(cnt)
        }

        n /= 2;
    }

    return res
}

fn case() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ng = HashSet::new();
    for i in 0..s.len() {
        if s[i] == '0' {
            continue
        }

        ng.insert(make_ng(i+1));
    }

    let mut empty =


}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t { case() }
}