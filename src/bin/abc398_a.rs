use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ei: Vec<usize> = Vec::new();
    if n % 2 == 0 {
        ei.push(n / 2);
        ei.push(n / 2 - 1);
    } else {
        ei.push(n / 2);
    }

    let mut ans = String::new();
    for i in 0..n {
        if ei.contains(&i) {
            ans += "="
        } else {
            ans += "-";
        }
    }
    println!("{}", ans);
}
