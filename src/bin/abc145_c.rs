use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut xs: Vec<f64> = vec![];
    let mut ys: Vec<f64> = vec![];
    for _i in 0..n {
        input! {
            x: f64,
            y: f64,
        }

        xs.push(x);
        ys.push(y);
    }

    let mut dis = 0.0;
    let mut count = 0_usize;

    for perm in (0..n).permutations(n) {
        let mut route_dist = 0.0;
        for w in perm.windows(2) {
            let i = w[0];
            let j = w[1];
            let dx = xs[i] - xs[j];
            let dy = ys[i] - ys[j];
            route_dist += (dx * dx + dy * dy).sqrt();
        }

        dis += route_dist;
        count += 1;
    }

    let avg = dis / count as f64;

    println!("{:.10}", avg);
}
