use proconio::input;

fn main() {
    input! {
        n: usize,
    }


    let mut ans = String::new();
    let mut cnt: usize = 0;
    for _ in 0..n {
        input! {
            c: String,
            l: usize,
        }

        cnt += l;
        if cnt > 100 {
            println!("Too Long");
            return;
        }
        ans += &c.repeat(l);
    }

    println!("{}", ans)
}
