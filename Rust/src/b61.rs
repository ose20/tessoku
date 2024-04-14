use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut adj = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        adj[a - 1].push(b - 1);
        adj[b - 1].push(a - 1);
    }

    let mut tmp = adj
        .iter()
        .enumerate()
        .map(|(i, lst)| (i + 1, lst.len()))
        .collect::<Vec<_>>();

    tmp.sort_by_key(|v| v.1);
    println!("{}", tmp.last().unwrap().0);
}
