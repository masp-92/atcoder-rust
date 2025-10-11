use proconio::input;

fn main() {
	input! {
		x: usize,
		c: usize,
	}

	let n = x / (1000 + c);
	println!("{:?}", 1000 * n);
}
