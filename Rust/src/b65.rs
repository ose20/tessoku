use proconio::input;

fn main() {
    input! {
        n: usize, t: usize,
        ab: [(usize, usize); n-1],
    }

    let adj = make_adj(n, &ab);
    let grade = calc_grade(t - 1, &adj);
    let ans = grade
        .iter()
        .map(|g| g.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", ans);
}

fn make_adj(n: usize, ab: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];

    for &(a, b) in ab.iter() {
        adj[a - 1].push(b - 1);
        adj[b - 1].push(a - 1);
    }

    adj
}

fn calc_grade(s: usize, adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut seen = vec![false; adj.len()];
    let mut grade = vec![0; adj.len()];

    dfs(s, &mut seen, &adj, &mut grade);

    grade
}

fn dfs(now: usize, seen: &mut [bool], adj: &Vec<Vec<usize>>, grade: &mut [usize]) -> usize {
    seen[now] = true;
    let mut g = 0;
    let mut rookie = true;
    for &nxt in adj[now].iter() {
        if !seen[nxt] {
            g = g.max(dfs(nxt, seen, adj, grade));
            // 部下がいるのでフラグを折る
            rookie = false;
        }
    }
    if !rookie {
        g += 1;
    }

    grade[now] = g;
    g
}
