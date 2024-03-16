use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let dp = calc_dp(n, s, &a);
    if !dp[n][s] {
        println!("-1");
    } else {
        let path = construct_path(n, s, &a, &dp);
        let format = path.iter().map(usize::to_string).collect::<Vec<_>>().join(" ");
        println!("{}", path.len());
        println!("{}", format);
    }
}

fn calc_dp(n: usize, s: usize, a: & Vec<usize>) -> Vec<Vec<bool>> {
    let mut dp = vec![vec![false; s+1]; n+1];
    dp[0][0] = true;

    // もらうdp
    for i in 1..=n {
        for j in 0..=s {
            // 使わないとき
            dp[i][j] = dp[i][j] || dp[i-1][j];
            // 使うとき
            if j >= a[i-1] {
                dp[i][j] = dp[i][j] || dp[i-1][j-a[i-1]];
            }
        }
    }

    dp
}

fn construct_path(n: usize, s: usize, a: & Vec<usize>, dp: & Vec<Vec<bool>>) -> Vec<usize> {
    let mut path = Vec::new();
    let mut sum = s;

    for i in (1..=n).rev() {
        if dp[i-1][sum] {
            continue;
        } else {
            path.push(i);
            sum -= a[i-1];
        }

    }

    path.reverse();
    path
}