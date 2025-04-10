use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        r: isize,
        c: isize,
        s: String,
    }

    let schar = s.chars().collect::<Vec<char>>();
    let mut set: HashSet<(isize, isize)> = HashSet::new();
    let mut h = (r, c);
    let mut f = (0, 0);
    set.insert(f);

    let mut m_map: HashMap<char, (isize, isize)> = HashMap::new();
    m_map.insert('N', (-1, 0));
    m_map.insert('W', (0, -1));
    m_map.insert('S', (1, 0));
    m_map.insert('E', (0, 1));

    let mut ans = String::new();
    for i in 0..n {
        if let Some(&(dy, dx)) = m_map.get(&schar[i]) {
            h = (h.0 - dy, h.1 - dx);
            f = (f.0 - dy, f.1 - dx);
            set.insert(f);
        }

        if set.contains(&h) {
            ans += "1";
        } else {
            ans += "0"
        }

        // println!("-- {} {:?} {:?}", i, h, f);
        // println!("{:?}", set);
    }

    println!("{}", ans);
    return;
}
