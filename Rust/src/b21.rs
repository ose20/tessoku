use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let dp = calc_dp(n, &s);
    println!("{}", dp[1][n]);

    // for left in 1..=n {
    //     for right in 1..=n {
    //         print!("{} ", dp[left][right]);
    //     }
    //     println!("");
    // }https://twitter.com/home
}

fn calc_dp(n: usize, s: &Vec<char>) -> Vec<Vec<usize>> {
    let mut dp = vec![vec![0; n+1]; n+1];

    // init
    for i in 1..=n { dp[i][i] = 1; }
    for i in 1..n {
        if s[i-1] == s[i] { dp[i][i+1] = 2; }
    }

    // construct
    // leftは右から左に、rightは左から右に
    for left in (1..=n-1).rev() {
        for right in left+1..=n {
            if s[left-1] != s[right-1] {
                dp[left][right] = dp[left+1][right].max(dp[left][right-1]);
            } else {
                dp[left][right] = dp[left+1][right].max(dp[left][right-1]).max(dp[left+1][right-1]+2);
            }
        }
    }

    dp
}
