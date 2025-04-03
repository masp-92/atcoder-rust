use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut s_chars: Vec<char> = s.chars().collect();
    for i in (0..s_chars.len()-1).rev() {
        if s_chars[i] == 'W' && s_chars[i + 1] == 'A' {
            s_chars[i] = 'A';
            s_chars[i + 1] = 'C';
        }
    }

    println!("{}", s_chars.into_iter().collect::<String>());
}
