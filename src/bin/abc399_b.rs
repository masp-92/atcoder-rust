use proconio::input;

const MAX_VAL: usize = 200;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut hist = [0; MAX_VAL];
    p.iter().for_each(|&pi| hist[pi] += 1);

    let mut rank = [0; MAX_VAL];
    let mut idx = 1;
    for value in (0..MAX_VAL).rev() {
        rank[value] = idx;
        idx += hist[value];
    }

    for &pi in &p {
        println!("{}", rank[pi]);
    }
}
