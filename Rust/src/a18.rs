use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i32,
        a: [i32; n],
    }

    let dp = calc_dp(&a, s);

    match construct_path(dp, a, n, s) {
        Some(path) => {
            println!("{}", path.len());
            let str: String = path.iter().map(|idx| idx.to_string()).collect::<Vec<_>>().join(" ");
            println!("{}", str);
        }
        None => println!("{}", -1)
    }
}

fn calc_dp(a: &Vec<i32>, s: i32) -> Vec<Vec<bool>> {
    let n = a.len();
    let mut dp = vec![vec![false; (s+1) as usize]; n+1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=(s as usize) {
            // 使わない場合
            dp[i][j] = dp[i][j] || dp[i-1][j];
            // 使う場合
            if j as i32 - a[i-1] >= 0 { 
                dp[i][j] = dp[i][j] || dp[i-1][j - (a[i-1] as usize)]; 
            }
        }
    }

    dp
}


fn construct_path(dp: Vec<Vec<bool>>, a: Vec<i32>, n: usize, s: i32) -> Option<Vec<usize>> {
    let mut path = Vec::new();

    if !dp[n][s as usize] { return None }

    let mut sum = s;
    for i in (1..=n).rev() {
        // 使う場合
        if sum - a[i-1] >= 0 && dp[i-1][(sum - a[i-1]) as usize] {
            path.push(i);
            sum -= a[i-1];
        }
    }

    path.reverse();
    Some(path)
}