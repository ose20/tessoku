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
    use std::{cmp::Reverse, collections::BinaryHeap};
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

    let adj = create_adj(n, &abc);
    let dist = dijkstra(0, &adj);
    let path = construct_path(0, n - 1, &adj, &dist);
    let ans = path
        .iter()
        .map(|v| (v + 1).to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", ans);
}

fn create_adj(n: usize, abc: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut adj = vec![vec![]; n];

    for &(a, b, c) in abc.iter() {
        adj[a - 1].push((b - 1, c));
        adj[b - 1].push((a - 1, c));
    }

    adj
}

fn construct_path(
    s: usize,
    t: usize,
    adj: &Vec<Vec<(usize, usize)>>,
    dist: &Vec<usize>,
) -> Vec<usize> {
    let mut path = vec![];
    path.push(t);

    let mut now = t;
    while now != s {
        // dist[now] = dist[prev] + dist(prev, now) となるような　prev を探す
        let prev = adj[now]
            .iter()
            .filter(|&&(prev, d)| dist[prev] + d == dist[now])
            .last()
            .unwrap()
            .0;
        path.push(prev);
        now = prev;
    }

    path.reverse();
    path
}
