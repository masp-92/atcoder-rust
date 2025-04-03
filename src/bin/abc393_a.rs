use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
    }

    let mut o = vec![1; 4];
    if s1 == "sick" {
        o[2] = 0; o[3] = 0;
    } else {
        o[0] = 0; o[1] = 0;
    }

    if s2 == "sick" {
        o[1] = 0; o[3] = 0;
    } else {
        o[0] = 0; o[2] = 0;
    }

    for i in 0..o.len() {
        if o[i] == 1 {
            println!("{}", i+1)
        }
    }
}
