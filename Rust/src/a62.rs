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

    let mut visited = vec![false; n];
    dfs(0, &mut visited, &adj);

    if visited.iter().all(|&b| b) {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}

fn dfs(now: usize, visited: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
    visited[now] = true;
    for &next in adj[now].iter() {
        if !visited[next] {
            dfs(next, visited, adj)
        }
    }
}
