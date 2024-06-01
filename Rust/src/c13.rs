use std::collections::HashMap;

use proconio::input;

// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------
// 拡張ユークリッドの互除法
// ax + by = gcd(a, b) となる整数解 (x,y) と d = gcd(a,b) を求める
// 返り値は　 (d, x, y)
// ref. https://qiita.com/drken/items/b97ff231e43bce50199a
pub fn ext_gcd(a: u64, b: u64) -> (u64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, s, t) = ext_gcd(b, a % b);
        let q = (a / b) as i64;
        (d, t, s - q * t)
    }
}

// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------
// a^(-1) を mod m で求める
// ax = 1 mod m となる x を求めたい
// ax - 1 が m で割り切れることと同値
// ax + my = 1 となる整数 y が存在し、この時の x を求めることと同値
// これが存在するための条件（つまりこの関数が使える条件）は、a と m が互いに素であること（pが素数である必要はない）
// ref. https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a
pub fn modinv(a: u64, m: u64) -> u64 {
    let (d, mut x, _) = ext_gcd(a, m);
    assert!(d == 1);

    if x < 0 {
        x += m as i64
    }
    (x as u64) % m
}

const MOD: u64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        p: u64,
        a: [u64; n]
    }

    let map = get_map(&a);
    let ans = count_ans(&map, p, n as u64);
    println!("{}", ans)
}

fn get_map(a: &[u64]) -> HashMap<u64, u64> {
    let mut map = HashMap::new();

    for &ai in a.iter() {
        *map.entry(ai % MOD).or_insert(0) += 1;
    }

    map
}

fn count_ans(map: &HashMap<u64, u64>, p: u64, n: u64) -> u64 {
    let mut ans = 0;

    // 重複をしないように i * j = p mod MOD となる (i, j) (i <= j) を数え上げる
    for (&i, &cnt_i) in map.iter() {
        eprintln!("i: {}, cnt_i: {}", i, cnt_i);
        if i == 0 {
            if p == 0 {
                // 0以外と組み合わせるとき
                ans += cnt_i * (n - cnt_i);
                // 0だけで組み合わせる時
                ans += cnt_i * (cnt_i - 1) / 2;
            } else {
                ans += 0
            }
        } else {
            let j = modinv(i, MOD) * p % MOD;
            if j < i {
                continue;
            }
            let &cnt_j = map.get(&j).unwrap_or(&0);
            if i == j {
                ans += cnt_i * (cnt_i - 1) / 2
            } else {
                ans += cnt_i * cnt_j
            }
        }
    }

    ans
}
