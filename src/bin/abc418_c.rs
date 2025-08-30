use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        b: [usize; q],
    }

    let a_max = *a.iter().max().unwrap();
    let mut freq = vec![0usize; a_max+1];
    for ai in a {
        freq[ai] += 1;
    }

    let mut ge = n;
    let mut ans = vec![0; a_max+1];
    ans[1] = 1;
    for i in 2..=a_max {
        ans[i] = ans[i-1] + ge;
        ge -= freq[i-1];
    }

    for i in 0..q {
        if a_max < b[i] {
            println!("-1")
        } else {
            println!("{}", ans[b[i]])
        }
    }
}