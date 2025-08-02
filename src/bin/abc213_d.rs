use proconio::input;

fn dfs(g: &Vec<Vec<usize>>, n: usize, route: &mut Vec<usize>, p: usize) {
    route.push(n);
    for &next in &g[n] {
        if p == next {
            continue
        }
        dfs(g, next, route, n);
        route.push(n);
    }
}

fn main() {
    input! {
        n: usize,
    }

    let mut g = vec![vec![]; n];
    for _ in 0..n-1 {
        input! { a: usize, b: usize}
        let (a, b) = (a - 1, b - 1);
        g[a].push(b);
        g[b].push(a);
    }

    for l in &mut g {
        l.sort();
    }

    let mut ans = vec![];
    dfs(&g, 0, &mut ans, 0);
    for a in ans {
        print!("{} ", a+1);
    }
    println!();
}