use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        d: usize,
        lr: [(usize, usize); d]
    }

    let left = calc_left(&a, n);
    let right = calc_right(&a, n);

    for &(l, r) in lr.iter() {
        println!("{}", max(left[l-1], right[r+1]));
    }
}

fn calc_left(a: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut left = vec![0; n+2];

    for i in 1..=n {
        left[i] = max(left[i-1], a[i-1]);
    }

    left
}

fn calc_right(a: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut right = vec![0; n+2];

    for i in (1..=n).rev() {
        right[i] = max(right[i+1], a[i-1]);   
    }

    right
}
