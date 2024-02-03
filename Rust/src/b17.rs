use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    let dp = calc_dp(n, &h);
    let path = construct_path(n, &h, &dp);
    
    let format = path.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", path.len());
    println!("{}", format);

}

fn calc_dp(n: usize, h: &Vec<i32>) -> Vec<i32> {
    let max = i32::MAX;
    let mut dp = vec![max; n+1];

    for i in 1..n {
        dp[i+1] = dp[i+1].min(dp[i] + (h[i-1] - h[i]).abs());

        if i+2 <= n {
            dp[i+2] = dp[i+2].min(dp[i] + (h[i-1] - h[i+1]).abs());
        }
    }

    dp
}

fn construct_path(n: usize, h: &Vec<i32>, dp: &Vec<i32>) -> Vec<usize> {
    let mut path = Vec::new();
    let mut pos = n;
    path.push(n);

    loop {
        if pos == 2 {
            path.push(1);
            break;
        }
        if pos == 1 {
            break;
        }

        
        if dp[pos] == dp[pos-1] + (h[pos-1] - h[pos-2]).abs() {
            pos -= 1;
            path.push(pos);
        } else {
            pos -= 2;
            path.push(pos);
        }

    }


    path.reverse();
    path
}