use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        ab: [(usize, usize); m],
    }

    let score = calc_score(n, &ab);
    let dp = calc_dp(n, k, &score);

    println!("{}", dp[k][n].unwrap());
}

fn calc_score(n: usize, ab: &[(usize, usize)]) -> Vec<Vec<usize>> {
    // score[i][j] := iページからjページまでを同じ章に入れた時の、小説の良さの増分
    let mut score = vec![vec![0; n + 1]; n + 1];

    // O(n^2 m)
    for i in 0..=n {
        for j in 0..=n {
            score[i][j] = ab
                .iter()
                .filter(|&&(a, b)| i <= a && b <= j)
                .collect::<Vec<_>>()
                .len();
        }
    }

    score
}

fn calc_dp(n: usize, k: usize, score: &Vec<Vec<usize>>) -> Vec<Vec<Option<usize>>> {
    // dp[i][j] := i 章の終わりを j ページ目にした場合の小説の良さの最大値
    let mut dp = vec![vec![None; n + 1]; k + 1];
    dp[0][0] = Some(0);

    // O(n^2 k)
    for i in 1..=k {
        for j in 0..=n {
            dp[i][j] = (0..j)
                .filter_map(|l| dp[i - 1][l].map(|dpval| dpval + score[l + 1][j]))
                .max()
        }
    }

    dp
}
