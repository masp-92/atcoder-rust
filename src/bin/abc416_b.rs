use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let chars:Vec<char> = s.chars().collect();


    for i in 0..chars.len() {
        if chars[i] == '.' {
            if i == 0 || chars[i-1] == '#' {
                print!("o")
            } else {
                print!(".")
            }
        } else {
            print!("#")
        }
    }
    println!()
}