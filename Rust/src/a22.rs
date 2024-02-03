use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-1],
    }

    let dp = calc_dp(n, &a, &b);
    println!("{}", dp[n].unwrap());
}

fn calc_dp(n: usize, a: &Vec<usize>, b: &Vec<usize>) -> Vec<Option<i32>> {
    let mut dp = vec![None; n+1];
    dp[1] = Some(0);

    for i in 1..n {
        if dp[i].is_some() {
            dp[a[i-1]] = dp[a[i-1]].max(Some(dp[i].unwrap() + 100));
            dp[b[i-1]] = dp[b[i-1]].max(Some(dp[i].unwrap() + 150));
        }
    }

    dp
}
