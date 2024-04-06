use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let ans = solve(n, &a);
    println!("{}", ans);
}

fn solve(_n: usize, a: &[usize]) -> usize {
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();

    for ai in a.iter() {
        *map.entry(*ai).or_default() += 1
    }

    let mut ans = 0;
    for (_, v) in map.iter() {
        ans += v * (v - 1) / 2
    }

    ans
}
