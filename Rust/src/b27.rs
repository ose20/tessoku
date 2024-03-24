use std::ops::{Rem, Sub};

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,       
    }

    println!("{}", a * b / gcd(a, b));

}

fn gcd<T>(x: T, y: T) -> T
where
    T: Copy + Clone + Ord + Rem<Output = T> + Sub<Output = T> + Default,
{
    assert!(x >= T::default() && y >= T::default());
    let (x, y) = (x.max(y), x.min(y));
    if x == T::default() {
        y
    } else if y == T::default() {
        x
    } else {
        gcd(y, x%y)
    }
}