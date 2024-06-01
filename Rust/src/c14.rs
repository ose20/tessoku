use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------
// ダイクストラ法
// adj: 隣接リスト(0-indexed)
// adj の要素は (to, dist)
// 到達不可能な場合は　usize::max_value()が入ってる
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

fn main() {
    input! {
        n: usize, m: usize,
        abc: [(usize, usize, usize); m],
    }

    let adj = get_adj(&abc, n);
    let dist_from_1 = dijkstra(0, &adj);
    let dist_from_n = dijkstra(n - 1, &adj);
    let ans = (0..n)
        .filter(|&i| dist_from_1[i] + dist_from_n[i] == dist_from_1[n - 1])
        .count();
    println!("{}", ans);
}

fn get_adj(abc: &[(usize, usize, usize)], n: usize) -> Vec<Vec<(usize, usize)>> {
    let mut adj = vec![vec![]; n];

    for &(a, b, c) in abc.iter() {
        adj[a - 1].push((b - 1, c));
        adj[b - 1].push((a - 1, c));
    }

    adj
}
