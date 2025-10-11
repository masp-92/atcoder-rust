use proconio::input;

fn main() {
	input! {
		n: usize,
		r: usize,
		l: [usize; n],
	}

	let mut left = r;
	for i in 0..r {
		if l[i] == 0 {
			left = i;
			break
		}
	}

	let mut right = r;
	for mut i in r..n {
		i = n - i + r - 1;
		if l[i] == 0 {
			right = i + 1;
			break
		}
	}

	// println!("left: {:?}, right: {}", left, right);
	let mut ans = 0;
	for i in left..r {
		ans += l[i] + 1;
	}

	for i in r..right {
		ans += l[i] + 1;
	}

	println!("{:?}", ans);
}