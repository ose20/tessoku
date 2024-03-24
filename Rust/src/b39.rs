use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut xy: [(usize, usize); n],
    }

    xy.sort();
    let ans = solve(n, d, &xy);
    println!("{}", ans);
}

fn solve(n: usize, d: usize, xy: &Vec<(usize, usize)>) -> usize {
    let mut que = BinaryHeap::new();
    let mut ans = 0;
    let mut cur = 0;

    for i in 1..=d {
        // i 日に受けれられるジョブを追加する
        while cur < n && xy[cur].0 <= i {
            que.push(xy[cur].1);
            cur += 1;
        }

        if let Some(v) = que.pop() {
            ans += v;
        }
    }

    ans
}
