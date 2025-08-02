use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut graph = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! {
            a: usize,
            b: usize,
        }
        let (a, b) = (a - 1, b - 1);

        graph[a].push(b);
        graph[b].push(a);
    }

    let root = 0;
    let mut que = VecDeque::new();

    que.push_back(root);

    let mut depth = vec![usize::MAX; n];
    depth[root] = 0;

    while let Some(node) = que.pop_front() {
        for &next in &graph[node] {
            if depth[next] != usize::MAX {
                continue
            }

            que.push_back(next);
            depth[next] = depth[node] + 1;
        }
    }

    for _ in 0..q {
        input! {
            mut c: usize,
            mut d: usize,
        }
        let (c, d) = (c - 1, d - 1);
        if (depth[c] + depth[d]) % 2 == 0 {
            println!("Town")
        } else {
            println!("Road")
        }
    }

    // println!("{:?}", depth);
}
