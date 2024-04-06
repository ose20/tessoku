use std::collections::BTreeSet;

use proconio::input;

enum Query {
    Insert(usize),
    Remove(usize),
    Search(usize),
}

fn main() {
    input! { q: usize }

    let mut set = BTreeSet::new();
    for _ in 0..q {
        proc_query(&mut set);
    }
}

fn proc_query(set: &mut BTreeSet<usize>) {
    match read_query() {
        Query::Insert(x) => discard(set.insert(x)),
        Query::Remove(x) => discard(set.remove(&x)),
        Query::Search(x) => {
            if let Some(y) = set.range(x..).next() {
                println!("{}", y)
            } else {
                println!("-1")
            }
        }
    }
}

fn read_query() -> Query {
    input! { n: usize }
    match n {
        1 => {
            input! { x: usize }
            Query::Insert(x)
        }
        2 => {
            input! { x: usize }
            Query::Remove(x)
        }
        3 => {
            input! { x: usize }
            Query::Search(x)
        }
        _ => unreachable!(),
    }
}

fn discard<T>(_val: T) {}
