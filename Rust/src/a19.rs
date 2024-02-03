use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let dp = calc_dp(n, w, &wv);

    //println!("{:?}", dp);

    let ans = (0..=w).into_iter()
        .map(|i| dp[n][i])
        .filter(|v| v.is_some())
        .map(|s| s.unwrap())
        .max()
        .unwrap();
        
    println!("{}", ans);
    

}

fn calc_dp(n: usize, w: usize, wv: &Vec<(usize, usize)>) -> Vec<Vec<Option<usize>>> {
    // dp[i][j] := i番目までの品物を使って重さwになるように使った時の、価値の最大値
    let mut dp = vec![vec![None; w+1]; n+1];
    dp[0][0] = Some(0);

    for i in 1..=n {
        for j in 0..=w {
            // i番目の品物を使わないとき
            dp[i][j] = dp[i][j].max(dp[i-1][j]);

            // i番目の品物を使う時
            let (wi, vi) = wv[i-1];
            if j >= wi && dp[i-1][j-wi].is_some() { 
                dp[i][j] = dp[i][j].max(Some(dp[i-1][j - wi].unwrap() + vi))
            }
        }
    }

    dp
}
