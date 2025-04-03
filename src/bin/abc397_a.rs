use proconio::input;

fn main() {
    input! {
        x: f64,
    }

    if x >= 38.0 {
        println!("1");
        return;
    }

    if x < 37.5 {
        println!("3");
        return;
    }

    println!("2");
    return;
}
