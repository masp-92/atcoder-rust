use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // ワーシャルフロイド
    let max = 1_000_000_000_000;
    let mut dist: Vec<Vec<usize>> = vec![vec![max; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            c: usize,
        }

        dist[a - 1][b - 1] = c;
    }

    let mut ans = 0;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }

        for i in 0..n {
            for j in 0..n {
                if dist[i][j] == max {
                    continue;
                }
                ans += dist[i][j];
            }
        }
    }

    println!("{}", ans);
}
