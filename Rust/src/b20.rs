use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let dp = calc_dp(&s, &t);
    println!("{}", dp[s.len()][t.len()]);

}

fn calc_dp(s: &Vec<char>, t: &Vec<char>) -> Vec<Vec<usize>> {
    let (slen, tlen) = (s.len(), t.len());
    let mut dp = vec![vec![0; tlen+1]; slen+1];

    for i in 0..=slen {
        for j in 0..=tlen {
            if i == 0 { dp[i][j] = j }
            else if j == 0 { dp[i][j] = i }
            else {
                let ins = dp[i][j-1] + 1;
                let del = dp[i-1][j] + 1;
                let rep = dp[i-1][j-1] + if s[i-1] == t[j-1] { 0 } else { 1 };
                dp[i][j] = [ins, del, rep].into_iter().min().unwrap();
            }
        }
    }

    dp
}