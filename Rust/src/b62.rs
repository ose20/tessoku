use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    }

    let adj = create_adj(n, &ab);
    let dist = bfs(0, &adj);
    let path = construct_path(n, &dist, &adj);

    let ans = path
        .iter()
        .map(|s| (s + 1).to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", ans);
}

fn create_adj(n: usize, ab: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];

    for &(a, b) in ab.iter() {
        adj[a - 1].push(b - 1);
        adj[b - 1].push(a - 1);
    }

    adj
}

fn bfs(s: usize, adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let inf = usize::MAX;
    let mut dist = vec![inf; adj.len()];

    let mut que = VecDeque::new();
    que.push_back(s);
    dist[s] = 0;

    while !que.is_empty() {
        let now = que.pop_front().unwrap();
        for &next in adj[now].iter() {
            if dist[next] != inf {
                continue;
            }

            dist[next] = dist[now] + 1;
            que.push_back(next);
        }
    }

    dist
}

fn construct_path(n: usize, dist: &[usize], adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut path = vec![];
    let mut now = n - 1;
    path.push(n - 1);

    while now != 0 {
        // now とつながっている prev で、　dist[prev] + 1 = dist[now] になっているようなものを探す
        let &&prev = adj[now]
            .iter()
            .filter(|&&prev| dist[prev] + 1 == dist[now])
            .collect::<Vec<_>>()
            .last()
            .unwrap();
        path.push(prev);
        now = prev;
    }

    path.reverse();
    path
}
