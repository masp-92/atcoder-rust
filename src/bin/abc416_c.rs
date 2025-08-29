use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        s: [String; n],
    }

    let m = n.pow(k as u32);
    let mut ps = Vec::new();

    for mut i in 0..m {
        let mut joined = String::new();
        for _ in 0..k {
            joined += &s[i % n];
            i /= n;
        }
        ps.push(joined);
    }

    ps.sort();

    // println!("{:?}", ps);

    println!("{}", ps[x-1]);
}