use proconio::input;

fn case() {
    input!{
        n: usize,
        mut a: [usize; n],
    }

    let start = a[0];
    let end = a[n-1];

    a.sort_unstable();

    let mut now = start;
    let mut cnt = 1;
    while now * 2 < end {
        let idx = a.partition_point(|&v| v <= now * 2);
        if idx == 0 || a[idx-1] == now{
            println!("-1");
            return
        }
        now = a[idx-1];
        cnt += 1;
    }

    println!("{}", cnt+1);
    return
}

fn main() {
    input! {t: usize}

    for _ in 0..t {
        case()
    }
}
