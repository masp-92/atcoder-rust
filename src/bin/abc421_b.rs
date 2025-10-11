use proconio::input;

fn rev(a: usize) -> usize {
	let s = a.to_string();
	let rs: String = s.chars().rev().collect();
	return rs.parse().unwrap();
}

fn main() {
	input! {
		x: usize,
		y: usize,
	}

	let mut a1 = x;
	let mut a2 = y;

	for _ in 0..8 {
		let next = rev(a2 + a1);
		a1 = a2;
		a2 = next;
		// println!("{:?}", next);
	}
	println!("{:?}", a2);
}