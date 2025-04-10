use proconio::input;

fn main() {
    input! {
        a: [usize; 5]
    }

    for i in 0..4 {
        let mut swapped = a.clone();
        swapped.swap(i, i+1);

        let mut yes = true;
        for i in 0..5 {
            if swapped[i] != i + 1 {
                yes = false;
            }
        }

        if yes {
            println!("Yes");
            return;
        }

    }

    println!("No");
    return;
}
