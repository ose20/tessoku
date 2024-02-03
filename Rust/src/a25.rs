use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let dp = calc_dp(h, w, &c);
    println!("{}", dp[h][w]);
}


fn calc_dp(h: usize, w: usize, c: &Vec<Vec<char>>) -> Vec<Vec<u64>> {
    let mut dp = vec![vec![0; w+1]; h+1];
    dp[1][1] = 1;

    for i in 1..=h {
        for j in 1..=w {
            // 右方向への遷移
            if j < w && c[i-1][j] == '.' {
                dp[i][j+1] += dp[i][j];
            }

            // 下方向遷移
            if i < h && c[i][j-1] == '.' {
                dp[i+1][j] += dp[i][j];
            }
        }
    }

    dp
}

