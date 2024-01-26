use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
    }

    let ans = solve(n, k);
    println!("{ans}");
}

fn solve(n: i32, k: i32) -> i32 {
    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=n {
            if 1 <= k-i-j && k-i-j <= n { ans += 1; }
        }
    }

    ans
}
