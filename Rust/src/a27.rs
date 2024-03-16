use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    println!("{}", gcd(a, b));
}

fn gcd(x: i32, y: i32) -> i32 {
    assert!(x >= 0 && y >= 0);
    let (x, y) = (x.max(y), x.min(y));

    if x == 0 { y }
    else if y == 0 { x }
    else {
        let r = x % y;
        gcd(y, r)
    }
}