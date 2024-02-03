use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n-1],
        b: [i32; n-2],
    }

    let dp = calc_dp(n, &a, &b);
    let path = contrust_path(n, &a, &b, &dp);

    let ans: String = path.iter()
        .rev()
        .map(|pos| pos.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", path.len());
    println!("{}", ans);
}

fn calc_dp(n: usize, a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    let max_v = 1000_000_000;
    let mut dp = vec![max_v; n+1];
    dp[1] = 0;

    for i in 2..=n {
        // a[i] := A[i+2]
        // b[i] := B[i+3]
        dp[i] = dp[i].min(dp[i-1] + a[i-2]);
        if i>2 { dp[i] = dp[i].min(dp[i-2] + b[i-3]); }
    }

    dp   
}

fn contrust_path(n: usize, a: &Vec<i32>, _b: &Vec<i32>, dp: &Vec<i32>) -> Vec<usize> {
    let mut pos = n;
    let mut path = Vec::new();
    path.push(n);

    // 各ループで次の行き先をpushする
    loop {
        if pos == 1 { break; }
        if pos == 2 { path.push(1); break; }

        if dp[pos] == dp[pos-1] + a[pos-2] {
            path.push(pos-1);
            pos -= 1;
        } else {
            path.push(pos-2);
            pos -= 2;
        }
    }

    path
}
