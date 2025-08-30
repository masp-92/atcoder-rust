use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s_chars: Vec<char> = s.chars().collect();
    let n = s_chars.len();
    let mut prefix = vec![0; n+1];

    for i in 0..n {
        prefix[i+1] = prefix[i] + if s_chars[i] == 't' { 1 } else { 0 }
    }

    let mut ans: f64 = 0.0;
    for si in 0..n-2 {
        for sj in si+2..n {
            if s_chars[si] != 't' || s_chars[sj] != 't' {
                continue
            }

            let rate = (prefix[sj] - prefix[si] - 1) as f64 / ((sj - si) - 1) as f64;
            if rate > ans {
                ans = rate
            }
        }
    }

    println!("{}", ans);
}