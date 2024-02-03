use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let dp = calc_dp(&s, &t);
    println!("{}", dp[s.len()][t.len()])

}

fn calc_dp(s: &Vec<char>, t: &Vec<char>) -> Vec<Vec<usize>> {
    let mut dp = vec![vec![0; t.len()+1]; s.len()+1];

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i-1] == t[j-1] {
                dp[i][j] = *[dp[i][j], dp[i-1][j-1]+1, dp[i-1][j], dp[i][j-1]].iter().max().unwrap();
            } else {
                dp[i][j] = *[dp[i][j], dp[i-1][j], dp[i][j-1]].iter().max().unwrap();
            }
        }
    }

    dp
}

