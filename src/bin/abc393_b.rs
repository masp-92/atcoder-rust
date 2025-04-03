use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s_char = s.chars().collect::<Vec<char>>();
    let mut ans = 0;
    for i in 0..s_char.len() - 2 {
        for iter in 1..((s_char.len() - i) / 2 + 1) {
            let j = i + iter;
            let k = j + iter;
            if k >= s.len() {
                break;
            }
            if s_char[i] == 'A' && s_char[j] == 'B' && s_char[k] == 'C' {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
