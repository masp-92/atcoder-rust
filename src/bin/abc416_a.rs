use proconio::input;

fn main() {
    input! {
        _: usize,
        l: usize,
        r: usize,
        s: String,
    }

    let chars:Vec<char> = s.chars().collect();
    for i in l-1..r {
        if chars[i] != 'o' {
            println!("No");
            return;
        }
    }
    println!("Yes")
}