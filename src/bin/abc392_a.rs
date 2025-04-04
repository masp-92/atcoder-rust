use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        a: [usize; 3]
    }

    for perm in (0..3).permutations(3) {
        let [i, j, k] = perm[..] else { continue };
        if a[i] * a[j] == a[k] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
