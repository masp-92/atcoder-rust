use proconio::input;

fn is_palindrome(mut v: usize, n: usize) -> bool {
    let mut vl = Vec::<usize>::new();

    while v > 0 {
        vl.push(v % n);
        v /= n;
    }

    for i in 0..(vl.len()/2) {
        if vl[i] != vl[vl.len() - i  - 1] {
            return false
        }
    }

    return true
}

fn generate_palindromes(limit: usize) -> Vec<usize> {
    let mut res = Vec::new();

    for half in 1.. {
        let s = half.to_string();
        let rev = s.chars().rev().collect::<String>();
        let palindrome = format!("{}{}", s, rev).parse::<usize>().unwrap();
        if palindrome > limit {
            break;
        }
        res.push(palindrome);
    }

    for half in 1.. {
        let s = half.to_string();
        let rev = s.chars().rev().skip(1).collect::<String>(); // 中央の桁は重ねない
        let palindrome = format!("{}{}", s, rev).parse::<usize>().unwrap();
        if palindrome > limit {
            break;
        }
        res.push(palindrome);
    }

    return res;
}

fn main() {
    input! {
        a: usize,
        n: usize,
    }

    let mut ans: usize = 0;
    let ps = generate_palindromes(n);
    for v in ps {
        if is_palindrome(v, a) {
            ans += v;
        }
    }

    println!("{}", ans);
}
