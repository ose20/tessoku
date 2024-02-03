use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    let dp = calc_dp(n, &h);
    println!("{}", dp[n]);
}

fn calc_dp(n: usize, h: &Vec<i32>) -> Vec<i32> {
    let max = i32::MAX;
    let mut dp = vec![max; n+1];
    dp[1] = 0;

    for i in 1..n {
        dp[i+1] = dp[i+1].min(dp[i] + (h[i] - h[i-1]).abs());

        if i <= n-2 {
            dp[i+2] = dp[i+2].min(dp[i] + (h[i+1] - h[i-1]).abs());
        }
    }
 
    dp
}
