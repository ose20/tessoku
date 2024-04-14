use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
    }

    let mut adj = vec![vec![]; n];
    for i in 0..n - 1 {
        adj[a[i] - 1].push(i + 1)
    }

    let mut dp = vec![None; n];
    let mut ans = vec![0; n];
    for i in 0..n {
        ans[i] = dfs(i, &mut dp, &adj);
    }

    println!(
        "{}",
        ans.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn dfs(now: usize, dp: &mut Vec<Option<usize>>, adj: &Vec<Vec<usize>>) -> usize {
    if let Some(val) = dp[now] {
        return val;
    }

    let mut ans = 0;
    for &child in adj[now].iter() {
        ans += 1;
        ans += dfs(child, dp, adj);
    }

    dp[now] = Some(ans);
    ans
}
