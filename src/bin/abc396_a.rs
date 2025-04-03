use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    for i in 2..n {
        if a[i - 2] == a[i - 1] && a[i - 1] == a[i] {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
