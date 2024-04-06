use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xy: [(usize, usize); q],
    }

    let dp = calc_dp(n, &a);
    for xyi in xy.iter() {
        let target = walk(&dp, *xyi);
        println!("{}", target);
    }
}

// dp[i][j] := 穴　 i から出発して 2^j 日後の場所を求める
fn calc_dp(n: usize, a: &[usize]) -> Vec<Vec<usize>> {
    let mut dp = vec![vec![0; 31]; n + 1];

    // initialization
    for i in 1..=n {
        dp[i][0] = a[i - 1];
    }

    for j in 1..=30 {
        for i in 1..=n {
            dp[i][j] = dp[dp[i][j - 1]][j - 1]
        }
    }

    dp
}

fn walk(dp: &Vec<Vec<usize>>, (x, y): (usize, usize)) -> usize {
    let mut cur = x;
    for i in 0..dp[0].len() {
        // y の右から i 番目のビットが立っている時
        if (y >> i) & 1 != 0 {
            cur = dp[cur][i];
        }
    }
    cur
}
