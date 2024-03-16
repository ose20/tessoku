use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let dp = calc_dp(n, &xy);
    println!("{:.10}", dp[(1<<n)-1][0]);

}

fn calc_dp(n: usize, xy: &Vec<(i64, i64)>) -> Vec<Vec<f64>> {
    let inf = f64::MAX;
    // dp[bit][i] := bitが立っている頂点に全て訪れて、最終到達点iに行くまでの最短距離
    let mut dp = vec![vec![inf; n]; 1<<n];

    // init
    dp[0][0] = 0.0;

    let dist = |(x1, y1), (x2, y2)| {
        let dx: i64 = x2 - x1;
        let dy: i64 = y2 - y1;
        ((dx.pow(2) + dy.pow(2)) as f64).sqrt()
    };

    // 配るdp
    for bit in 0..(1<<n) {
        for now in 0..n {
            // 今いる場所が到達不可能なら遷移処理はskip
            if dp[bit][now] == inf { continue }

            for next in 0..n {
                // nextがすでに訪れた頂点ならskip
                if (bit & (1<<next)) != 0 { continue }
                let dist = dp[bit][now] + dist(xy[now], xy[next]);
                let next_bit = bit | (1 << next);
                dp[next_bit][next] = dp[next_bit][next].min(dist);
            }
        }
    }

    dp
}

