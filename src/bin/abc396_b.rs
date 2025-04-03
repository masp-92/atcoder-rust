use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut stack = vec![0; 100];
    for _i in 0..q {
        input! {
            query: usize,
        }
        if query == 1 {
            input! {
                x: usize,
            }
            stack.push(x);
        } else {
            println!("{}", stack.pop().unwrap());
        }
    }
}
