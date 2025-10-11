use proconio::input;

fn main() {
	input!{
		t: usize,
	}

	for _ in 0..t {
		input! {mut a: usize, b: usize, mut c: usize}
		let mut ans = 0;
		let m = a.min(b).min(c);
		ans += m;
		a -= m; c-=m;
		if a == 0 || c == 0 {
			println!("{:?}", ans);
			continue
		}

		ans += a.min(c).min((a + c) / 3);
		println!("{:?}", ans);
	}
}