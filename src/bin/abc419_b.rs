use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
      Q: usize,
    }

    fn insert_sorted(dq: &mut VecDeque<usize>, val: usize) {
        let pos = dq.binary_search(&val).unwrap_or_else(|e| e);
        dq.insert(pos, val);
    }

    let mut dq = VecDeque::new();
    for _ in 0..Q{
        input! { q: usize }
        if q == 1 {
            input! { x: usize }
            insert_sorted(&mut dq, x);
        } else {
            println!("{}", dq.pop_front().unwrap())
        }
    }
}