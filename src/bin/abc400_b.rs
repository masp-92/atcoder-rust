use proconio::input;


fn digit_count(mut n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}


fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let cnt = digit_count(n) - 1;

    let mut ans = 0;
    for i in 0..m+1 {
        if cnt * i > 9 {
            println!("inf");
            return;
        }
        ans += n.pow(i as u32);
    }

    if ans > 1_000_000_000 {
        println!("inf");
        return;
    }

    println!("{}", ans);
}
