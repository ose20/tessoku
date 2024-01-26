use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let cum = calc_cum(a, n);

    for &(l, r) in lr.iter() {
        let hit = cum[r] - cum[l-1];
        let miss = (r as i32) + 1 - (l as i32) - hit;

        match hit.cmp(&miss) {
            Ordering::Greater => println!("win"),
            Ordering::Less => println!("lose"),
            Ordering::Equal => println!("draw"),
        }
    }

}

fn calc_cum(a: Vec<i32>, n: usize) -> Vec<i32> {
    let mut cum = vec![0; n+1];

    for i in 1..=n {
        cum[i] = cum[i-1] + a[i-1];
    }

    cum
}
