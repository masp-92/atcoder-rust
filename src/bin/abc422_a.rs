use proconio::input;

fn main() {
	input!{
		s: String,
	}

	let sc: Vec<char> = s.chars().collect();
	let w: u32 = sc[0].to_digit(10).unwrap();
	let s: u32 = sc[2].to_digit(10).unwrap();
	if s+1 <= 8 {
		println!("{}-{}", w, s+1);
	} else {
		println!("{}-{}", w+1, 1);
	}
}