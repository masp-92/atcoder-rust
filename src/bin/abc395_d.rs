use proconio::input;

fn main() {
    input! {
        n: usize,
        qn: usize,
    }

    let mut map: Vec<usize> = (0..n).collect();
    let mut map_rev: Vec<usize> = (0..n).collect();
    let mut p: Vec<usize> = (0..n).collect();

    for _ in 0..qn {
        input! {
          q: usize,
        }
        if q == 3 {
            input! {
                mut a: usize,
            }
            a -= 1;

            println!("{}", map[p[a]]+1);
        } else if q == 1 {
            input! {
              mut a: usize,
              mut b: usize,
            }
            a -= 1;b -= 1;
            p[a] = map_rev[b];
        } else {
            input! {
                mut a: usize,
                mut b: usize,
            }
            a -= 1;b -= 1;
            let ai = map[a]; let bi = map[b];
            map.swap(ai, bi);
            map_rev.swap(a, b);
        }
    }
}
