use proconio::input;

fn main() {
    input! {
        n: usize,
        pa: [(usize, i32); n],  
    }

    let dp = calc_dp(n, &pa);

    let ans = (0..=n).map(|i| dp[i][i+1]).max().unwrap();
    println!("{}", ans);
}

fn calc_dp(n: usize, pa: &Vec<(usize, i32)>) -> Vec<Vec<i32>> {
    // dp[i][j] := [1..i], [j..n] まで取り出すときの最高得点
    let mut dp = vec![vec![0; n+2]; n+2];

    for i in 0..=n {
        for j in (i+1..=n+1).rev() {
            // i番目をとってこの状態になる時
            if i > 0 {
                let (p, a) = pa[i-1];
                let rhs = dp[i-1][j] + if i < p && p < j { a } else { 0 };
                dp[i][j] = dp[i][j].max(rhs);
            }

            // j番目をとってこの状態になる時
            if j < n+1 {
                let (p, a) = pa[j-1];
                let rhs = dp[i][j+1] + if i < p && p < j { a } else { 0 };
                dp[i][j] = dp[i][j].max(rhs);
            }
        }
    }

    dp
}
