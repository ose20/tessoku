use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }

    let tickets: Vec<_> = a.iter().map(|l|
        l.iter().rev().fold(0, |acc, &x| acc*2 + x)
    ).collect();

    let dp = calc_dp(n, &tickets);
    match dp[(1<<n) - 1] {
        None => println!("{}", -1),
        Some(ans) => println!("{}", ans),
    }

}

fn calc_dp(n: usize, tickets: &Vec<usize>) -> Vec<Option<u32>> {
    let mut dp = vec![None; 1<<n];
    dp[0] = Some(0);

    for bit in 0..(1<<n) {
        if dp[bit].is_none() { continue; }

        for ticket in tickets {
            if dp[ticket | bit].is_none() {
                dp[ticket | bit] = Some(dp[bit].unwrap() + 1);
            } else {
                dp[ticket | bit] = dp[ticket | bit].min(Some(dp[bit].unwrap() + 1));
            }
        }
    }

    dp
}

