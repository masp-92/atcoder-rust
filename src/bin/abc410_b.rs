use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }

    let mut b = vec![0; n];

    for xi in x {
        if xi == 0 {
            let mut m_idx = 0;
            let mut m = b[0];
            for i in 1..n {
                if m > b[i] {
                    m = b[i];
                    m_idx = i;
                }
            }
            println!("{}", m_idx+1);
            b[m_idx] += 1;
        } else {
            println!("{}", xi);
            b[xi-1] += 1;
        }
    }
}