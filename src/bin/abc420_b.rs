use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut score = vec![0usize; n];
    for j in 0..m {
        let mut x = 0;
        for i in 0..n {
            if s[i][j] == '0' {
                x += 1;
            }
        }

        let inc = if 2 * x > n {
            '1'
        } else {
            '0'
        };

        for i in 0..n {
            if s[i][j] == inc {
                score[i] += 1
            }
        }
    }

    let max = *score.iter().max().unwrap();
    for i in 0..n {
        if score[i] == max {
            println!("{}", i + 1)
        }
    }
}