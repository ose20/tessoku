use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; 2*n]
    }

    let dp = calc_dp(n, &a);
    println!("{}", dp[1][2 * n]);
    //print_vec(n, &dp);
}

fn calc_dp(n: usize, a: &[i32]) -> Vec<Vec<i32>> {
    let inf = 1_000_000_000;
    let mut dp = vec![vec![inf; 2 * n + 1]; 2 * n + 1];
    // dp[i][j] := [a_i, a_j] までを全て取り除くのにかかる最小コスト（i, j は 1-index）

    // 初期化
    for i in 1..(2 * n) {
        dp[i][i + 1] = (a[i - 1] - a[i]).abs()
    }

    // 状態遷移
    // dp[i][j] は以下の候補の最小値
    // - dp[i+1][j-1] + |a[i] - a[j]|
    // - dp[i][k] + dp[k+1][j] (i+1 <= k <= j-1)
    // DAGをうまく作るために区間の長さが短い方から決めてく

    for len in 3..=(2 * n) {
        for i in 1..=(2 * n - 1) {
            let j = i + len - 1;
            if j > 2 * n {
                break;
            }

            if i + 1 < j - 1 {
                dp[i][j] = dp[i][j].min(dp[i + 1][j - 1] + (a[i - 1] - a[j - 1]).abs())
            }

            for k in (i + 1)..=(j - 1) {
                if k + 1 < j {
                    dp[i][j] = dp[i][j].min(dp[i][k] + dp[k + 1][j])
                }
            }
        }
    }

    dp
}

fn print_vec(n: usize, dp: &Vec<Vec<i32>>) {
    let inf = 1_000_000_000;
    print!("    ");
    for j in 1..=(2 * n) {
        print!(" {:>4}", j);
    }
    println!("");

    for i in 1..=(2 * n) {
        print!("{:>3}:", i);
        for j in 1..=(2 * n) {
            print!(
                " {:>4}",
                if dp[i][j] == inf {
                    "none".to_string()
                } else {
                    dp[i][j].to_string()
                }
            )
        }
        println!("");
    }
}
