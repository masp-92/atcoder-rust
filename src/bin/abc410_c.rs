use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut a: Vec<usize> = (1..n+1).collect();

    let mut offset: usize = 0;
    for _ in 0..q {
        input!{t: usize}
        match t {
            1=> {
                input!{p: usize, x: usize}
                let idx = (offset + p - 1) % n;
                a[idx] = x;
            }
            2 => {
                input!{p: usize}
                let idx = (offset + p - 1) % n;
                println!("{}", a[idx]);
            }
            3 => {
                input!{k: usize}
                offset = (offset + k) % n;
            }
            _ => unreachable!(),
        }
    }
}