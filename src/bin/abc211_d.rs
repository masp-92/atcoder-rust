use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![]; n];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        let (a, b) = (a-1, b-1);
        g[a].push(b);
        g[b].push(a);
    }

    let m = 1_000_000_007;
    let start = 0;
    let mut que = VecDeque::new();
    let mut dist = vec![usize::MAX; n];
    let mut way = vec![0; n];
    que.push_back(start);
    dist[start] = 0;
    way[start] = 1;

    while let Some(node) = que.pop_front() {
        for &next in &g[node] {
            if dist[next] != usize::MAX {
                if dist[next] == dist[node] + 1 {
                    way[next] = (way[next] + way[node]) % m;
                }
                continue
            }

            que.push_back(next);
            dist[next] = dist[node] + 1;
            way[next] = way[node];
        }
    }

    println!("{}", way[n-1]);
}