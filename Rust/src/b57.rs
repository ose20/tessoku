use std::mem::uninitialized;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let imax = 30;
    let dp = calc_dp(n, k, imax);
    let bit = bit_stat(k, imax);
    for j in 1..=n {
        let mut now = j;
        for i in 0..=imax {
            if bit[i] {
                now = dp[i][now]
            }
        }
        println!("{}", now);
    }
}

fn calc_dp(n: usize, _k: usize, imax: usize) -> Vec<Vec<usize>> {
    // dp[i][j] := j に対して、操作を 2^i 回した時の数
    // i in 0..=30, j in 0..=n
    // 遷移:
    //  - dp[0][j] := 愚直に計算
    //  - dp[i+1][j] := dp[i][dp[i][j]]

    let mut dp = vec![vec![0; n + 1]; imax + 1];

    let digit_sum = |mut num: usize| {
        let mut sum = 0;
        while num != 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    };

    // init
    for j in 0..=n {
        dp[0][j] = j - digit_sum(j);
    }

    // transition
    for i in 1..=imax {
        for j in 0..=n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    dp
}

// bit[] を返す
// bit[i] := 2^i の位の bit が立っている時、またその時のみ true
fn bit_stat(k: usize, imax: usize) -> Vec<bool> {
    let mut bit = vec![true; imax + 1];
    for i in 0..=imax {
        bit[i] = (k >> i) & 1 == 1
    }
    bit
}
