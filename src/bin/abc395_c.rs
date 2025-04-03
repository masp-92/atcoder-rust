use std::collections::HashMap;
use proconio::input;


const INF: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..n {
        map.entry(a[i]).or_insert(vec![]).push(i);
    }

    let mut ans = INF;
    for (_, value) in &map {
        if value.len() < 1 {
            continue;
        }

        for w in value.windows(2) {
            ans = ans.min(w[1] - w[0] + 1);
        }
    }

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);

    }
}
