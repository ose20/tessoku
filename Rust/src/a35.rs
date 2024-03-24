use proconio::input;

fn main() {

    input! {
        n: usize,
        a: [usize; n],
    }

    let dp = calc_dp(n, &a);
    println!("{}", dp[0][0]);
}

/// dp[i][j] := (i,j)からスタートした時のスコア
/// - i = n-1 の時、 dp[i][j] = a[j]
/// - i < n-1 の時
///     - 先手のターンの場合(iが偶数)
///         - dp[i][j] = max(dp[i+1][j], dp[i+1][j+1])
///     - 後手のターンの場合(iが奇数)
///         - dp[i][j] = min(dp[i+1][j], dp[i+1][j+1])
fn calc_dp(n: usize, a: & Vec<usize>) -> Vec<Vec<usize>> {
    let mut dp = vec![vec![0; n]; n];

    for j in 0..=n-1 {
        dp[n-1][j] = a[j];
    }

    for i in (0..=n-2).rev() {
        for j in 0..=i {
            if i%2 == 0 {
                // 先手のターン
                dp[i][j] = dp[i+1][j].max(dp[i+1][j+1]);
            } else {
                // 後手のターン 
                dp[i][j] = dp[i+1][j].min(dp[i+1][j+1]);
            }
        }
    }

    dp
}