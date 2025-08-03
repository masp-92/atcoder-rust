use proconio::input;

fn main() {
    input! {
        n: usize,
        t: String,
        a: String,
    }

    let tc: Vec<char> = t.chars().collect();
    let ac: Vec<char> = a.chars().collect();

    for i in 0..n {
        if tc[i] == 'o' && tc[i] == ac[i] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}