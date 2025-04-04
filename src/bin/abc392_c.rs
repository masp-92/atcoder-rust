use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut a: HashSet<usize> = HashSet::new();
    for _i in 0..m {
        input! { _a: usize }
        a.insert(_a);
    }

    let mut x = vec![0; 0];
    for i in 1..n+1 {
        if !a.contains(&i) {
            x.push(i);
        }
    }

    println!("{}", x.len());
    for xi in x {
        print!("{} ", xi);
    }
    println!();


}
