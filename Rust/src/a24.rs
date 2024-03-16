use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let dp = calc_dp(n, &a);
    println!("{}", dp.iter().max().unwrap());
}

fn calc_dp(n: usize, a: &Vec<usize>) -> Vec<usize> {
    // dp[n] := a_nを最後尾とする増加列のうち、長さが最大のものの長さ
    // aux[n] := 長さがnになるような増加列の最後尾の値のうち、最も小さいもの(短調増加)
    let inf = 1_000_000_000;
    let mut dp = vec![0; n+1];
    let mut aux = vec![inf; n+1];
    aux[0] = 0;

    for i in 1..=n {
        let idx = aux.partition_point(|&x| x < a[i-1]);
        dp[i] = idx;
        aux[idx] = a[i-1];
    }

    dp
}
