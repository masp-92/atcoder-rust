use proconio::input;

fn main() {
    input! {
      n: usize,
    }

    let (mut r_max, mut r_min) = (0usize, usize::MAX);
    let (mut c_max, mut c_min) = (0usize, usize::MAX);
    for _ in 0..n {
        input! { r: usize, c: usize }
        r_max = r_max.max(r);
        r_min = r_min.min(r);
        c_max = c_max.max(c);
        c_min = c_min.min(c);
    }

    let d_max = (r_max - r_min).max(c_max - c_min);
    println!("{}", (d_max + 1) / 2);
}