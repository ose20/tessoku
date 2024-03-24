use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: i64,
        a: [i64; n],
        c: [i64; m],
    }

    let ans = solve(n, m, b, &a, &c);
    println!("{ans}")
}

fn solve(n: usize, m: usize, b: i64, a: & Vec<i64>, c: & Vec<i64>) -> i64 {
    let mut ans;

    ans = b * (n as i64) * (m as i64);
    ans += a.iter().map(|ai| ai * (m as i64)).sum::<i64>();
    ans += c.iter().map(|ci| ci * (n as i64)).sum::<i64>();

    ans
}
