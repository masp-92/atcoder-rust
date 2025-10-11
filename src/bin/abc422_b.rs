use proconio::input;

fn main() {
	input!{
		h: usize,
		w: usize,
		s: [String; h],
	}

	let mut sc: Vec<Vec<char>> = Vec::new();
	for i in 0..h {
		sc.push(s[i].chars().collect())
	}
	
	for i in 0..h {
		for j in 0..w {
			if sc[i][j] != '#' {
				continue
			}

			let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
			let mut cnt = 0;
			for (_di, _dj) in neighbors.iter() {
				let di = _di + (i as isize);
				let dj = _dj + (j as isize);
				if di < 0 || di >= (h as isize) || dj < 0 || dj >= (w as isize) {
					continue
				}

				if sc[di as usize][dj as usize] == '#' {
					cnt += 1;
				}
			}
			if !(cnt == 2 || cnt == 4) {
				println!("No");
				return
			}
		}
	}
	println!("Yes");
}
