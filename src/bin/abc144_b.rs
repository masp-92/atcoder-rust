use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in 1..10 {
        for j in 1..10 {
            if n == i*j {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}