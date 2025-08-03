use proconio::input;

fn count(a: &Vec<usize>, idx: usize) -> i32 {
    if a[idx] == 0 && a[idx+1] == 1 {
        return 1
    }
    return 0
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut cnt = 0;
    let mut m = vec![0; n+2];
    for _ in 0..q {
        input!{ ai: usize }

        let before = count(&m, ai-1) + count(&m, ai);

        m[ai] = m[ai]^1;

        let after = count(&m, ai-1) + count(&m, ai);

        cnt += after - before;
        println!("{}", cnt);
    }
}