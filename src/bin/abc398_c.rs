use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut map: HashMap<isize, isize> = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut ans: isize = -1;
    let mut maxv: isize = 0;
    for i in 0..n {
        if map[&a[i]] != 1 {
            continue;
        }

        if maxv < a[i] {
            maxv = a[i];
            ans = i as isize + 1;
        }
    }

    println!("{}", ans);
}
