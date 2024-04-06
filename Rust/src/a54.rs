use std::collections::HashMap;

use proconio::input;

enum Query {
    Update(String, usize),
    Get(String),
}

fn main() {
    input! {
        q: usize,
    }

    let mut map = HashMap::new();
    for _ in 0..q {
        proc_query(&mut map);
    }
}

fn proc_query(map: &mut HashMap<String, usize>) {
    match read_query() {
        Query::Update(name, score) => *map.entry(name).or_default() = score,
        Query::Get(name) => println!("{}", map.get(&name).unwrap()),
    }
}

fn read_query() -> Query {
    input! { n: usize }
    match n {
        1 => {
            input! { name: String, score: usize, }
            Query::Update(name, score)
        }
        2 => {
            input! { name: String }
            Query::Get(name)
        }
        _ => unreachable!(),
    }
}
