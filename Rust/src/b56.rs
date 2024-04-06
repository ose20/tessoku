use proconio::{input, marker::Chars};

struct StringHash {
    modulo: u64,
    // Stringの文字数
    len: usize,
    // Stringのi番目のu64表現
    t: Vec<u64>,
    // h[i] := String[1,i] のハッシュ値 (1-indexd）
    // つまり、h[i] := 100^(i-1) * t[1] + ... + 100^1 * t[i-1] + 100^0 * t[i]
    h: Vec<u64>,
    // pow100[i] := 100^i mod modulo (計算用のメモ)
    pow100: Vec<u64>,
}

impl StringHash {
    fn alpha_to_u64(c: char) -> Option<u64> {
        match c {
            'A'..='Z' => Some(c as u64 - 'A' as u64),
            'a'..='z' => Some(c as u64 - 'a' as u64),
            _ => None,
        }
    }
    fn init(string: &String) -> StringHash {
        let modulo: u64 = 21_4748_3647;
        let len = string.len();

        // t[] の設定
        let mut t = vec![0; len + 1];
        let mut t = vec![0; len + 1];
        for (i, c) in string.chars().enumerate() {
            t[i + 1] = StringHash::alpha_to_u64(c).expect("文字列にはアルファベットのみが含まれる");
        }

        // pow100[] の設定
        let mut pow100 = vec![1; len + 1];
        for i in 1..=len {
            pow100[i] = pow100[i - 1] * 100 % modulo
        }

        // h[] の設定
        let mut h = vec![0; len + 1];
        for i in 1..=len {
            h[i] = (100 * h[i - 1] + t[i]) % modulo
        }

        StringHash {
            modulo,
            len,
            t,
            h,
            pow100,
        }
    }

    // String[l..=r]のハッシュ値
    fn hash(&self, l: usize, r: usize) -> u64 {
        assert!(l <= r);
        assert!(1 <= l && r <= self.len);
        // h[r] - 100^(r-l+1) * h[l-1]
        (self.h[r] + self.modulo - (self.h[l - 1] * self.pow100[r - l + 1] % self.modulo))
            % self.modulo
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q]
    }

    let t_rev: String = s.iter().rev().collect();
    let t: String = s.iter().collect();

    let str_hash = StringHash::init(&t);
    let str_hash_rev = StringHash::init(&t_rev);
    for &(l, r) in lr.iter() {
        if str_hash.hash(l, r) == str_hash_rev.hash(n + 1 - r, n + 1 - l) {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
