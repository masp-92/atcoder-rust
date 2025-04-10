use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    for i in 0..n-2 {
        if a[i+1] * a[i+1] != a[i] * a[i + 2] {
            println!("No");
            return;
        }
    }

    println!("Yes");
    return;
}
