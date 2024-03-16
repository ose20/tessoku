use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let vmax = 1000 * n;
    let dp = calc_dp(n, vmax, &wv);
    let ans = (0..=vmax).filter(|&v| dp[n][v].is_some() && dp[n][v].unwrap() <= w).max().unwrap();
    println!("{}", ans);
}

fn calc_dp(n: usize, vmax: usize, wv: & Vec<(usize, usize)>) -> Vec<Vec<Option<usize>>> {
    // dp[i][j] := i番目までのアイテムで価値jを達成するのに必要最小限の重さ
    let mut dp = vec![vec![None; vmax+1]; n+1];
    dp[0][0] = Some(0);
    for i in 1..=n {
        for j in 0..=vmax {
            let (wi, vi) = wv[i-1];
            // 使わない
            dp[i][j] = min(&dp[i][j], &dp[i-1][j]);

            // 使う
            if j >= vi && dp[i-1][j-vi].is_some() {
                dp[i][j] = min(&dp[i][j], &(dp[i-1][j - vi].map(|x| x+wi)))
            }
        }
    }

    dp
}

fn min(x: &Option<usize>, y: &Option<usize>) -> Option<usize> {
    match (x, y) {
        (None, y) => y.clone(),
        (x, None) => x.clone(),
        (Some(x), Some(y)) => Some(*x.min(y)),
    }
}
