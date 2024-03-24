use proconio::input;

enum Query {
    Get(usize, usize),
    Swap(usize, usize),
}

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        q: usize,
    }

    solve(n, &a, q);
}

fn solve(n: usize, a: &Vec<Vec<usize>>, q: usize) {
    let mut to_row = vec![0; n];
    for i in 0..n { to_row[i] = i }

    for _ in 0..q {
        match get_query() {
            Query::Swap(x, y) => {
                to_row.swap(x-1, y-1);
            },
            Query::Get(x, y) => {
                println!("{}", a[to_row[x-1]][y-1]);
            },
        }
    }

}

fn get_query() -> Query {
    input! {
        c: usize,
        x: usize,
        y: usize,
    }

    match c {
        1 => Query::Swap(x, y),
        2 => Query::Get(x, y),
        _ => unreachable!()
    }
}
