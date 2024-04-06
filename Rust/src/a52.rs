use std::collections::VecDeque;

use proconio::input;

enum Query {
    Push(String),
    Front,
    PopFront,
}

fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();
    for _ in 0..q {
        proc_query(&mut queue)
    }
}

fn proc_query(queue: &mut VecDeque<String>) {
    match read_query() {
        Query::Push(x) => queue.push_back(x),
        Query::Front => {
            println!("{}", queue.front().unwrap())
        }
        Query::PopFront => {
            queue.pop_front();
        }
    }
}

fn read_query() -> Query {
    input! { n: usize }
    match n {
        1 => {
            input! { x:String }
            Query::Push(x)
        }
        2 => Query::Front,
        3 => Query::PopFront,
        _ => unreachable!(),
    }
}
