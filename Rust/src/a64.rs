use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }

    let adj = create_adj(n, &abc);
    let dist = dijkstra(0, &adj);
    for &d in dist.iter() {
        if d == usize::max_value() {
            println!("-1")
        } else {
            println!("{}", d)
        }
    }
}

fn create_adj(n: usize, abc: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut adj = vec![vec![]; n];

    for &(a, b, c) in abc.iter() {
        adj[a - 1].push((b - 1, c));
        adj[b - 1].push((a - 1, c));
    }

    adj
}

fn dijkstra(start: usize, adj: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let inf = usize::max_value();
    let mut dist = vec![inf; adj.len()];
    dist[start] = 0;
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), start));

    while !que.is_empty() {
        let (Reverse(d), now) = que.pop().unwrap();
        if d > dist[now] {
            continue;
        }

        dist[now] = d;
        for &(next, next_d) in adj[now].iter() {
            if dist[next] > dist[now] + next_d {
                dist[next] = dist[now] + next_d;
                que.push((Reverse(dist[now] + next_d), next))
            }
        }
    }

    dist
}
