use proconio::input;

fn main() {
    input! {n: usize}

    let mut ans = 0;
    for _ in 0..n {
        input!{a: usize, b: usize}
        if a < b { ans += 1 }
    }

    println!("{}", ans)
}
