use std::ops::Add;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
        lr: [(usize, usize); q],
    }

    let sum = calc_accum(a, 0);

    for &(l, r) in lr.iter() {
        println!("{}", sum[r] - sum[l-1]);
    }

}

/// 累積和を計算する関数
/// Tに和の単位元があることを型が保証していないので、初期値を渡す必要がある
fn calc_accum<T>(a: Vec<T>, init: T) -> Vec<T>
where
    T: Add<Output =T> + Copy
{
    let n = a.len();
    let mut sum = vec![init; n+1];

    for i in 1..=n {
        sum[i] = sum[i-1] + a[i-1];
    }

    sum
}
