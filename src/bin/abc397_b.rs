use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s_vec: Vec<char> = s.chars().collect();
    let mut cnt = 0;

    for i in 0..s_vec.len() {
        let expected = if (i + cnt) % 2 == 0 { 'i' } else { 'o' };
        if s_vec[i] != expected {
            cnt += 1;
        }
    }

    if (cnt + s_vec.len()) % 2 == 1 {
        cnt += 1;
    }

    println!("{}", cnt);
    return;
}
