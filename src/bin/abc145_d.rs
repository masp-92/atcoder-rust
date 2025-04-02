use proconio::input;

const MOD: u64 = 1_000_000_007;
const MAX: usize = 2_000_000;

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp /= 2;
    }
    result
}

fn prepare_factorials() -> (Vec<u64>, Vec<u64>) {
    let mut fact = vec![1u64; MAX + 1];
    let mut inv_fact = vec![1u64; MAX + 1];

    for i in 1..=MAX {
        fact[i] = fact[i - 1] * (i as u64) % MOD;
    }

    inv_fact[MAX] = mod_pow(fact[MAX], MOD - 2, MOD); // 逆元（Fermat）
    for i in (1..=MAX - 1).rev() {
        inv_fact[i] = inv_fact[i + 1] * ((i + 1) as u64) % MOD;
    }

    (fact, inv_fact)
}

fn combination_mod(n: u64, k: u64, fact: &[u64], inv_fact: &[u64]) -> u64 {
    if k > n {
        return 0;
    }
    fact[n as usize] * inv_fact[k as usize] % MOD * inv_fact[(n - k) as usize] % MOD
}

fn main() {
    input! {
        x: u64,
        y: u64,
    }

    if (x + y) % 3 != 0 {
        println!("0");
        return;
    }

    let n = (x + y) / 3;
    if x < n || y < n {
        println!("0");
        return;
    }

    let a = x - n;

    let (fact, inv_fact) = prepare_factorials();
    let ans = combination_mod(x + y - 2 * n, a, &fact, &inv_fact);
    println!("{}", ans);
}
