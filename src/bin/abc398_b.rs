use std::collections::HashMap;
use proconio::input;
use itertools::Itertools;

fn main() {
    let n = 7;
    input! {
        a: [usize; n],
    }

    for perm in (0..n).combinations(5) {
        let mut hist: HashMap<usize, usize> = HashMap::new();
        for i in perm {
            *hist.entry(a[i]).or_insert(0) += 1;
        }

        if hist.len() == 2 {
            let counts: Vec<usize> = hist.values().cloned().sorted().collect();
            if counts == [2, 3] {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}
