use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    }

    // xが同じ場合、yだけ大きくても包含できないので、yの順序は逆転する必要がある
    xy.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    let dp = calc_dp(n, &xy);
    println!("{}", dp.iter().max().unwrap());
}

fn calc_dp(n: usize, xy: &Vec<(usize, usize)>) -> Vec<usize> {
    // dp[i] := i番目を最外箱としてマトリョーシカを作った時の包含数の最大値
    let mut dp = vec![0; n+1];
    // miny[v] := 包含数vを達成する組の中で、最外箱のy長のうち、最小値(dpの双対っぽいやつを管理)
    let inf = usize::MAX;
    let mut miny = vec![inf; n+1];

    // 初期化
    miny[0] = 0;

    // 遷移
    for i in 1..=n {
        let (_, y) = xy[i-1];
        let idx = miny.partition_point(|elt| *elt < y);
        dp[i] = idx;
        miny[idx] = miny[idx].min(y);
    }  

    dp
}
