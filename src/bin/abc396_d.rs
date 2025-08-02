use proconio::input;

fn dfs(node: usize, mut visited: Vec<bool>, graph: &Vec<Vec<(usize, usize)>>, w: usize) -> usize {
    visited[node] = true;

    if graph.len() - 1 == node {
        return w;
    }

    let mut ans = usize::MAX;
    for (neighbor, nw) in &graph[node] {
        if !visited[*neighbor] {
            ans = ans.min(dfs(*neighbor, visited.clone(), graph, w ^ nw));
        }
    }

    return ans;
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for _i in 0..m {
        input! {
            mut u: usize,
            mut v: usize,
            wi: usize,
        }
        u -= 1;
        v -= 1;

        graph[u].push((v, wi));
        graph[v].push((u, wi));
    }

    let ans = dfs(0, vec![false; n], &graph, 0);
    println!("{}", ans);
}
