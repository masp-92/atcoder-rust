use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    for i in (0..=100).rev() {
        let mut cnt = 0;
        for ai in &a {
            if *ai >= i {
                cnt += 1
            }
        }
        if cnt >= i {
            println!("{}", i);
            return;
        }
    }
}