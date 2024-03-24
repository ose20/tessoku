use proconio::input;

enum Query {
    Change(usize, usize),
    Reverse,
    Get(usize),
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    solve(n, q);
}

fn solve(n: usize, q: usize) {
    let mut a = vec![0; n+1];
    let mut is_reverse = false;
    for i in 1..=n { a[i] = i; }

    for _ in 0..q {
        match read_query() {
            Query::Change(x, y) => {
                if is_reverse {
                    a[n+1 - x] = y;
                } else {
                    a[x] = y;
                }
            },
            Query::Reverse => {
                is_reverse = !is_reverse;
            }
            Query::Get(x) => {
                if is_reverse {
                    println!("{}", a[n+1 - x]);
                } else {
                    println!("{}", a[x]);
                }
            }
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
                x: usize,
                y: usize,
            }
            Query::Change(x, y)
        },
        2 => {
            Query::Reverse
        },
        3 => {
            input! {
                x: usize,
            }
            Query::Get(x)
        }
        _ => unreachable!()
    }
}
