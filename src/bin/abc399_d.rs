use proconio::input;
use std::collections::{HashMap, HashSet};

fn _main() {
    input! {
        n: usize,
        a: [usize; 2*n],
    }

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..2 * n {
        map.entry(a[i]).or_insert(vec![]).push(i);
    }

    let mut cand: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..2 * n-1 {
        let v = if a[i] < a[i + 1] {
            (a[i], a[i + 1])
        } else {
            (a[i + 1], a[i])
        };
        cand.insert(v);
    }

    let mut ans = 0;
    for (x, y) in cand {
        let xi = map.get(&x).unwrap()[0] as isize;
        let xj = map.get(&x).unwrap()[1] as isize;
        let yi = map.get(&y).unwrap()[0] as isize;
        let yj = map.get(&y).unwrap()[1] as isize;

        if (xi - xj).abs() == 1 || (yi - yj).abs() == 1 {
            continue;
        }

        if (xi - yi).abs() == 1 && (xj - yj).abs() == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        _main();
    }
}
