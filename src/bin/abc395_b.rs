use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![vec![false; n]; n];

    for i in 0..n {
        let j = n - i;
        if i > j {
            continue;
        }

        let v = i % 2 == 0;
        for k in i..j {
            for l in i..j {
                ans[k][l] = v;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}", if ans[i][j] { "#" } else { "." });
        }
        println!();
    }
}
