use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let dp = calc_dp(n, &a);
    let ans = dp[n][0].max(dp[n][1]);
    println!("{}", ans);
}

fn calc_dp(n: usize, a: &[usize]) -> Vec<Vec<usize>> {
    // dp[i][j] := i日目に状態jだった時の実力最大値
    // j=0 => その日は勉強しない
    // j=1 => その日は勉強する
    let mut dp = vec![vec![0; 2]; n + 1];

    for i in 1..=n {
        // 勉強しない場合
        dp[i][0] = dp[i - 1][0].max(dp[i - 1][1]);
        // 勉強する場合
        dp[i][1] = dp[i - 1][0] + a[i - 1];
    }

    dp
}
