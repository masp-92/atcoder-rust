use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! { n: usize, m: usize }

    let mut stacks: Vec<VecDeque<usize>> = vec![VecDeque::new(); m];
    for i in 0..m {
        input! {
            k: usize,
        }

        for _ in 0..k {
            input! { a: usize }
            stacks[i].push_back(a-1);
        }
    }

    let mut top_idx = vec![vec![]; n];
    let mut que = VecDeque::new();
    for i in 0..m {
        if let Some(&c) = stacks[i].back() {
            top_idx[c].push(i);
            if top_idx[c].len() == 2 {
                que.push_back(c);
            }
        }
    }

    while let Some(c) = que.pop_front() {
        for i in top_idx[c].clone() {
            stacks[i].pop_back();
            if let Some(&c2) = stacks[i].back() {
                top_idx[c2].push(i);
                if top_idx[c2].len() == 2 {
                    que.push_back(c2);
                }
            }
        }
    }

    for idx in top_idx {
        if idx.len() != 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
    return;
}