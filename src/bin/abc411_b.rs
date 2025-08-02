use proconio::input;

fn main() {
    input! {n: usize, d: [usize; n-1]}

    for i in 0..n-1 {
        let mut v = d[i];
        print!("{}", v);
        for j in i+1..n-1 {
            v += d[j];
            print!(" {}", v)
        }
        println!()
    }
}