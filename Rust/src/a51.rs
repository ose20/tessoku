use proconio::input;

enum Query {
    Push(String),
    Peek,
    Pop,
}

fn main() {
    input! {
        q: usize,
    }

    let mut stack = Vec::new();
    for _ in 0..q {
        proc_query(&mut stack);
    }
}

fn proc_query(stack: &mut Vec<String>) {
    match read_query() {
        Query::Push(x) => {
            stack.push(x);
        }
        Query::Peek => {
            println!("{}", stack.last().unwrap());
        }
        Query::Pop => {
            stack.pop();
        }
    }
}

fn read_query() -> Query {
    input! {
        n: usize,
    }

    match n {
        1 => {
            input! {
                x: String
            }
            Query::Push(x)
        }
        2 => Query::Peek,
        3 => Query::Pop,
        _ => unreachable!(),
    }
}
