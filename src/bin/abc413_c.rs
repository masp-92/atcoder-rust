use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut idx = 0;

    let mut cl = Vec::new();
    let mut xl = Vec::new();
    for _ in 0..q {
        input! {qi: usize}
        if qi == 1 {
            input! {c: usize, x: usize}
            cl.push(c); xl.push(x);
        } else {
            input! {mut k: usize}
            let mut sum = 0;
            while k > 0 {
                let take = k.min(cl[idx]);
                sum += take * xl[idx];
                cl[idx] -= take;
                k -= take;
                if cl[idx] == 0 { idx += 1; }
            }
            println!("{}", sum);
        }
    }

}
