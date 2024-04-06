use std::collections::BTreeSet;

use proconio::input;

enum Query {
    Put(usize),
    Ans(usize),
}

fn main() {
    input! {
        q: usize,
    }

    let mut set = BTreeSet::new();
    for _ in 0..q {
        proc_query(&mut set);
    }
}

fn proc_query(set: &mut BTreeSet<usize>) {
    match read_query() {
        Query::Put(x) => {
            set.insert(x);
        }
        Query::Ans(x) => {
            let left = set.range(..x).next_back().map(|l| x.abs_diff(*l));
            let right = set.range(x..).next().map(|r| x.abs_diff(*r));
            match (left, right) {
                (Some(l), Some(r)) => {
                    println!("{}", l.min(r))
                }
                (Some(l), None) => {
                    println!("{}", l)
                }
                (None, Some(r)) => {
                    println!("{}", r)
                }
                _ => {
                    println!("{}", -1)
                }
            }
        }
    }
}

fn read_query() -> Query {
    input! { n: usize, x: usize, }
    match n {
        1 => Query::Put(x),
        2 => Query::Ans(x),
        _ => unreachable!(),
    }
}
