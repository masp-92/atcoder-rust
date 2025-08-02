use proconio::input;
use std::collections::VecDeque;

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> usize {
    let mut visited = vec![false; graph.len()];
    visited[start] = true;
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut cnt = 0;
    while let Some(node) = queue.pop_front() {
        cnt += 1;
        for i in &graph[node] {
            if visited[*i] {
                continue;
            }
            visited[*i] = true;
            queue.push_back(*i);
        }
    }

    cnt
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        input! {a: usize, b: usize}
        graph[a - 1].push(b - 1);
    }

    let mut ans = 0;
    for i in 0..n {
        ans += bfs(&graph, i);
    }

    println!("{}", ans);
}
