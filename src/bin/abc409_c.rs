use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        d: [usize; n-1],
    }

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut cnt = vec![0; l];
    let mut now = 0;
    cnt[0] = 1;

    for di in d {
        now = (now + di) % l;
        cnt[now] += 1;
    }

    let dif: usize = l / 3;
    let mut ans: usize = 0;
    for i1 in 0..dif {
        let i2 = i1 + dif;
        let i3 = i2 + dif;
        ans += cnt[i1] * cnt[i2] * cnt[i3];
    }
    println!("{}", ans);
}