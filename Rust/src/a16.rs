use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n-1],
        b: [i32; n-2],
    }

    solve(n, a, b);
}

fn solve(n: usize, a: Vec<i32>, b: Vec<i32>) {
    let max_v = 1000_000_000;
    let mut dp = vec![max_v; n+1];
    dp[1] = 0;
    for i in 2..=n {
        // a[i] := A[i+2]
        dp[i] = dp[i].min(dp[i-1] + a[i-2]);
        // b[i] := B[i+3]
        if i>2 { dp[i] = dp[i].min(dp[i-2] + b[i-3]) }
    }

    println!("{}", dp[n]);
}
