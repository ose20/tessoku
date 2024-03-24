use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!("{}", solve(&a));
}

fn solve(a: &Vec<usize>) -> usize {
    let mut map = HashMap::new();
    a.iter().for_each(|ai| {
        *map.entry(ai).or_insert(0) += 1;
    });

    map.values().fold(0, |acc, x| {
        acc + if *x >= 3 { x*(x-1)*(x-2) / 6 } else { 0 }
    })
}
