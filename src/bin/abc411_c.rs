use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut cnt = 0;
    let mut m = vec![0; n];
    for _ in 0..q {
        input!{ mut ai: usize }
        ai -= 1;
        m[ai] = m[ai]^1;

        // print!("{} a[]", ai);
        // for i in 0..n { print!(" {}", m[i])}
        // println!();

        if m[ai] == 1 {
            if n == 1 {
                cnt += 1
            } else if ai == 0 {
                if m[ai+1] == 0 {
                    cnt += 1
                }
            } else if ai == n-1 {
                if m[ai-1] == 0 {
                    cnt += 1
                }
            } else {
                if m[ai-1] == 0 && m[ai+1] == 0 {
                    cnt += 1;
                } else if m[ai-1] == 1 && m[ai+1] == 1 {
                    cnt -= 1;
                }
            }
        } else {
            if n == 1 {
                cnt -= 1
            }else if ai == 0 {
                if m[ai+1] == 0 {
                    cnt -= 1
                }
            } else if ai == n-1 {
                if m[ai-1] == 0 {
                    cnt -= 1
                }
            } else {
                if m[ai - 1] == 0 && m[ai + 1] == 0 {
                    cnt -= 1;
                } else if m[ai - 1] == 1 && m[ai + 1] == 1 {
                    cnt += 1;
                }
            }
        }
        println!("{}", cnt);
    }
}