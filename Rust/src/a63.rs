use std::collections::VecDeque;

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

    let inf = usize::max_value();
    let mut dist = vec![inf; n];
    dist[0] = 0;
    let mut que = VecDeque::new();
    que.push_back(0);

    while !que.is_empty() {
        let now = que.pop_front().unwrap();
        for &next in adj[now].iter() {
            if dist[next] == inf {
                dist[next] = dist[now] + 1;
                que.push_back(next);
            }
        }
    }

    for i in 0..n {
        println!("{}", if dist[i] == inf { -1 } else { dist[i] as i64 })
    }
}
