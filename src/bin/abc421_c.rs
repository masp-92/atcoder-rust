use proconio::input;

fn main() {
	input!{
		n: usize,
		s: String,
	}

	let sc: Vec<char> = s.chars().collect();
	let mut ai: Vec<usize> = Vec::new();
	for i in 0..2*n {
		if sc[i] == 'A' {
			ai.push(i);
		}
	}


	let mut ans1 = 0;
	let mut ans2 = 0;
	for i in 0..ai.len() {
		let e = i*2;
		ans1 += e.abs_diff(ai[i]);
		ans2 += (e+1).abs_diff(ai[i])
	}


	println!("{:?}", ans1.min(ans2))
}


