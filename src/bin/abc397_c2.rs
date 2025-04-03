use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let (mut ul, mut ur) = (vec![0; n - 1], vec![0; n - 1]);
    {
        let mut set: HashSet<usize> = HashSet::new();
        for i in 0..n - 1 {
            set.insert(a[i]);
            ul[i] = set.len();
        }
    }
    {
        let mut set: HashSet<usize> = HashSet::new();
        for i in (1..n).rev() {
            set.insert(a[i]);
            ur[i - 1] = set.len();
        }
    }
    let mut ans = 0;
    for i in 0..n - 1 {
        ans = ans.max(ul[i] + ur[i]);
    }

    println!("{}", ans);
    return;
}
