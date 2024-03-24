use proconio::input;

struct Comb {
    modulo: i64,
    // fac[i] := i! mod modulo
    fac: Vec<i64>,
    // inv[i] := i^(-1) mod modulo
    inv: Vec<i64>,
    // fac_inv[i] := (i!)^(-1) mod modulo
    fac_inv: Vec<i64>,
}

impl Comb {
    fn init(n: i64, modulo: i64) -> Comb {
        // O(n)の前処理
        // nCk (k<=n)が求められるようになる
        let size: usize = n as usize;
        let mut fac = vec![0; size+1];
        let mut inv = vec![0; size+1];
        let mut fac_inv = vec![0; size+1];

        fac[0] = 1;
        if size>=1 { fac[1] = 1; }
        fac_inv[0] = 1;
        if size>=1 { fac_inv[1] = 1; }
        if size>=1 { inv[1] = 1; }
        for i in 2..=size {
            fac[i] = (fac[i-1] * (i as i64)) % modulo;
            // modulo と i (2 <= i <= n) は互いに素である必要がある (moduloが素数であれば十分)
            inv[i] = modulo - inv[(modulo as usize) % i] * (modulo / (i as i64)) % modulo;
            fac_inv[i] = fac_inv[i-1] * inv[i] % modulo
        }

        Comb {
            modulo,
            fac,
            inv,
            fac_inv,
        }
    }

    fn com(&self, n: i64, k: i64) -> i64 {
        let nsize = n as usize;
        let ksize = k as usize;
        if n < k { 0 }
        else if n < 0 || k < 0 { 0 }
        else { self.fac[nsize] * (self.fac_inv[ksize] * self.fac_inv[nsize - ksize] % self.modulo) % self.modulo }
    }
}

fn main() {
    input! {
        h: i32,
        w: i32,
    }

    let modulo = 1_000_000_007;
    let com = Comb::init((h-1 + w-1).into(), modulo);
    println!("{}", com.com((h+w-2).into(), (h-1).into()));
}
