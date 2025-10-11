use proconio::input;

fn main() {
	input! {
		n: usize,
		l: [usize; n],
	}

	if l.iter().all(|&x| x == 0) {
		println!("0");
		return
	}

	let mut ans = n+1;
	for i in 0..n {
		if l[i] == 1 {
			ans -= i + 1;
			break
		}
	}
	for i in 0..n {
		if l[n-i-1] == 1 {
			ans -= i + 1;
			break
		}
	}

	println!("{:?}", ans);	
}