use proconio::input;

fn int_sqrt(n: usize) -> usize {
    if n < 2 {
        return n;
    }
    let mut lo = 1;
    let mut hi = n / 2 + 1;

    while lo <= hi {
        let mid = (lo + hi) >> 1;
        let sq = mid.saturating_mul(mid); // 念のため saturating_mul すると安全
        if sq == n {
            return mid;
        } else if sq < n {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    hi
}


fn sqrt(n: u64) -> u64 {
    let mut lo = 1;
    let mut hi = n / 2 + 1;

    while lo < hi {
        let mid = (hi + lo) / 2;
        let sq = mid * mid;
        if n > sq {
            lo = mid + 1;
        } else if n < sq {
            hi = mid - 1;
        } else {
            return mid;
        }
    }

    return lo;
}

fn main() {
    input! {
        n: usize,
    }

    let mut ai = 2;
    let mut ans = 0;
    while ai <= n {
        let val = int_sqrt(n / ai);
        ans += (val+1) / 2;
        ai *= 2;
    }

    println!("{}", ans);
}
