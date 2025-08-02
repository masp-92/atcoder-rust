use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! { n: usize, m: usize }

    let mut from = vec![vec![]; n];
    let mut cnt = vec![0; n];

    for _ in 0..m {
        input! {
            k: usize,
            a: [usize; n]
        }

        for i in 0..k-1 {
            let (a1, a2) = (a[i]-1, a[i+1]-1);
            cnt[a1] += 1;
            from[a1].push(a2);
        }
    }
    
    let mut que = VecDeque::new();
    for i in 0..n {
        if cnt[i] == 0 {
            que.push_back(i);
        }
    }

    while let Some(node) = que.pop_front() {
        for &x in &from[node] {
            cnt[x] -= 1;
            if cnt[x] == 0 {
                que.push_back(x);
            }
        }
    }

    for c in cnt {
        if c != 0 {
            println!("No");
            return
        }
    }

    println!("Yes");
}