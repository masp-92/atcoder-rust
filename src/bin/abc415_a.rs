use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    }

    for i in 0..n{
        if x == a[i] {
            println!("Yes");
            return
        }
    }
    println!("No");
}