use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

enum Query {
    Push(usize),
    Peek,
    Pop,
}

fn main() {
    input! {
        q: usize,
    }

    let mut pq = BinaryHeap::new();
    for _ in 0..q {
        proc_query(&mut pq)
    }
}

fn proc_query(pq: &mut BinaryHeap<Reverse<usize>>) {
    match read_query() {
        Query::Push(x) => pq.push(Reverse(x)),
        Query::Peek => println!("{}", pq.peek().unwrap().0),
        Query::Pop => discard(pq.pop()),
    }
}

fn read_query() -> Query {
    input! { n: usize }
    match n {
        1 => {
            input! { x: usize }
            Query::Push(x)
        }
        2 => Query::Peek,
        3 => Query::Pop,
        _ => unreachable!(),
    }
}

fn discard<T>(_v: T) {}
