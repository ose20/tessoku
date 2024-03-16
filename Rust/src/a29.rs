use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let modulo: i64 = 1_000_000_007;
    println!("{}", mod_pow(a, b, modulo));
}

fn mod_pow(a: i64, b: i64, modulo: i64) -> i64 {
    assert!(b >= 0);
    match b {
        0 => 1,
        b if b % 2 == 0 => {
            let half = mod_pow(a, b/2, modulo);
            (half * half) % modulo
        },
        b if b % 2 == 1 => {
            let half = mod_pow(a, b/2, modulo);
            (((half * half) % modulo) * a) % modulo
        },
        _ => unreachable!()
    }
}
