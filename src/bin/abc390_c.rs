use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [String; h]
    }

    let mut maxh: usize = 0;
    let mut maxw: usize = 0;
    let mut minh = h;
    let mut minw = w;

    for i in 0..h {
        let chars = s[i].chars().collect::<Vec<_>>();
        for j in 0..w {
            if chars[j] == '#' {
                maxh = maxh.max(i);
                maxw = maxw.max(j);
                minh = minh.min(i);
                minw = minw.min(j);
            }
        }
    }

    // println!("{} {}", minh, maxh);
    // println!("{} {}", minw, maxw);

    for i in minh..maxh+1 {
        let chars = s[i].chars().collect::<Vec<_>>();
        for j in minw..maxw+1 {
            if chars[j] == '.' {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
    return;
}
