use std::collections::VecDeque;

use proconio::input;

#[derive(Debug, Clone)]
enum Query {
    A(String),
    B(String),
    C,
    D,
}

fn main() {
    input! {
        q: usize,
    }

    let queries = parse_queries(q);
    solve(&queries);
}

fn parse_queries(q: usize) -> Vec<Query> {
    (0..q)
        .map(|_| {
            input! { t: char }
            match t {
                'A' => {
                    input! { x: String }
                    Query::A(x)
                }
                'B' => {
                    input! { x: String }
                    Query::B(x)
                }
                'C' => Query::C,
                'D' => Query::D,
                _ => unreachable!(),
            }
        })
        .collect()
}

fn solve(queries: &[Query]) {
    let mut left = VecDeque::new();
    let mut right = VecDeque::new();
    let mut size = 0;

    for query in queries.iter() {
        match query {
            Query::A(x) => {
                if size % 2 == 0 {
                    right.push_back(x);
                    left.push_back(right.pop_front().unwrap());
                } else {
                    right.push_back(x);
                }
                size += 1;
            }
            Query::B(x) => {
                if size % 2 == 0 {
                    left.push_back(x);
                } else {
                    right.push_front(x);
                }
                size += 1;
            }
            Query::C => {
                if size % 2 == 0 {
                    left.pop_front();
                    left.push_back(right.pop_front().unwrap());
                } else {
                    left.pop_front();
                }
                size -= 1;
            }
            Query::D => {
                println!("{}", left.front().unwrap());
            }
        }
    }
}
