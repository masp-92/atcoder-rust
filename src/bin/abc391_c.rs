use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }


    let mut sum: Vec<usize> = vec![1; n];
    let mut pos: Vec<usize> = (0..n).collect();
    let mut cnt = 0;

    for _ in 0..q {
        input!{i: usize}
        if i == 1 {
            input! {
                q: usize,
                h: usize,
            }
            let p = q - 1;
            let h = h - 1;

            sum[pos[p]] -= 1;
            if sum[pos[p]] == 1 {
                cnt -= 1;
            }

            sum[h] += 1;
            if sum[h] == 2 {
                cnt += 1;
            }

            pos[p] = h;
        } else {
            println!("{}", cnt);
        }
    }
}