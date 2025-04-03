use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    for w in a.windows(2) {
        if w[0] >= w[1] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
