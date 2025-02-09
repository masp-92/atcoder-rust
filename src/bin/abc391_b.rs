use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }

    for i in 0..=n-m {
        for j in 0..=n-m {
            let mut ok = true;
            for it in 0..m {
                for jt in 0..m {
                    if s[i+it][j+jt] != t[it][jt] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    break;
                }
            }
            if ok {
                println!("{} {}", i+1, j+1);
                return;
            }
        }
    }
}