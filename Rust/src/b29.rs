use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let modulo = 1_000_000_007;
    println!("{}", pow(a, b, modulo))
}

fn pow(a: i64, b: i64, modulo: i64) -> i64 {
    assert!(b >= 0);

    if b == 0 { 1 }
    else if b%2 == 0 {
        let sqrt = pow(a, b/2, modulo);
        sqrt * sqrt % modulo
    } else {
        let sqrt = pow(a, b/2, modulo);
        sqrt * sqrt % modulo * a % modulo
    }
}

